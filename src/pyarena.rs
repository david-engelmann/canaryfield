//! The mutable arena form of the pyast tree, the shape the
//! seed applies plant into under PORT2-1. The parser builds
//! the boxed tree of pyast, and this module lowers it into a
//! flat arena where every node stands at an index that plays
//! the part the Python object identity plays in the landed
//! readings, so a collector can record a node and its parent
//! slot and the planting can replace the child in that slot
//! after the collection is whole. The arena carries the
//! context field CPython assigns (Load, Store, Del) as a
//! reading over the lowered tree, and it raises the arena
//! back into the boxed tree for the unparse spelling. Nodes
//! the applies create carry no position, as a node the Python
//! builds by hand carries no line number until
//! fix_missing_locations runs, and no recorded surface reads
//! a position the change test or the rendering would spell.

use crate::pyast::*;
use std::collections::VecDeque;

/// One refusal of the port, in the port's own wording under
/// PORT2-7. A parse refusal and a depth ceiling refusal of
/// the recursion family both travel as this shape.
#[derive(Debug, Clone)]
pub struct Refusal {
    pub message: String,
}

pub fn refusal(message: &str) -> Refusal {
    Refusal {
        message: message.to_string(),
    }
}

/// The depth ceiling of the recursive readings, the dump
/// among them, measured in arena nodes on the longest path
/// from the module. The interpreter's recursive readings of
/// a chain stand to a depth of 992 here and refuse past it,
/// the band the round probes measured against CPython.
pub const DEEP_LIMIT: usize = 992;

/// The depth ceiling of the recursive collectors of the
/// seed layer, measured the same way. The interpreter's
/// collectors stand to a depth of 994 here.
pub const COLLECT_LIMIT: usize = 994;

/// The depth ceiling of the unparse rendering, measured the
/// same way. The interpreter's unparser stands to a depth
/// of 330 here, the shallowest of the recursive readings.
pub const RENDER_LIMIT: usize = 330;

/// Run work on a spawned thread carrying a stack of 512 MiB,
/// the pattern the parser and the unparser state, so a tree
/// nested to the port bound is read whole on a caller's
/// plain thread instead of overflowing its stack. A panic
/// on the spawned thread resumes on the caller's thread.
pub fn run_big_stack<T: Send>(work: impl FnOnce() -> T + Send) -> T {
    let outcome = std::thread::scope(|scope| {
        let handle = std::thread::Builder::new()
            .stack_size(512 * 1024 * 1024)
            .spawn_scoped(scope, work);
        match handle {
            Ok(joined) => joined.join(),
            Err(_) => Err(Box::new(()) as Box<dyn std::any::Any + Send>),
        }
    });
    match outcome {
        Ok(value) => value,
        Err(payload) => std::panic::resume_unwind(payload),
    }
}

/// The expression context CPython assigns, the ctx field of
/// the context bearing expressions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ctx {
    Load,
    Store,
    Del,
}

impl Ctx {
    pub fn name(&self) -> &'static str {
        match self {
            Ctx::Load => "Load",
            Ctx::Store => "Store",
            Ctx::Del => "Del",
        }
    }
}

/// One child bearing field of a node, in the field order
/// CPython states. A One field holds a single child or
/// nothing, a Many field holds a list of children, and a
/// Maybe field holds a list whose entries may be holes, the
/// dict keys and the keyword defaults where Python itself
/// stands in the list.
#[derive(Debug, Clone)]
pub enum Slot {
    One(Option<usize>),
    Many(Vec<usize>),
    Maybe(Vec<Option<usize>>),
}

/// The scalar payload of one arena node, everything about
/// the node that is not a child. The variant names the
/// CPython class the node stands for, with the async and
/// star forms carried as flags the class name reading
/// resolves.
#[derive(Debug, Clone)]
pub enum Data {
    Module,
    FuncDef { name: String, is_async: bool },
    ClassDef { name: String },
    Return,
    Delete,
    Assign,
    AugAssign { op: BinOpKind },
    AnnAssign { simple: bool },
    For { is_async: bool },
    While,
    If,
    With { is_async: bool },
    Match,
    Raise,
    Try { star: bool },
    Assert,
    Import,
    ImportFrom { module: Option<String>, level: u32 },
    Global { names: Vec<String> },
    Nonlocal { names: Vec<String> },
    ExprStmt,
    Pass,
    Break,
    Continue,
    TypeAlias,
    BoolOp { op: BoolKind },
    NamedExpr,
    BinOp { op: BinOpKind },
    UnaryOp { op: UnaryKind },
    Lambda,
    IfExp,
    Dict,
    Set,
    ListComp,
    SetComp,
    DictComp,
    GeneratorExp,
    Await,
    Yield,
    YieldFrom,
    Compare { ops: Vec<CmpKind> },
    Call,
    FormattedValue { conversion: i32 },
    JoinedStr,
    Constant { value: ConstVal },
    Attribute { attr: String, ctx: Ctx },
    Subscript { ctx: Ctx },
    Starred { ctx: Ctx },
    Name { id: String, ctx: Ctx },
    List { ctx: Ctx },
    Tuple { ctx: Ctx },
    Slice,
    Arguments,
    Arg { name: String },
    Keyword { arg: Option<String> },
    Comprehension { is_async: bool },
    ExceptHandler { name: Option<String> },
    MatchCase,
    WithItem,
    Alias { name: String, asname: Option<String> },
    TypeVar { name: String },
    TypeVarTuple { name: String },
    ParamSpec { name: String },
    MatchValue,
    MatchSingleton { value: ConstVal },
    MatchSequence,
    MatchMapping { rest: Option<String> },
    MatchClass { kwd_attrs: Vec<String> },
    MatchAs { name: Option<String> },
    MatchStar { name: Option<String> },
    MatchOr,
}

/// The CPython class name of a node, the spelling the dump
/// prints and the type name the position order breaks a tie
/// with.
pub fn type_name(data: &Data) -> &'static str {
    match data {
        Data::Module => "Module",
        Data::FuncDef { is_async, .. } => {
            if *is_async {
                "AsyncFunctionDef"
            } else {
                "FunctionDef"
            }
        }
        Data::ClassDef { .. } => "ClassDef",
        Data::Return => "Return",
        Data::Delete => "Delete",
        Data::Assign => "Assign",
        Data::AugAssign { .. } => "AugAssign",
        Data::AnnAssign { .. } => "AnnAssign",
        Data::For { is_async, .. } => {
            if *is_async {
                "AsyncFor"
            } else {
                "For"
            }
        }
        Data::While => "While",
        Data::If => "If",
        Data::With { is_async, .. } => {
            if *is_async {
                "AsyncWith"
            } else {
                "With"
            }
        }
        Data::Match => "Match",
        Data::Raise => "Raise",
        Data::Try { star } => {
            if *star {
                "TryStar"
            } else {
                "Try"
            }
        }
        Data::Assert => "Assert",
        Data::Import => "Import",
        Data::ImportFrom { .. } => "ImportFrom",
        Data::Global { .. } => "Global",
        Data::Nonlocal { .. } => "Nonlocal",
        Data::ExprStmt => "Expr",
        Data::Pass => "Pass",
        Data::Break => "Break",
        Data::Continue => "Continue",
        Data::TypeAlias => "TypeAlias",
        Data::BoolOp { .. } => "BoolOp",
        Data::NamedExpr => "NamedExpr",
        Data::BinOp { .. } => "BinOp",
        Data::UnaryOp { .. } => "UnaryOp",
        Data::Lambda => "Lambda",
        Data::IfExp => "IfExp",
        Data::Dict => "Dict",
        Data::Set => "Set",
        Data::ListComp => "ListComp",
        Data::SetComp => "SetComp",
        Data::DictComp => "DictComp",
        Data::GeneratorExp => "GeneratorExp",
        Data::Await => "Await",
        Data::Yield => "Yield",
        Data::YieldFrom => "YieldFrom",
        Data::Compare { .. } => "Compare",
        Data::Call => "Call",
        Data::FormattedValue { .. } => "FormattedValue",
        Data::JoinedStr => "JoinedStr",
        Data::Constant { .. } => "Constant",
        Data::Attribute { .. } => "Attribute",
        Data::Subscript { .. } => "Subscript",
        Data::Starred { .. } => "Starred",
        Data::Name { .. } => "Name",
        Data::List { .. } => "List",
        Data::Tuple { .. } => "Tuple",
        Data::Slice => "Slice",
        Data::Arguments => "arguments",
        Data::Arg { .. } => "arg",
        Data::Keyword { .. } => "keyword",
        Data::Comprehension { .. } => "comprehension",
        Data::ExceptHandler { .. } => "ExceptHandler",
        Data::MatchCase => "match_case",
        Data::WithItem => "withitem",
        Data::Alias { .. } => "alias",
        Data::TypeVar { .. } => "TypeVar",
        Data::TypeVarTuple { .. } => "TypeVarTuple",
        Data::ParamSpec { .. } => "ParamSpec",
        Data::MatchValue => "MatchValue",
        Data::MatchSingleton { .. } => "MatchSingleton",
        Data::MatchSequence => "MatchSequence",
        Data::MatchMapping { .. } => "MatchMapping",
        Data::MatchClass { .. } => "MatchClass",
        Data::MatchAs { .. } => "MatchAs",
        Data::MatchStar { .. } => "MatchStar",
        Data::MatchOr => "MatchOr",
    }
}

