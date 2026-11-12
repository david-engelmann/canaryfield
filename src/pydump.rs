//! The ast.dump spelling of PORT2-3, the change test the
//! planting stands on. The spelling names each node by its
//! CPython class, walks the fields in the order the
//! interpreter states, leaves out a field whose value is
//! the Python None, prints an empty list as an empty list,
//! and spells a constant by its repr, the interpreter's own
//! readings in each case. Positions stand outside the
//! spelling, so a change of position alone is no change,
//! the reading PORT2-3 states. A tree standing past the
//! depth ceiling of the recursive readings refuses here,
//! where the interpreter raises its recursion error.

use crate::pyarena::{field_table, refusal, run_big_stack, Data, Refusal, Shape, Tree, DEEP_LIMIT};
use crate::pyast::{
    complex_repr, float_repr, BinOpKind, BoolKind, CmpKind, ConstVal, PyText, UnaryKind,
};
use crate::pyunparse::{bytes_repr, str_repr};

/// Spell the tree the way ast.dump spells it without
/// attributes.
pub fn dump_tree(tree: &Tree) -> Result<String, Refusal> {
    if tree.depth() > DEEP_LIMIT {
        return Err(refusal("the tree stands past the depth the dump reads"));
    }
    let tree = tree.clone();
    Ok(run_big_stack(move || {
        let mut dumper = Dumper {
            tree: &tree,
            out: String::new(),
        };
        dumper.node(tree.root);
        dumper.out
    }))
}

struct Dumper<'a> {
    tree: &'a Tree,
    out: String,
}

impl<'a> Dumper<'a> {
    fn node(&mut self, id: usize) {
        let node = &self.tree.nodes[id];
        self.out.push_str(crate::pyarena::type_name(&node.data));
        self.out.push('(');
        let mut first = true;
        for field in field_table(&node.data) {
            let spelled = match field.shape {
                Shape::One => match self.tree.one(id, field.name) {
                    Some(child) => {
                        let mut sub = Dumper {
                            tree: self.tree,
                            out: String::new(),
                        };
                        sub.node(child);
                        Some(sub.out)
                    }
                    None => None,
                },
                Shape::Many => {
                    let mut parts = Vec::new();
                    for child in self.tree.many(id, field.name) {
                        let mut sub = Dumper {
                            tree: self.tree,
                            out: String::new(),
                        };
                        sub.node(*child);
                        parts.push(sub.out);
                    }
                    Some(format!("[{}]", parts.join(", ")))
                }
                Shape::Maybe => {
                    let mut parts = Vec::new();
                    for child in self.tree.maybe(id, field.name) {
                        match child {
                            Some(child) => {
                                let mut sub = Dumper {
                                    tree: self.tree,
                                    out: String::new(),
                                };
                                sub.node(*child);
                                parts.push(sub.out);
                            }
                            None => parts.push("None".to_string()),
                        }
                    }
                    Some(format!("[{}]", parts.join(", ")))
                }
                Shape::EmptyList => Some("[]".to_string()),
                Shape::Scalar => scalar_text(&node.data, field.name),
            };
            if let Some(text) = spelled {
                if !first {
                    self.out.push_str(", ");
                }
                first = false;
                self.out.push_str(field.name);
                self.out.push('=');
                self.out.push_str(&text);
            }
        }
        self.out.push(')');
    }
}