/// The shape of one field in a field table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    One,
    Many,
    Maybe,
    /// A scalar field, carried in the node data, printed by
    /// the dump and never walked.
    Scalar,
    /// A list field the parser never fills, printed empty by
    /// the dump, the module's type ignores.
    EmptyList,
}

/// One field of a node kind, in the order CPython states
/// for the kind.
#[derive(Debug, Clone, Copy)]
pub struct Field {
    pub name: &'static str,
    pub shape: Shape,
}

const ONE: Shape = Shape::One;
const MANY: Shape = Shape::Many;
const MAYBE: Shape = Shape::Maybe;
const SCALAR: Shape = Shape::Scalar;

fn fields(names: &[(&'static str, Shape)]) -> Vec<Field> {
    names
        .iter()
        .map(|(name, shape)| Field {
            name,
            shape: *shape,
        })
        .collect()
}

/// The field table of a node kind, in the field order
/// CPython states, scalar fields standing where the
/// interpreter states them among the child fields.
pub fn field_table(data: &Data) -> Vec<Field> {
    match data {
        Data::Module => fields(&[("body", MANY), ("type_ignores", Shape::EmptyList)]),
        Data::FuncDef { .. } => fields(&[
            ("name", SCALAR),
            ("args", ONE),
            ("body", MANY),
            ("decorator_list", MANY),
            ("returns", ONE),
            ("type_params", MANY),
        ]),
        Data::ClassDef { .. } => fields(&[
            ("name", SCALAR),
            ("bases", MANY),
            ("keywords", MANY),
            ("body", MANY),
            ("decorator_list", MANY),
            ("type_params", MANY),
        ]),
        Data::Return => fields(&[("value", ONE)]),
        Data::Delete => fields(&[("targets", MANY)]),
        Data::Assign => fields(&[("targets", MANY), ("value", ONE)]),
        Data::AugAssign { .. } => fields(&[("target", ONE), ("op", SCALAR), ("value", ONE)]),
        Data::AnnAssign { .. } => fields(&[
            ("target", ONE),
            ("annotation", ONE),
            ("value", ONE),
            ("simple", SCALAR),
        ]),
        Data::For { .. } => fields(&[
            ("target", ONE),
            ("iter", ONE),
            ("body", MANY),
            ("orelse", MANY),
        ]),
        Data::While => fields(&[("test", ONE), ("body", MANY), ("orelse", MANY)]),
        Data::If => fields(&[("test", ONE), ("body", MANY), ("orelse", MANY)]),
        Data::With { .. } => fields(&[("items", MANY), ("body", MANY)]),
        Data::Match => fields(&[("subject", ONE), ("cases", MANY)]),
        Data::Raise => fields(&[("exc", ONE), ("cause", ONE)]),
        Data::Try { .. } => fields(&[
            ("body", MANY),
            ("handlers", MANY),
            ("orelse", MANY),
            ("finalbody", MANY),
        ]),
        Data::Assert => fields(&[("test", ONE), ("msg", ONE)]),
        Data::Import => fields(&[("names", MANY)]),
        Data::ImportFrom { .. } => fields(&[
            ("module", SCALAR),
            ("names", MANY),
            ("level", SCALAR),
        ]),
        Data::Global { .. } | Data::Nonlocal { .. } => fields(&[("names", SCALAR)]),
        Data::ExprStmt => fields(&[("value", ONE)]),
        Data::Pass | Data::Break | Data::Continue => fields(&[]),
        Data::TypeAlias => fields(&[("name", ONE), ("type_params", MANY), ("value", ONE)]),
        Data::BoolOp { .. } => fields(&[("op", SCALAR), ("values", MANY)]),
        Data::NamedExpr => fields(&[("target", ONE), ("value", ONE)]),
        Data::BinOp { .. } => fields(&[("left", ONE), ("op", SCALAR), ("right", ONE)]),
        Data::UnaryOp { .. } => fields(&[("op", SCALAR), ("operand", ONE)]),
        Data::Lambda => fields(&[("args", ONE), ("body", ONE)]),
        Data::IfExp => fields(&[("test", ONE), ("body", ONE), ("orelse", ONE)]),
        Data::Dict => fields(&[("keys", MAYBE), ("values", MANY)]),
        Data::Set => fields(&[("elts", MANY)]),
        Data::ListComp | Data::SetComp | Data::GeneratorExp => {
            fields(&[("elt", ONE), ("generators", MANY)])
        }
        Data::DictComp => fields(&[("key", ONE), ("value", ONE), ("generators", MANY)]),
        Data::Await | Data::YieldFrom => fields(&[("value", ONE)]),
        Data::Yield => fields(&[("value", ONE)]),
        Data::Compare { .. } => fields(&[
            ("left", ONE),
            ("ops", SCALAR),
            ("comparators", MANY),
        ]),
        Data::Call => fields(&[("func", ONE), ("args", MANY), ("keywords", MANY)]),
        Data::FormattedValue { .. } => fields(&[
            ("value", ONE),
            ("conversion", SCALAR),
            ("format_spec", ONE),
        ]),
        Data::JoinedStr => fields(&[("values", MANY)]),
        Data::Constant { .. } => fields(&[("value", SCALAR), ("kind", SCALAR)]),
        Data::Attribute { .. } => fields(&[("value", ONE), ("attr", SCALAR), ("ctx", SCALAR)]),
        Data::Subscript { .. } => fields(&[("value", ONE), ("slice", ONE), ("ctx", SCALAR)]),
        Data::Starred { .. } => fields(&[("value", ONE), ("ctx", SCALAR)]),
        Data::Name { .. } => fields(&[("id", SCALAR), ("ctx", SCALAR)]),
        Data::List { .. } | Data::Tuple { .. } => fields(&[("elts", MANY), ("ctx", SCALAR)]),
        Data::Slice => fields(&[("lower", ONE), ("upper", ONE), ("step", ONE)]),
        Data::Arguments => fields(&[
            ("posonlyargs", MANY),
            ("args", MANY),
            ("vararg", ONE),
            ("kwonlyargs", MANY),
            ("kw_defaults", MAYBE),
            ("kwarg", ONE),
            ("defaults", MANY),
        ]),
        Data::Arg { .. } => fields(&[("arg", SCALAR), ("annotation", ONE)]),
        Data::Keyword { .. } => fields(&[("arg", SCALAR), ("value", ONE)]),
        Data::Comprehension { .. } => fields(&[
            ("target", ONE),
            ("iter", ONE),
            ("ifs", MANY),
            ("is_async", SCALAR),
        ]),
        Data::ExceptHandler { .. } => fields(&[("type", ONE), ("name", SCALAR), ("body", MANY)]),
        Data::MatchCase => fields(&[("pattern", ONE), ("guard", ONE), ("body", MANY)]),
        Data::WithItem => fields(&[("context_expr", ONE), ("optional_vars", ONE)]),
        Data::Alias { .. } => fields(&[("name", SCALAR), ("asname", SCALAR)]),
        Data::TypeVar { .. } => fields(&[("name", SCALAR), ("bound", ONE)]),
        Data::TypeVarTuple { .. } | Data::ParamSpec { .. } => fields(&[("name", SCALAR)]),
        Data::MatchValue => fields(&[("value", ONE)]),
        Data::MatchSingleton { .. } => fields(&[("value", SCALAR)]),
        Data::MatchSequence => fields(&[("patterns", MANY)]),
        Data::MatchMapping { .. } => fields(&[
            ("keys", MANY),
            ("patterns", MANY),
            ("rest", SCALAR),
        ]),
        Data::MatchClass { .. } => fields(&[
            ("cls", ONE),
            ("patterns", MANY),
            ("kwd_attrs", SCALAR),
            ("kwd_patterns", MANY),
        ]),
        Data::MatchAs { .. } => fields(&[("pattern", ONE), ("name", SCALAR)]),
        Data::MatchStar { .. } => fields(&[("name", SCALAR)]),
        Data::MatchOr => fields(&[("patterns", MANY)]),
    }
}

/// One arena node, its scalar payload, its child slots in
/// field order, and its position when the kind carries one.
#[derive(Debug, Clone)]
pub struct Node {
    pub data: Data,
    pub slots: Vec<Slot>,
    pub pos: Option<Pos>,
}

/// One lowered module, the arena the applies run over. The
/// root is the module node's index.
#[derive(Debug, Clone)]
pub struct Tree {
    pub nodes: Vec<Node>,
    pub root: usize,
}

/// The slot standing at a field of a node kind, an index
/// into the node's slots counting only the child bearing
/// fields.
fn slot_index(data: &Data, field: &str) -> Option<usize> {
    let mut index = 0usize;
    for entry in field_table(data) {
        match entry.shape {
            Shape::One | Shape::Many | Shape::Maybe => {
                if entry.name == field {
                    return Some(index);
                }
                index += 1;
            }
            Shape::Scalar | Shape::EmptyList => {}
        }
    }
    None
}

impl Tree {
    pub fn new() -> Tree {
        Tree {
            nodes: Vec::new(),
            root: 0,
        }
    }

    /// Add a node and return its index, the identity the
    /// readings key on.
    pub fn push(&mut self, data: Data, slots: Vec<Slot>, pos: Option<Pos>) -> usize {
        let index = self.nodes.len();
        self.nodes.push(Node { data, slots, pos });
        index
    }

    pub fn node(&self, id: usize) -> &Node {
        &self.nodes[id]
    }

    pub fn node_mut(&mut self, id: usize) -> &mut Node {
        &mut self.nodes[id]
    }

    pub fn data(&self, id: usize) -> &Data {
        &self.nodes[id].data
    }

    /// The child of a One field, when one stands.
    pub fn one(&self, id: usize, field: &str) -> Option<usize> {
        let node = &self.nodes[id];
        let index = slot_index(&node.data, field)?;
        match &node.slots[index] {
            Slot::One(child) => *child,
            _ => None,
        }
    }

    /// The children of a Many field.
    pub fn many(&self, id: usize, field: &str) -> &[usize] {
        let node = &self.nodes[id];
        match slot_index(&node.data, field) {
            Some(index) => match &node.slots[index] {
                Slot::Many(children) => children,
                _ => &[],
            },
            None => &[],
        }
    }

    /// The children of a Maybe field, holes standing.
    pub fn maybe(&self, id: usize, field: &str) -> &[Option<usize>] {
        let node = &self.nodes[id];
        match slot_index(&node.data, field) {
            Some(index) => match &node.slots[index] {
                Slot::Maybe(children) => children,
                _ => &[],
            },
            None => &[],
        }
    }

    pub fn set_one(&mut self, id: usize, field: &str, child: Option<usize>) {
        let node = &mut self.nodes[id];
        let index = slot_index(&node.data, field).expect("the field stands");
        node.slots[index] = Slot::One(child);
    }

    pub fn set_many(&mut self, id: usize, field: &str, children: Vec<usize>) {
        let node = &mut self.nodes[id];
        let index = slot_index(&node.data, field).expect("the field stands");
        node.slots[index] = Slot::Many(children);
    }

    pub fn set_maybe(&mut self, id: usize, field: &str, children: Vec<Option<usize>>) {
        let node = &mut self.nodes[id];
        let index = slot_index(&node.data, field).expect("the field stands");
        node.slots[index] = Slot::Maybe(children);
    }

    /// A mutable handle to one child slot, the surgery the
    /// parent carrying collectors record. The index is the
    /// child's place in a list field, or None for a single
    /// child field.
    pub fn slot_mut(&mut self, id: usize, field: &str, index: Option<usize>) -> &mut usize {
        let node = &mut self.nodes[id];
        let slot = slot_index(&node.data, field).expect("the field stands");
        match (&mut node.slots[slot], index) {
            (Slot::One(child), None) => child.as_mut().expect("a child stands"),
            (Slot::Many(children), Some(at)) => &mut children[at],
            (Slot::Maybe(children), Some(at)) => children[at].as_mut().expect("a child stands"),
            _ => panic!("the slot shape does not match the surgery"),
        }
    }

    /// The children of a node, each with its field name, its
    /// index in a list field or None for a single child
    /// field, and its index in the arena, in the field order
    /// iter_child_nodes yields.
    pub fn children(&self, id: usize) -> Vec<(&'static str, Option<usize>, usize)> {
        let node = &self.nodes[id];
        let mut out = Vec::new();
        let mut slot = 0usize;
        for entry in field_table(&node.data) {
            match entry.shape {
                Shape::One | Shape::Many | Shape::Maybe => {
                    match &node.slots[slot] {
                        Slot::One(child) => {
                            if let Some(child) = child {
                                out.push((entry.name, None, *child));
                            }
                        }
                        Slot::Many(children) => {
                            for (at, child) in children.iter().enumerate() {
                                out.push((entry.name, Some(at), *child));
                            }
                        }
                        Slot::Maybe(children) => {
                            for (at, child) in children.iter().enumerate() {
                                if let Some(child) = child {
                                    out.push((entry.name, Some(at), *child));
                                }
                            }
                        }
                    }
                    slot += 1;
                }
                Shape::Scalar | Shape::EmptyList => {}
            }
        }
        out
    }

    /// The child ids of a node alone, in field order.
    pub fn child_ids(&self, id: usize) -> Vec<usize> {
        self.children(id).into_iter().map(|(_, _, child)| child).collect()
    }

    /// Every node reachable from the root in the breadth
    /// first order ast.walk yields, the module first.
    pub fn walk(&self) -> Vec<usize> {
        let mut out = Vec::new();
        let mut queue: VecDeque<usize> = VecDeque::new();
        queue.push_back(self.root);
        while let Some(id) = queue.pop_front() {
            out.push(id);
            for child in self.child_ids(id) {
                queue.push_back(child);
            }
        }
        out
    }

    /// The parent of each reachable node, keyed by node
    /// index, over the same enumeration the walk makes.
    pub fn parent_map(&self) -> std::collections::HashMap<usize, usize> {
        let mut parents = std::collections::HashMap::new();
        for id in self.walk() {
            for child in self.child_ids(id) {
                parents.insert(child, id);
            }
        }
        parents
    }

    /// The nodes carrying a position, sorted the way
    /// _sorted_nodes sorts them in the landed Python, by
    /// source position with the node type name breaking a
    /// tie at one position, the sort stable over the walk.
    pub fn sorted_nodes(&self) -> Vec<usize> {
        let mut nodes: Vec<usize> = self
            .walk()
            .into_iter()
            .filter(|id| self.nodes[*id].pos.is_some())
            .collect();
        nodes.sort_by(|a, b| {
            let pa = self.nodes[*a].pos.expect("a position stands");
            let pb = self.nodes[*b].pos.expect("a position stands");
            (pa.line, pa.col, type_name(&self.nodes[*a].data)).cmp(&(
                pb.line,
                pb.col,
                type_name(&self.nodes[*b].data),
            ))
        });
        nodes
    }

    /// The depth of the tree, the number of nodes on the
    /// longest path from the module down, measured without
    /// recursion so the reading itself never joins the
    /// recursion family.
    pub fn depth(&self) -> usize {
        let mut deepest = 0usize;
        let mut stack: Vec<(usize, usize)> = vec![(self.root, 1)];
        while let Some((id, depth)) = stack.pop() {
            if depth > deepest {
                deepest = depth;
            }
            for child in self.child_ids(id) {
                stack.push((child, depth + 1));
            }
        }
        deepest
    }
}

/// Parse a whole source into the arena, the library's tree
/// reading of PORT2-2. A source the grammar refuses comes
/// back as a refusal in the port's own wording.
pub fn parse_tree(source: &str) -> Result<Tree, Refusal> {
    let module = crate::pyparse::parse_module(source)
        .map_err(|error| refusal(&format!("the source does not parse ({})", error.message)))?;
    Ok(run_big_stack(move || lower_module(&module)))
}

/// Render the tree the way ast.unparse renders it, under
/// PORT2-3. A tree standing past the rendering ceiling
/// comes back as a refusal of the recursion family, where
/// the interpreter raises its recursion error.
pub fn render_tree(tree: &Tree) -> Result<String, Refusal> {
    if tree.depth() > RENDER_LIMIT {
        return Err(refusal(
            "the tree stands past the depth the rendering reads",
        ));
    }
    let module = run_big_stack(|| raise_module(tree));
    Ok(crate::pyunparse::unparse_module(&module))
}

fn one(child: Option<usize>) -> Slot {
    Slot::One(child)
}

fn many(children: Vec<usize>) -> Slot {
    Slot::Many(children)
}

fn maybe(children: Vec<Option<usize>>) -> Slot {
    Slot::Maybe(children)
}

/// Lower the boxed tree into the arena, then assign the
/// contexts the interpreter assigns.
pub fn lower_module(module: &Module) -> Tree {
    let mut tree = Tree::new();
    let root = {
        let mut lowerer = Lowerer { tree: &mut tree };
        let body = lowerer.stmts(&module.body);
        lowerer.tree.push(Data::Module, vec![many(body)], None)
    };
    tree.root = root;
    assign_contexts(&mut tree);
    tree
}

struct Lowerer<'a> {
    tree: &'a mut Tree,
}

impl<'a> Lowerer<'a> {
    fn stmts(&mut self, body: &[Stmt]) -> Vec<usize> {
        body.iter().map(|stmt| self.stmt(stmt)).collect()
    }

    fn exprs(&mut self, exprs: &[Expr]) -> Vec<usize> {
        exprs.iter().map(|expr| self.expr(expr)).collect()
    }

    fn stmt(&mut self, stmt: &Stmt) -> usize {
        let pos = Some(stmt.pos);
        match &stmt.kind {
            StmtKind::FuncDef {
                name,
                args,
                body,
                decorators,
                returns,
                type_params,
                is_async,
            } => {
                let args = self.arguments(args);
                let body = self.stmts(body);
                let decorators = self.exprs(decorators);
                let returns = returns.as_ref().map(|expr| self.expr(expr));
                let type_params = self.type_params(type_params);
                self.tree.push(
                    Data::FuncDef {
                        name: name.clone(),
                        is_async: *is_async,
                    },
                    vec![
                        one(Some(args)),
                        many(body),
                        many(decorators),
                        one(returns),
                        many(type_params),
                    ],
                    pos,
                )
            }
            StmtKind::ClassDef {
                name,
                bases,
                keywords,
                body,
                decorators,
                type_params,
            } => {
                let bases = self.exprs(bases);
                let keywords = keywords.iter().map(|kw| self.keyword(kw)).collect();
                let body = self.stmts(body);
                let decorators = self.exprs(decorators);
                let type_params = self.type_params(type_params);
                self.tree.push(
                    Data::ClassDef { name: name.clone() },
                    vec![
                        many(bases),
                        many(keywords),
                        many(body),
                        many(decorators),
                        many(type_params),
                    ],
                    pos,
                )
            }
            StmtKind::Return { value } => {
                let value = value.as_ref().map(|expr| self.expr(expr));
                self.tree.push(Data::Return, vec![one(value)], pos)
            }
            StmtKind::Delete { targets } => {
                let targets = self.exprs(targets);
                self.tree.push(Data::Delete, vec![many(targets)], pos)
            }
            StmtKind::Assign { targets, value } => {
                let targets = self.exprs(targets);
                let value = self.expr(value);
                self.tree
                    .push(Data::Assign, vec![many(targets), one(Some(value))], pos)
            }
            StmtKind::AugAssign { target, op, value } => {
                let target = self.expr(target);
                let value = self.expr(value);
                self.tree.push(
                    Data::AugAssign { op: *op },
                    vec![one(Some(target)), one(Some(value))],
                    pos,
                )
            }
            StmtKind::AnnAssign {
                target,
                annotation,
                value,
                simple,
            } => {
                let target = self.expr(target);
                let annotation = self.expr(annotation);
                let value = value.as_ref().map(|expr| self.expr(expr));
                self.tree.push(
                    Data::AnnAssign { simple: *simple },
                    vec![one(Some(target)), one(Some(annotation)), one(value)],
                    pos,
                )
            }
            StmtKind::For {
                target,
                iter,
                body,
                orelse,
                is_async,
            } => {
                let target = self.expr(target);
                let iter = self.expr(iter);
                let body = self.stmts(body);
                let orelse = self.stmts(orelse);
                self.tree.push(
                    Data::For { is_async: *is_async },
                    vec![
                        one(Some(target)),
                        one(Some(iter)),
                        many(body),
                        many(orelse),
                    ],
                    pos,
                )
            }
            StmtKind::While { test, body, orelse } => {
                let test = self.expr(test);
                let body = self.stmts(body);
                let orelse = self.stmts(orelse);
                self.tree.push(
                    Data::While,
                    vec![one(Some(test)), many(body), many(orelse)],
                    pos,
                )
            }
            StmtKind::If { test, body, orelse } => {
                let test = self.expr(test);
                let body = self.stmts(body);
                let orelse = self.stmts(orelse);
                self.tree.push(
                    Data::If,
                    vec![one(Some(test)), many(body), many(orelse)],
                    pos,
                )
            }
            StmtKind::With {
                items,
                body,
                is_async,
            } => {
                let items = items.iter().map(|item| self.with_item(item)).collect();
                let body = self.stmts(body);
                self.tree.push(
                    Data::With { is_async: *is_async },
                    vec![many(items), many(body)],
                    pos,
                )
            }
            StmtKind::Match { subject, cases } => {
                let subject = self.expr(subject);
                let cases = cases.iter().map(|case| self.case(case)).collect();
                self.tree
                    .push(Data::Match, vec![one(Some(subject)), many(cases)], pos)
            }
            StmtKind::Raise { exc, cause } => {
                let exc = exc.as_ref().map(|expr| self.expr(expr));
                let cause = cause.as_ref().map(|expr| self.expr(expr));
                self.tree.push(Data::Raise, vec![one(exc), one(cause)], pos)
            }
            StmtKind::Try {
                body,
                handlers,
                orelse,
                finalbody,
                star,
            } => {
                let body = self.stmts(body);
                let handlers = handlers.iter().map(|h| self.handler(h)).collect();
                let orelse = self.stmts(orelse);
                let finalbody = self.stmts(finalbody);
                self.tree.push(
                    Data::Try { star: *star },
                    vec![many(body), many(handlers), many(orelse), many(finalbody)],
                    pos,
                )
            }
            StmtKind::Assert { test, msg } => {
                let test = self.expr(test);
                let msg = msg.as_ref().map(|expr| self.expr(expr));
                self.tree
                    .push(Data::Assert, vec![one(Some(test)), one(msg)], pos)
            }
            StmtKind::Import { names } => {
                let names = names.iter().map(|alias| self.alias(alias)).collect();
                self.tree.push(Data::Import, vec![many(names)], pos)
            }
            StmtKind::ImportFrom {
                module,
                names,
                level,
            } => {
                let names = names.iter().map(|alias| self.alias(alias)).collect();
                self.tree.push(
                    Data::ImportFrom {
                        module: module.clone(),
                        level: *level,
                    },
                    vec![many(names)],
                    pos,
                )
            }
            StmtKind::Global { names } => self.tree.push(
                Data::Global { names: names.clone() },
                vec![],
                pos,
            ),
            StmtKind::Nonlocal { names } => self.tree.push(
                Data::Nonlocal {
                    names: names.clone(),
                },
                vec![],
                pos,
            ),
            StmtKind::ExprStmt { value } => {
                let value = self.expr(value);
                self.tree
                    .push(Data::ExprStmt, vec![one(Some(value))], pos)
            }
            StmtKind::Pass => self.tree.push(Data::Pass, vec![], pos),
            StmtKind::Break => self.tree.push(Data::Break, vec![], pos),
            StmtKind::Continue => self.tree.push(Data::Continue, vec![], pos),
            StmtKind::TypeAlias {
                name,
                type_params,
                value,
            } => {
                let name = self.expr(name);
                let type_params = self.type_params(type_params);
                let value = self.expr(value);
                self.tree.push(
                    Data::TypeAlias,
                    vec![one(Some(name)), many(type_params), one(Some(value))],
                    pos,
                )
            }
        }
    }

    fn expr(&mut self, expr: &Expr) -> usize {
        let pos = Some(expr.pos);
        match &expr.kind {
            ExprKind::Constant(value) => self.tree.push(
                Data::Constant {
                    value: value.clone(),
                },
                vec![],
                pos,
            ),
            ExprKind::Name(name) => self.tree.push(
                Data::Name {
                    id: name.clone(),
                    ctx: Ctx::Load,
                },
                vec![],
                pos,
            ),
            ExprKind::Attribute { value, attr } => {
                let value = self.expr(value);
                self.tree.push(
                    Data::Attribute {
                        attr: attr.clone(),
                        ctx: Ctx::Load,
                    },
                    vec![one(Some(value))],
                    pos,
                )
            }
            ExprKind::Call {
                func,
                args,
                keywords,
            } => {
                let func = self.expr(func);
                let args = self.exprs(args);
                let keywords = keywords.iter().map(|kw| self.keyword(kw)).collect();
                self.tree.push(
                    Data::Call,
                    vec![one(Some(func)), many(args), many(keywords)],
                    pos,
                )
            }
            ExprKind::BinOp { left, op, right } => {
                let left = self.expr(left);
                let right = self.expr(right);
                self.tree.push(
                    Data::BinOp { op: *op },
                    vec![one(Some(left)), one(Some(right))],
                    pos,
                )
            }
            ExprKind::UnaryOp { op, operand } => {
                let operand = self.expr(operand);
                self.tree.push(
                    Data::UnaryOp { op: *op },
                    vec![one(Some(operand))],
                    pos,
                )
            }
            ExprKind::BoolOp { op, values } => {
                let values = self.exprs(values);
                self.tree
                    .push(Data::BoolOp { op: *op }, vec![many(values)], pos)
            }
            ExprKind::Compare {
                left,
                ops,
                comparators,
            } => {
                let left = self.expr(left);
                let comparators = self.exprs(comparators);
                self.tree.push(
                    Data::Compare { ops: ops.clone() },
                    vec![one(Some(left)), many(comparators)],
                    pos,
                )
            }
            ExprKind::IfExp { test, body, orelse } => {
                let test = self.expr(test);
                let body = self.expr(body);
                let orelse = self.expr(orelse);
                self.tree.push(
                    Data::IfExp,
                    vec![one(Some(test)), one(Some(body)), one(Some(orelse))],
                    pos,
                )
            }
            ExprKind::Subscript { value, slice } => {
                let value = self.expr(value);
                let slice = self.expr(slice);
                self.tree.push(
                    Data::Subscript { ctx: Ctx::Load },
                    vec![one(Some(value)), one(Some(slice))],
                    pos,
                )
            }
            ExprKind::Slice { lower, upper, step } => {
                let lower = lower.as_ref().map(|expr| self.expr(expr));
                let upper = upper.as_ref().map(|expr| self.expr(expr));
                let step = step.as_ref().map(|expr| self.expr(expr));
                self.tree.push(
                    Data::Slice,
                    vec![one(lower), one(upper), one(step)],
                    pos,
                )
            }
            ExprKind::List { elts } => {
                let elts = self.exprs(elts);
                self.tree
                    .push(Data::List { ctx: Ctx::Load }, vec![many(elts)], pos)
            }
            ExprKind::Tuple { elts } => {
                let elts = self.exprs(elts);
                self.tree
                    .push(Data::Tuple { ctx: Ctx::Load }, vec![many(elts)], pos)
            }
            ExprKind::Set { elts } => {
                let elts = self.exprs(elts);
                self.tree.push(Data::Set, vec![many(elts)], pos)
            }
            ExprKind::Dict { keys, values } => {
                let keys = keys
                    .iter()
                    .map(|key| key.as_ref().map(|expr| self.expr(expr)))
                    .collect();
                let values = self.exprs(values);
                self.tree
                    .push(Data::Dict, vec![maybe(keys), many(values)], pos)
            }
            ExprKind::Starred { value } => {
                let value = self.expr(value);
                self.tree.push(
                    Data::Starred { ctx: Ctx::Load },
                    vec![one(Some(value))],
                    pos,
                )
            }
            ExprKind::Await { value } => {
                let value = self.expr(value);
                self.tree.push(Data::Await, vec![one(Some(value))], pos)
            }
            ExprKind::Yield { value } => {
                let value = value.as_ref().map(|expr| self.expr(expr));
                self.tree.push(Data::Yield, vec![one(value)], pos)
            }
            ExprKind::YieldFrom { value } => {
                let value = self.expr(value);
                self.tree.push(Data::YieldFrom, vec![one(Some(value))], pos)
            }
            ExprKind::Lambda { args, body } => {
                let args = self.arguments(args);
                let body = self.expr(body);
                self.tree
                    .push(Data::Lambda, vec![one(Some(args)), one(Some(body))], pos)
            }
            ExprKind::ListComp { elt, generators } => {
                let elt = self.expr(elt);
                let generators = self.comprehensions(generators);
                self.tree.push(
                    Data::ListComp,
                    vec![one(Some(elt)), many(generators)],
                    pos,
                )
            }
            ExprKind::SetComp { elt, generators } => {
                let elt = self.expr(elt);
                let generators = self.comprehensions(generators);
                self.tree.push(
                    Data::SetComp,
                    vec![one(Some(elt)), many(generators)],
                    pos,
                )
            }
            ExprKind::GeneratorExp { elt, generators } => {
                let elt = self.expr(elt);
                let generators = self.comprehensions(generators);
                self.tree.push(
                    Data::GeneratorExp,
                    vec![one(Some(elt)), many(generators)],
                    pos,
                )
            }
            ExprKind::DictComp {
                key,
                value,
                generators,
            } => {
                let key = self.expr(key);
                let value = self.expr(value);
                let generators = self.comprehensions(generators);
                self.tree.push(
                    Data::DictComp,
                    vec![one(Some(key)), one(Some(value)), many(generators)],
                    pos,
                )
            }
            ExprKind::JoinedStr { values } => {
                let values = self.exprs(values);
                self.tree.push(Data::JoinedStr, vec![many(values)], pos)
            }
            ExprKind::FormattedValue {
                value,
                conversion,
                format_spec,
            } => {
                let value = self.expr(value);
                let format_spec = format_spec.as_ref().map(|expr| self.expr(expr));
                self.tree.push(
                    Data::FormattedValue {
                        conversion: *conversion,
                    },
                    vec![one(Some(value)), one(format_spec)],
                    pos,
                )
            }
            ExprKind::NamedExpr { target, value } => {
                let target = self.expr(target);
                let value = self.expr(value);
                self.tree.push(
                    Data::NamedExpr,
                    vec![one(Some(target)), one(Some(value))],
                    pos,
                )
            }
        }
    }

    fn arguments(&mut self, args: &Arguments) -> usize {
        let posonlyargs = args.posonlyargs.iter().map(|arg| self.arg(arg)).collect();
        let plain = args.args.iter().map(|arg| self.arg(arg)).collect();
        let vararg = args.vararg.as_ref().map(|arg| self.arg(arg));
        let kwonlyargs = args.kwonlyargs.iter().map(|arg| self.arg(arg)).collect();
        let kw_defaults = args
            .kw_defaults
            .iter()
            .map(|expr| expr.as_ref().map(|expr| self.expr(expr)))
            .collect();
        let kwarg = args.kwarg.as_ref().map(|arg| self.arg(arg));
        let defaults = self.exprs(&args.defaults);
        self.tree.push(
            Data::Arguments,
            vec![
                many(posonlyargs),
                many(plain),
                one(vararg),
                many(kwonlyargs),
                maybe(kw_defaults),
                one(kwarg),
                many(defaults),
            ],
            None,
        )
    }

    fn arg(&mut self, arg: &Arg) -> usize {
        let annotation = arg.annotation.as_ref().map(|expr| self.expr(expr));
        self.tree.push(
            Data::Arg {
                name: arg.name.clone(),
            },
            vec![one(annotation)],
            Some(arg.pos),
        )
    }

    fn keyword(&mut self, keyword: &Keyword) -> usize {
        let value = self.expr(&keyword.value);
        self.tree.push(
            Data::Keyword {
                arg: keyword.arg.clone(),
            },
            vec![one(Some(value))],
            Some(keyword.pos),
        )
    }

    fn comprehensions(&mut self, comps: &[Comprehension]) -> Vec<usize> {
        comps
            .iter()
            .map(|comp| {
                let target = self.expr(&comp.target);
                let iter = self.expr(&comp.iter);
                let ifs = self.exprs(&comp.ifs);
                self.tree.push(
                    Data::Comprehension {
                        is_async: comp.is_async,
                    },
                    vec![one(Some(target)), one(Some(iter)), many(ifs)],
                    None,
                )
            })
            .collect()
    }

    fn handler(&mut self, handler: &ExceptHandler) -> usize {
        let type_ = handler.type_.as_ref().map(|expr| self.expr(expr));
        let body = self.stmts(&handler.body);
        self.tree.push(
            Data::ExceptHandler {
                name: handler.name.clone(),
            },
            vec![one(type_), many(body)],
            Some(handler.pos),
        )
    }

    fn case(&mut self, case: &MatchCase) -> usize {
        let pattern = self.pattern(&case.pattern);
        let guard = case.guard.as_ref().map(|expr| self.expr(expr));
        let body = self.stmts(&case.body);
        self.tree.push(
            Data::MatchCase,
            vec![one(Some(pattern)), one(guard), many(body)],
            None,
        )
    }

    fn pattern(&mut self, pattern: &Pattern) -> usize {
        let pos = Some(pattern.pos);
        match &pattern.kind {
            PatternKind::MatchValue(expr) => {
                let value = self.expr(expr);
                self.tree
                    .push(Data::MatchValue, vec![one(Some(value))], pos)
            }
            PatternKind::MatchSingleton(value) => self.tree.push(
                Data::MatchSingleton {
                    value: value.clone(),
                },
                vec![],
                pos,
            ),
            PatternKind::MatchSequence(patterns) => {
                let patterns = patterns.iter().map(|p| self.pattern(p)).collect();
                self.tree
                    .push(Data::MatchSequence, vec![many(patterns)], pos)
            }
            PatternKind::MatchStar(name) => self.tree.push(
                Data::MatchStar { name: name.clone() },
                vec![],
                pos,
            ),
            PatternKind::MatchMapping {
                keys,
                patterns,
                rest,
            } => {
                let keys = self.exprs(keys);
                let patterns = patterns.iter().map(|p| self.pattern(p)).collect();
                self.tree.push(
                    Data::MatchMapping { rest: rest.clone() },
                    vec![many(keys), many(patterns)],
                    pos,
                )
            }
            PatternKind::MatchClass {
                cls,
                patterns,
                kwd_attrs,
                kwd_patterns,
            } => {
                let cls = self.expr(cls);
                let patterns = patterns.iter().map(|p| self.pattern(p)).collect();
                let kwd_patterns = kwd_patterns.iter().map(|p| self.pattern(p)).collect();
                self.tree.push(
                    Data::MatchClass {
                        kwd_attrs: kwd_attrs.clone(),
                    },
                    vec![one(Some(cls)), many(patterns), many(kwd_patterns)],
                    pos,
                )
            }
            PatternKind::MatchAs { pattern, name } => {
                let pattern = pattern.as_ref().map(|p| self.pattern(p));
                self.tree.push(
                    Data::MatchAs { name: name.clone() },
                    vec![one(pattern)],
                    pos,
                )
            }
            PatternKind::MatchOr(patterns) => {
                let patterns = patterns.iter().map(|p| self.pattern(p)).collect();
                self.tree.push(Data::MatchOr, vec![many(patterns)], pos)
            }
        }
    }

    fn with_item(&mut self, item: &WithItem) -> usize {
        let context = self.expr(&item.context_expr);
        let vars = item.optional_vars.as_ref().map(|expr| self.expr(expr));
        self.tree.push(
            Data::WithItem,
            vec![one(Some(context)), one(vars)],
            None,
        )
    }

    fn alias(&mut self, alias: &Alias) -> usize {
        self.tree.push(
            Data::Alias {
                name: alias.name.clone(),
                asname: alias.asname.clone(),
            },
            vec![],
            Some(alias.pos),
        )
    }

    fn type_params(&mut self, params: &[TypeParam]) -> Vec<usize> {
        params
            .iter()
            .map(|param| {
                let pos = Some(param.pos);
                match &param.kind {
                    TypeParamKind::TypeVar { name, bound } => {
                        let bound = bound.as_ref().map(|expr| self.expr(expr));
                        self.tree.push(
                            Data::TypeVar { name: name.clone() },
                            vec![one(bound)],
                            pos,
                        )
                    }
                    TypeParamKind::TypeVarTuple { name } => self.tree.push(
                        Data::TypeVarTuple { name: name.clone() },
                        vec![],
                        pos,
                    ),
                    TypeParamKind::ParamSpec { name } => self.tree.push(
                        Data::ParamSpec { name: name.clone() },
                        vec![],
                        pos,
                    ),
                }
            })
            .collect()
    }
}

/// Assign the context of every context bearing node the way
/// the interpreter's parser assigns it. Every such node
/// starts at Load in the lowering, and the target positions
/// move to Store or Del under the rules the parser states
/// for assignments, deletions, loop and comprehension
/// targets, with items, walrus targets, and alias names.
fn assign_contexts(tree: &mut Tree) {
    let ids: Vec<usize> = (0..tree.nodes.len()).collect();
    for id in ids {
        match tree.data(id).clone() {
            Data::Assign => {
                for target in tree.many(id, "targets").to_vec() {
                    set_context(tree, target, Ctx::Store);
                }
            }
            Data::AugAssign { .. } | Data::AnnAssign { .. } => {
                if let Some(target) = tree.one(id, "target") {
                    set_context(tree, target, Ctx::Store);
                }
            }
            Data::Delete => {
                for target in tree.many(id, "targets").to_vec() {
                    set_context(tree, target, Ctx::Del);
                }
            }
            Data::For { .. } => {
                if let Some(target) = tree.one(id, "target") {
                    set_context(tree, target, Ctx::Store);
                }
            }
            Data::TypeAlias => {
                if let Some(name) = tree.one(id, "name") {
                    set_context(tree, name, Ctx::Store);
                }
            }
            Data::NamedExpr => {
                if let Some(target) = tree.one(id, "target") {
                    set_context(tree, target, Ctx::Store);
                }
            }
            Data::Comprehension { .. } => {
                if let Some(target) = tree.one(id, "target") {
                    set_context(tree, target, Ctx::Store);
                }
            }
            Data::WithItem => {
                if let Some(vars) = tree.one(id, "optional_vars") {
                    set_context(tree, vars, Ctx::Store);
                }
            }
            _ => {}
        }
    }
}

/// Set the context of one target expression, following the
/// interpreter's rule. A name takes the context itself, an
/// attribute or a subscript takes it at its own node alone,
/// and a starred value or a list or tuple of targets
/// carries it into its elements.
fn set_context(tree: &mut Tree, id: usize, ctx: Ctx) {
    match &mut tree.node_mut(id).data {
        Data::Name { ctx: own, .. } => *own = ctx,
        Data::Attribute { ctx: own, .. } => *own = ctx,
        Data::Subscript { ctx: own } => *own = ctx,
        Data::Starred { ctx: own } => {
            *own = ctx;
            if let Some(value) = tree.one(id, "value") {
                set_context(tree, value, ctx);
            }
        }
        Data::List { ctx: own } | Data::Tuple { ctx: own } => {
            *own = ctx;
            for elt in tree.many(id, "elts").to_vec() {
                set_context(tree, elt, ctx);
            }
        }
        _ => {}
    }
}

/// Raise the arena back into the boxed tree of pyast for
/// the unparse spelling. Node identities are assigned fresh
/// in the raising, and a node carrying no position raises
/// with a zero position, which no spelling reads.
pub fn raise_module(tree: &Tree) -> Module {
    let mut raiser = Raiser { tree, next: 1 };
    let body = raiser.stmts(tree.many(tree.root, "body"));
    Module { id: 0, body }
}

struct Raiser<'a> {
    tree: &'a Tree,
    next: usize,
}

impl<'a> Raiser<'a> {
    fn id(&mut self) -> usize {
        let id = self.next;
        self.next += 1;
        id
    }

    fn pos(&self, id: usize) -> Pos {
        self.tree.nodes[id].pos.unwrap_or(Pos { line: 0, col: 0 })
    }

    fn stmts(&mut self, ids: &[usize]) -> Vec<Stmt> {
        ids.iter().map(|id| self.stmt(*id)).collect()
    }

    fn exprs(&mut self, ids: &[usize]) -> Vec<Expr> {
        ids.iter().map(|id| self.expr(*id)).collect()
    }

    fn stmt(&mut self, id: usize) -> Stmt {
        let pos = self.pos(id);
        let kind = match &self.tree.nodes[id].data {
            Data::FuncDef { name, is_async } => {
                let args = self.arguments(self.tree.one(id, "args").expect("args stand"));
                let body = self.stmts(self.tree.many(id, "body"));
                let decorators = self.exprs(self.tree.many(id, "decorator_list"));
                let returns = self
                    .tree
                    .one(id, "returns")
                    .map(|expr| Box::new(self.expr(expr)));
                let type_params = self.type_params(self.tree.many(id, "type_params"));
                StmtKind::FuncDef {
                    name: name.clone(),
                    args: Box::new(args),
                    body,
                    decorators,
                    returns,
                    type_params,
                    is_async: *is_async,
                }
            }
            Data::ClassDef { name } => {
                let bases = self.exprs(self.tree.many(id, "bases"));
                let keywords = self
                    .tree
                    .many(id, "keywords")
                    .iter()
                    .map(|kw| self.keyword(*kw))
                    .collect();
                let body = self.stmts(self.tree.many(id, "body"));
                let decorators = self.exprs(self.tree.many(id, "decorator_list"));
                let type_params = self.type_params(self.tree.many(id, "type_params"));
                StmtKind::ClassDef {
                    name: name.clone(),
                    bases,
                    keywords,
                    body,
                    decorators,
                    type_params,
                }
            }
            Data::Return => StmtKind::Return {
                value: self.tree.one(id, "value").map(|expr| self.expr(expr)),
            },
            Data::Delete => StmtKind::Delete {
                targets: self.exprs(self.tree.many(id, "targets")),
            },
            Data::Assign => StmtKind::Assign {
                targets: self.exprs(self.tree.many(id, "targets")),
                value: self.expr(self.tree.one(id, "value").expect("a value stands")),
            },
            Data::AugAssign { op } => StmtKind::AugAssign {
                target: self.expr(self.tree.one(id, "target").expect("a target stands")),
                op: *op,
                value: self.expr(self.tree.one(id, "value").expect("a value stands")),
            },
            Data::AnnAssign { simple } => StmtKind::AnnAssign {
                target: self.expr(self.tree.one(id, "target").expect("a target stands")),
                annotation: self.expr(
                    self.tree
                        .one(id, "annotation")
                        .expect("an annotation stands"),
                ),
                value: self.tree.one(id, "value").map(|expr| self.expr(expr)),
                simple: *simple,
            },
            Data::For { is_async } => StmtKind::For {
                target: self.expr(self.tree.one(id, "target").expect("a target stands")),
                iter: self.expr(self.tree.one(id, "iter").expect("an iter stands")),
                body: self.stmts(self.tree.many(id, "body")),
                orelse: self.stmts(self.tree.many(id, "orelse")),
                is_async: *is_async,
            },
            Data::While => StmtKind::While {
                test: self.expr(self.tree.one(id, "test").expect("a test stands")),
                body: self.stmts(self.tree.many(id, "body")),
                orelse: self.stmts(self.tree.many(id, "orelse")),
            },
            Data::If => StmtKind::If {
                test: self.expr(self.tree.one(id, "test").expect("a test stands")),
                body: self.stmts(self.tree.many(id, "body")),
                orelse: self.stmts(self.tree.many(id, "orelse")),
            },
            Data::With { is_async } => StmtKind::With {
                items: self
                    .tree
                    .many(id, "items")
                    .iter()
                    .map(|item| self.with_item(*item))
                    .collect(),
                body: self.stmts(self.tree.many(id, "body")),
                is_async: *is_async,
            },
            Data::Match => StmtKind::Match {
                subject: self.expr(self.tree.one(id, "subject").expect("a subject stands")),
                cases: self
                    .tree
                    .many(id, "cases")
                    .iter()
                    .map(|case| self.case(*case))
                    .collect(),
            },
            Data::Raise => StmtKind::Raise {
                exc: self.tree.one(id, "exc").map(|expr| self.expr(expr)),
                cause: self.tree.one(id, "cause").map(|expr| self.expr(expr)),
            },
            Data::Try { star } => StmtKind::Try {
                body: self.stmts(self.tree.many(id, "body")),
                handlers: self
                    .tree
                    .many(id, "handlers")
                    .iter()
                    .map(|handler| self.handler(*handler))
                    .collect(),
                orelse: self.stmts(self.tree.many(id, "orelse")),
                finalbody: self.stmts(self.tree.many(id, "finalbody")),
                star: *star,
            },
            Data::Assert => StmtKind::Assert {
                test: self.expr(self.tree.one(id, "test").expect("a test stands")),
                msg: self.tree.one(id, "msg").map(|expr| self.expr(expr)),
            },
            Data::Import => StmtKind::Import {
                names: self
                    .tree
                    .many(id, "names")
                    .iter()
                    .map(|alias| self.alias(*alias))
                    .collect(),
            },
            Data::ImportFrom { module, level } => StmtKind::ImportFrom {
                module: module.clone(),
                names: self
                    .tree
                    .many(id, "names")
                    .iter()
                    .map(|alias| self.alias(*alias))
                    .collect(),
                level: *level,
            },
            Data::Global { names } => StmtKind::Global {
                names: names.clone(),
            },
            Data::Nonlocal { names } => StmtKind::Nonlocal {
                names: names.clone(),
            },
            Data::ExprStmt => StmtKind::ExprStmt {
                value: self.expr(self.tree.one(id, "value").expect("a value stands")),
            },
            Data::Pass => StmtKind::Pass,
            Data::Break => StmtKind::Break,
            Data::Continue => StmtKind::Continue,
            Data::TypeAlias => StmtKind::TypeAlias {
                name: self.expr(self.tree.one(id, "name").expect("a name stands")),
                type_params: self.type_params(self.tree.many(id, "type_params")),
                value: self.expr(self.tree.one(id, "value").expect("a value stands")),
            },
            _ => panic!("a statement kind stands at a statement place"),
        };
        Stmt {
            id: self.id(),
            pos,
            kind,
        }
    }