/// The spelling of one scalar field, or None when the
/// field's value is the Python None and the dump leaves
/// the field out.
fn scalar_text(data: &Data, field: &str) -> Option<String> {
    match (data, field) {
        (Data::FuncDef { name, .. }, "name") => Some(str_text(name)),
        (Data::ClassDef { name }, "name") => Some(str_text(name)),
        (Data::AugAssign { op }, "op") => Some(format!("{}()", binop_name(*op))),
        (Data::AnnAssign { simple }, "simple") => Some(if *simple { "1" } else { "0" }.to_string()),
        (Data::ImportFrom { module, .. }, "module") => module.as_ref().map(|m| str_text(m)),
        (Data::ImportFrom { level, .. }, "level") => Some(level.to_string()),
        (Data::Global { names }, "names") | (Data::Nonlocal { names }, "names") => {
            Some(str_list(names))
        }
        (Data::BoolOp { op }, "op") => Some(format!("{}()", boolop_name(*op))),
        (Data::BinOp { op }, "op") => Some(format!("{}()", binop_name(*op))),
        (Data::UnaryOp { op }, "op") => Some(format!("{}()", unary_name(*op))),
        (Data::Compare { ops }, "ops") => Some(format!(
            "[{}]",
            ops.iter()
                .map(|op| format!("{}()", cmp_name(*op)))
                .collect::<Vec<_>>()
                .join(", ")
        )),
        (Data::FormattedValue { conversion }, "conversion") => Some(conversion.to_string()),
        (Data::Constant { value }, "value") => Some(const_text(value)),
        (Data::Constant { value }, "kind") => match value {
            ConstVal::Str(_, true) => Some("'u'".to_string()),
            _ => None,
        },
        (Data::Attribute { attr, .. }, "attr") => Some(str_text(attr)),
        (Data::Attribute { ctx, .. }, "ctx")
        | (Data::Subscript { ctx }, "ctx")
        | (Data::Starred { ctx }, "ctx")
        | (Data::Name { ctx, .. }, "ctx")
        | (Data::List { ctx }, "ctx")
        | (Data::Tuple { ctx }, "ctx") => Some(format!("{}()", ctx.name())),
        (Data::Name { id, .. }, "id") => Some(str_text(id)),
        (Data::Arg { name }, "arg") => Some(str_text(name)),
        (Data::Keyword { arg }, "arg") => arg.as_ref().map(|a| str_text(a)),
        (Data::Comprehension { is_async }, "is_async") => {
            Some(if *is_async { "1" } else { "0" }.to_string())
        }
        (Data::ExceptHandler { name }, "name") => name.as_ref().map(|n| str_text(n)),
        (Data::Alias { name, .. }, "name") => Some(str_text(name)),
        (Data::Alias { asname, .. }, "asname") => asname.as_ref().map(|a| str_text(a)),
        (Data::TypeVar { name }, "name")
        | (Data::TypeVarTuple { name }, "name")
        | (Data::ParamSpec { name }, "name") => Some(str_text(name)),
        (Data::MatchSingleton { value }, "value") => Some(const_text(value)),
        (Data::MatchMapping { rest }, "rest") => rest.as_ref().map(|r| str_text(r)),
        (Data::MatchClass { kwd_attrs }, "kwd_attrs") => Some(str_list(kwd_attrs)),
        (Data::MatchAs { name }, "name") => name.as_ref().map(|n| str_text(n)),
        (Data::MatchStar { name }, "name") => name.as_ref().map(|n| str_text(n)),
        _ => None,
    }
}

/// The repr of a plain string, the spelling the dump gives
/// a name or a string constant alike.
fn str_text(text: &str) -> String {
    str_repr(PyText::from_text(text).codepoints())
}

/// A list of strings spelled as the dump spells a list of
/// reprs.
fn str_list(names: &[String]) -> String {
    format!(
        "[{}]",
        names
            .iter()
            .map(|name| str_text(name))
            .collect::<Vec<_>>()
            .join(", ")
    )
}

/// The repr of a constant value, the spelling the dump
/// gives the value field of a constant.
pub fn const_text(value: &ConstVal) -> String {
    match value {
        ConstVal::NoneVal => "None".to_string(),
        ConstVal::Bool(flag) => {
            if *flag {
                "True".to_string()
            } else {
                "False".to_string()
            }
        }
        ConstVal::Int(text) => text.clone(),
        ConstVal::Float(value) => float_repr(*value),
        ConstVal::Complex(real, imag) => complex_repr(*real, *imag),
        ConstVal::Str(text, _) => str_repr(text.codepoints()),
        ConstVal::Bytes(bytes) => bytes_repr(bytes),
        ConstVal::Ellipsis => "Ellipsis".to_string(),
    }
}

fn binop_name(op: BinOpKind) -> &'static str {
    match op {
        BinOpKind::Add => "Add",
        BinOpKind::Sub => "Sub",
        BinOpKind::Mult => "Mult",
        BinOpKind::MatMult => "MatMult",
        BinOpKind::Div => "Div",
        BinOpKind::Mod => "Mod",
        BinOpKind::Pow => "Pow",
        BinOpKind::LShift => "LShift",
        BinOpKind::RShift => "RShift",
        BinOpKind::BitOr => "BitOr",
        BinOpKind::BitXor => "BitXor",
        BinOpKind::BitAnd => "BitAnd",
        BinOpKind::FloorDiv => "FloorDiv",
    }
}

fn unary_name(op: UnaryKind) -> &'static str {
    match op {
        UnaryKind::Invert => "Invert",
        UnaryKind::Not => "Not",
        UnaryKind::UAdd => "UAdd",
        UnaryKind::USub => "USub",
    }
}

fn boolop_name(op: BoolKind) -> &'static str {
    match op {
        BoolKind::And => "And",
        BoolKind::Or => "Or",
    }
}

fn cmp_name(op: CmpKind) -> &'static str {
    match op {
        CmpKind::Eq => "Eq",
        CmpKind::NotEq => "NotEq",
        CmpKind::Lt => "Lt",
        CmpKind::LtE => "LtE",
        CmpKind::Gt => "Gt",
        CmpKind::GtE => "GtE",
        CmpKind::Is => "Is",
        CmpKind::IsNot => "IsNot",
        CmpKind::In => "In",
        CmpKind::NotIn => "NotIn",
    }
}