    fn expr(&mut self, id: usize) -> Expr {
        let pos = self.pos(id);
        let kind = match &self.tree.nodes[id].data {
            Data::Constant { value } => ExprKind::Constant(value.clone()),
            Data::Name { id: name, .. } => ExprKind::Name(name.clone()),
            Data::Attribute { attr, .. } => ExprKind::Attribute {
                value: Box::new(self.expr(self.tree.one(id, "value").expect("a value stands"))),
                attr: attr.clone(),
            },
            Data::Call => ExprKind::Call {
                func: Box::new(self.expr(self.tree.one(id, "func").expect("a func stands"))),
                args: self.exprs(self.tree.many(id, "args")),
                keywords: self
                    .tree
                    .many(id, "keywords")
                    .iter()
                    .map(|kw| self.keyword(*kw))
                    .collect(),
            },
            Data::BinOp { op } => ExprKind::BinOp {
                left: Box::new(self.expr(self.tree.one(id, "left").expect("a left stands"))),
                op: *op,
                right: Box::new(self.expr(self.tree.one(id, "right").expect("a right stands"))),
            },
            Data::UnaryOp { op } => ExprKind::UnaryOp {
                op: *op,
                operand: Box::new(
                    self.expr(self.tree.one(id, "operand").expect("an operand stands")),
                ),
            },
            Data::BoolOp { op } => ExprKind::BoolOp {
                op: *op,
                values: self.exprs(self.tree.many(id, "values")),
            },
            Data::Compare { ops } => ExprKind::Compare {
                left: Box::new(self.expr(self.tree.one(id, "left").expect("a left stands"))),
                ops: ops.clone(),
                comparators: self.exprs(self.tree.many(id, "comparators")),
            },
            Data::IfExp => ExprKind::IfExp {
                test: Box::new(self.expr(self.tree.one(id, "test").expect("a test stands"))),
                body: Box::new(self.expr(self.tree.one(id, "body").expect("a body stands"))),
                orelse: Box::new(
                    self.expr(self.tree.one(id, "orelse").expect("an orelse stands")),
                ),
            },
            Data::Subscript { .. } => ExprKind::Subscript {
                value: Box::new(self.expr(self.tree.one(id, "value").expect("a value stands"))),
                slice: Box::new(self.expr(self.tree.one(id, "slice").expect("a slice stands"))),
            },
            Data::Slice => ExprKind::Slice {
                lower: self
                    .tree
                    .one(id, "lower")
                    .map(|expr| Box::new(self.expr(expr))),
                upper: self
                    .tree
                    .one(id, "upper")
                    .map(|expr| Box::new(self.expr(expr))),
                step: self
                    .tree
                    .one(id, "step")
                    .map(|expr| Box::new(self.expr(expr))),
            },
            Data::List { .. } => ExprKind::List {
                elts: self.exprs(self.tree.many(id, "elts")),
            },
            Data::Tuple { .. } => ExprKind::Tuple {
                elts: self.exprs(self.tree.many(id, "elts")),
            },
            Data::Set => ExprKind::Set {
                elts: self.exprs(self.tree.many(id, "elts")),
            },
            Data::Dict => ExprKind::Dict {
                keys: self
                    .tree
                    .maybe(id, "keys")
                    .iter()
                    .map(|key| key.map(|expr| self.expr(expr)))
                    .collect(),
                values: self.exprs(self.tree.many(id, "values")),
            },
            Data::Starred { .. } => ExprKind::Starred {
                value: Box::new(self.expr(self.tree.one(id, "value").expect("a value stands"))),
            },
            Data::Await => ExprKind::Await {
                value: Box::new(self.expr(self.tree.one(id, "value").expect("a value stands"))),
            },
            Data::Yield => ExprKind::Yield {
                value: self
                    .tree
                    .one(id, "value")
                    .map(|expr| Box::new(self.expr(expr))),
            },
            Data::YieldFrom => ExprKind::YieldFrom {
                value: Box::new(self.expr(self.tree.one(id, "value").expect("a value stands"))),
            },
            Data::Lambda => ExprKind::Lambda {
                args: Box::new(self.arguments(self.tree.one(id, "args").expect("args stand"))),
                body: Box::new(self.expr(self.tree.one(id, "body").expect("a body stands"))),
            },
            Data::ListComp => ExprKind::ListComp {
                elt: Box::new(self.expr(self.tree.one(id, "elt").expect("an elt stands"))),
                generators: self.comprehensions(self.tree.many(id, "generators")),
            },
            Data::SetComp => ExprKind::SetComp {
                elt: Box::new(self.expr(self.tree.one(id, "elt").expect("an elt stands"))),
                generators: self.comprehensions(self.tree.many(id, "generators")),
            },
            Data::GeneratorExp => ExprKind::GeneratorExp {
                elt: Box::new(self.expr(self.tree.one(id, "elt").expect("an elt stands"))),
                generators: self.comprehensions(self.tree.many(id, "generators")),
            },
            Data::DictComp => ExprKind::DictComp {
                key: Box::new(self.expr(self.tree.one(id, "key").expect("a key stands"))),
                value: Box::new(self.expr(self.tree.one(id, "value").expect("a value stands"))),
                generators: self.comprehensions(self.tree.many(id, "generators")),
            },
            Data::JoinedStr => ExprKind::JoinedStr {
                values: self.exprs(self.tree.many(id, "values")),
            },
            Data::FormattedValue { conversion } => ExprKind::FormattedValue {
                value: Box::new(self.expr(self.tree.one(id, "value").expect("a value stands"))),
                conversion: *conversion,
                format_spec: self
                    .tree
                    .one(id, "format_spec")
                    .map(|expr| Box::new(self.expr(expr))),
            },
            Data::NamedExpr => ExprKind::NamedExpr {
                target: Box::new(
                    self.expr(self.tree.one(id, "target").expect("a target stands")),
                ),
                value: Box::new(self.expr(self.tree.one(id, "value").expect("a value stands"))),
            },
            _ => panic!("an expression kind stands at an expression place"),
        };
        Expr {
            id: self.id(),
            pos,
            kind,
        }
    }

    fn arguments(&mut self, id: usize) -> Arguments {
        Arguments {
            id: self.id(),
            posonlyargs: self
                .tree
                .many(id, "posonlyargs")
                .iter()
                .map(|arg| self.arg(*arg))
                .collect(),
            args: self
                .tree
                .many(id, "args")
                .iter()
                .map(|arg| self.arg(*arg))
                .collect(),
            vararg: self.tree.one(id, "vararg").map(|arg| self.arg(arg)),
            kwonlyargs: self
                .tree
                .many(id, "kwonlyargs")
                .iter()
                .map(|arg| self.arg(*arg))
                .collect(),
            kw_defaults: self
                .tree
                .maybe(id, "kw_defaults")
                .iter()
                .map(|expr| expr.map(|expr| self.expr(expr)))
                .collect(),
            kwarg: self.tree.one(id, "kwarg").map(|arg| self.arg(arg)),
            defaults: self.exprs(self.tree.many(id, "defaults")),
        }
    }

    fn arg(&mut self, id: usize) -> Arg {
        let (name, pos) = match &self.tree.nodes[id].data {
            Data::Arg { name } => (name.clone(), self.pos(id)),
            _ => panic!("an arg kind stands at an arg place"),
        };
        Arg {
            id: self.id(),
            pos,
            name,
            annotation: self
                .tree
                .one(id, "annotation")
                .map(|expr| Box::new(self.expr(expr))),
        }
    }

    fn keyword(&mut self, id: usize) -> Keyword {
        let (arg, pos) = match &self.tree.nodes[id].data {
            Data::Keyword { arg } => (arg.clone(), self.pos(id)),
            _ => panic!("a keyword kind stands at a keyword place"),
        };
        Keyword {
            id: self.id(),
            pos,
            arg,
            value: self.expr(self.tree.one(id, "value").expect("a value stands")),
        }
    }

    fn comprehensions(&mut self, ids: &[usize]) -> Vec<Comprehension> {
        ids.iter()
            .map(|id| {
                let is_async = matches!(
                    &self.tree.nodes[*id].data,
                    Data::Comprehension { is_async } if *is_async
                );
                Comprehension {
                    id: self.id(),
                    pos: self.pos(*id),
                    target: self.expr(self.tree.one(*id, "target").expect("a target stands")),
                    iter: self.expr(self.tree.one(*id, "iter").expect("an iter stands")),
                    ifs: self.exprs(self.tree.many(*id, "ifs")),
                    is_async,
                }
            })
            .collect()
    }

    fn handler(&mut self, id: usize) -> ExceptHandler {
        let name = match &self.tree.nodes[id].data {
            Data::ExceptHandler { name } => name.clone(),
            _ => panic!("a handler kind stands at a handler place"),
        };
        ExceptHandler {
            id: self.id(),
            pos: self.pos(id),
            type_: self.tree.one(id, "type").map(|expr| self.expr(expr)),
            name,
            body: self.stmts(self.tree.many(id, "body")),
        }
    }

    fn case(&mut self, id: usize) -> MatchCase {
        MatchCase {
            id: self.id(),
            pos: self.pos(id),
            pattern: self.pattern(self.tree.one(id, "pattern").expect("a pattern stands")),
            guard: self.tree.one(id, "guard").map(|expr| self.expr(expr)),
            body: self.stmts(self.tree.many(id, "body")),
        }
    }

    fn pattern(&mut self, id: usize) -> Pattern {
        let pos = self.pos(id);
        let kind = match &self.tree.nodes[id].data {
            Data::MatchValue => PatternKind::MatchValue(
                self.expr(self.tree.one(id, "value").expect("a value stands")),
            ),
            Data::MatchSingleton { value } => PatternKind::MatchSingleton(value.clone()),
            Data::MatchSequence => PatternKind::MatchSequence(
                self.tree
                    .many(id, "patterns")
                    .iter()
                    .map(|pattern| self.pattern(*pattern))
                    .collect(),
            ),
            Data::MatchStar { name } => PatternKind::MatchStar(name.clone()),
            Data::MatchMapping { rest } => PatternKind::MatchMapping {
                keys: self.exprs(self.tree.many(id, "keys")),
                patterns: self
                    .tree
                    .many(id, "patterns")
                    .iter()
                    .map(|pattern| self.pattern(*pattern))
                    .collect(),
                rest: rest.clone(),
            },
            Data::MatchClass { kwd_attrs } => PatternKind::MatchClass {
                cls: Box::new(self.expr(self.tree.one(id, "cls").expect("a class stands"))),
                patterns: self
                    .tree
                    .many(id, "patterns")
                    .iter()
                    .map(|pattern| self.pattern(*pattern))
                    .collect(),
                kwd_attrs: kwd_attrs.clone(),
                kwd_patterns: self
                    .tree
                    .many(id, "kwd_patterns")
                    .iter()
                    .map(|pattern| self.pattern(*pattern))
                    .collect(),
            },
            Data::MatchAs { name } => PatternKind::MatchAs {
                pattern: self
                    .tree
                    .one(id, "pattern")
                    .map(|pattern| Box::new(self.pattern(pattern))),
                name: name.clone(),
            },
            Data::MatchOr => PatternKind::MatchOr(
                self.tree
                    .many(id, "patterns")
                    .iter()
                    .map(|pattern| self.pattern(*pattern))
                    .collect(),
            ),
            _ => panic!("a pattern kind stands at a pattern place"),
        };
        Pattern {
            id: self.id(),
            pos,
            kind,
        }
    }

    fn with_item(&mut self, id: usize) -> WithItem {
        WithItem {
            id: self.id(),
            pos: self.pos(id),
            context_expr: self.expr(
                self.tree
                    .one(id, "context_expr")
                    .expect("a context expression stands"),
            ),
            optional_vars: self
                .tree
                .one(id, "optional_vars")
                .map(|expr| self.expr(expr)),
        }
    }

    fn alias(&mut self, id: usize) -> Alias {
        match &self.tree.nodes[id].data {
            Data::Alias { name, asname } => Alias {
                id: self.id(),
                pos: self.pos(id),
                name: name.clone(),
                asname: asname.clone(),
            },
            _ => panic!("an alias kind stands at an alias place"),
        }
    }

    fn type_params(&mut self, ids: &[usize]) -> Vec<TypeParam> {
        ids.iter()
            .map(|id| {
                let kind = match &self.tree.nodes[*id].data {
                    Data::TypeVar { name } => TypeParamKind::TypeVar {
                        name: name.clone(),
                        bound: self
                            .tree
                            .one(*id, "bound")
                            .map(|expr| Box::new(self.expr(expr))),
                    },
                    Data::TypeVarTuple { name } => TypeParamKind::TypeVarTuple {
                        name: name.clone(),
                    },
                    Data::ParamSpec { name } => TypeParamKind::ParamSpec {
                        name: name.clone(),
                    },
                    _ => panic!("a type parameter kind stands at a type parameter place"),
                };
                TypeParam {
                    id: self.id(),
                    pos: self.pos(*id),
                    kind,
                }
            })
            .collect()
    }
}
