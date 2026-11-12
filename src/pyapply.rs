//! The forty eight seed applies of PORT2-1, each reproducing
//! seed.apply of the landed Python over the arena. One call
//! plants at every match, the matches are collected from the
//! tree as parsed before any change is made, and each matched
//! node is planted exactly once, the standing the module
//! states. The recursive collectors carry the depth ceiling
//! of PORT2-7 and refuse past it in the port's own wording,
//! where the interpreter raises its recursion error. The
//! readings over the flat walk and the explicit stacks carry
//! no ceiling, as theirs carry none.

use crate::pyarena::{refusal, run_big_stack, Ctx, Data, Refusal, Tree, COLLECT_LIMIT};
use crate::pyast::{BinOpKind, BoolKind, CmpKind, ConstVal, PyText, UnaryKind};
use std::collections::{HashMap, HashSet};

/// Apply one seed by its registry id. The answer is the
/// bool the Python returns. An id outside the registry is
/// a refusal.
pub fn apply_seed(tree: &mut Tree, seed_id: &str) -> Result<bool, Refusal> {
    if !crate::registry::SEEDS.iter().any(|seed| seed.id == seed_id) {
        return Err(refusal("the seed id stands outside the registry"));
    }
    run_big_stack(|| apply_known(tree, seed_id))
}

fn apply_known(tree: &mut Tree, seed_id: &str) -> Result<bool, Refusal> {
    Ok(match seed_id {
        "seed-00" => apply_stale_version(tree),
        "seed-01" => apply_dropped_return(tree),
        "seed-02" => apply_inverted_comparison(tree),
        "seed-03" => apply_off_by_one(tree),
        "seed-04" => apply_swallowed_exception(tree),
        "seed-05" => apply_inverted_boolean(tree),
        "seed-06" => apply_swapped_arithmetic(tree),
        "seed-07" => apply_dropped_call_argument(tree),
        "seed-08" => apply_negated_condition(tree),
        "seed-09" => apply_removed_element(tree),
        "seed-10" => apply_range_bound_shift(tree),
        "seed-11" => apply_wrong_variable_returned(tree),
        "seed-12" => apply_swapped_operand_order(tree),
        "seed-13" => apply_dropped_else(tree),
        "seed-14" => apply_swapped_boolean_operator(tree),
        "seed-15" => apply_dropped_await(tree)?,
        "seed-16" => apply_widened_exception_handler(tree),
        "seed-17" => apply_swapped_dictionary_entry(tree),
        "seed-18" => apply_inverted_truth_test(tree),
        "seed-19" => apply_reassigned_loop_variable(tree)?,
        "seed-20" => apply_swapped_call_argument_order(tree),
        "seed-21" => apply_altered_string_literal(tree),
        "seed-22" => apply_dropped_numeric_negation(tree)?,
        "seed-23" => apply_swapped_loop_control(tree)?,
        "seed-24" => apply_swapped_augmented_operator(tree),
        "seed-25" => apply_identity_comparison_as_equality(tree),
        "seed-26" => apply_subscript_index_shift(tree),
        "seed-27" => apply_swapped_keyword_argument_values(tree),
        "seed-28" => apply_swapped_raised_error(tree),
        "seed-29" => apply_slice_stop_shift(tree),
        "seed-30" => apply_shifted_numeric_default(tree),
        "seed-31" => apply_swapped_sibling_attribute(tree)?,
        "seed-32" => apply_narrowed_exception_handler(tree)?,
        "seed-33" => apply_swapped_parameter_defaults(tree),
        "seed-34" => apply_dropped_finally_body(tree),
        "seed-35" => apply_dropped_decorator(tree),
        "seed-36" => apply_dropped_exception_handler(tree),
        "seed-37" => apply_dropped_yield(tree)?,
        "seed-38" => apply_dropped_condition_term(tree),
        "seed-39" => apply_dropped_base_class(tree),
        "seed-40" => apply_slice_start_shift(tree),
        "seed-41" => apply_reversed_loop_control(tree)?,
        "seed-42" => apply_dropped_scope_declaration(tree)?,
        "seed-43" => apply_swapped_assignment_values(tree),
        "seed-44" => apply_dropped_loop_else(tree),
        "seed-45" => apply_dropped_keyword_argument(tree),
        "seed-46" => apply_flipped_range_step(tree),
        "seed-47" => apply_dropped_logical_negation(tree)?,
        _ => return Err(refusal("the seed id stands outside the registry")),
    })
}

// The shared readings.

fn is_int_constant(tree: &Tree, id: usize) -> bool {
    matches!(
        &tree.nodes[id].data,
        Data::Constant {
            value: ConstVal::Int(_)
        }
    )
}

fn is_bool_constant(tree: &Tree, id: usize) -> bool {
    matches!(
        &tree.nodes[id].data,
        Data::Constant {
            value: ConstVal::Bool(_)
        }
    )
}

fn str_constant(tree: &Tree, id: usize) -> Option<&PyText> {
    match &tree.nodes[id].data {
        Data::Constant {
            value: ConstVal::Str(text, _),
        } => Some(text),
        _ => None,
    }
}

/// A numeric zero in any literal spelling, plus sign
/// included, the _is_zero_literal reading. Booleans are
/// never zero literals.
fn is_zero_literal(tree: &Tree, mut id: usize) -> bool {
    if let Data::UnaryOp {
        op: UnaryKind::UAdd,
    } = &tree.nodes[id].data
    {
        if let Some(operand) = tree.one(id, "operand") {
            id = operand;
        }
    }
    match &tree.nodes[id].data {
        Data::Constant { value } => match value {
            ConstVal::Int(text) => text == "0",
            ConstVal::Float(value) => *value == 0.0,
            ConstVal::Complex(real, imag) => *real == 0.0 && *imag == 0.0,
            _ => false,
        },
        _ => false,
    }
}

/// Add one to the canonical decimal spelling of an
/// integer, exactly, at any size.
fn decimal_add_one(text: &str) -> String {
    if let Some(magnitude) = text.strip_prefix('-') {
        if magnitude == "1" {
            return "0".to_string();
        }
        return format!("-{}", decimal_sub_one(magnitude));
    }
    let mut digits: Vec<u8> = text.bytes().collect();
    let mut at = digits.len();
    loop {
        if at == 0 {
            digits.insert(0, b'1');
            break;
        }
        at -= 1;
        if digits[at] == b'9' {
            digits[at] = b'0';
        } else {
            digits[at] += 1;
            break;
        }
    }
    String::from_utf8(digits).expect("decimal digits stay ascii")
}

fn decimal_sub_one(text: &str) -> String {
    let mut digits: Vec<u8> = text.bytes().collect();
    let mut at = digits.len();
    loop {
        at -= 1;
        if digits[at] == b'0' {
            digits[at] = b'9';
        } else {
            digits[at] -= 1;
            break;
        }
    }
    let spelled = String::from_utf8(digits).expect("decimal digits stay ascii");
    let trimmed = spelled.trim_start_matches('0');
    if trimmed.is_empty() {
        "0".to_string()
    } else {
        trimmed.to_string()
    }
}

/// Negate the canonical decimal spelling of an integer.
fn decimal_negate(text: &str) -> String {
    if text == "0" {
        return "0".to_string();
    }
    if let Some(magnitude) = text.strip_prefix('-') {
        magnitude.to_string()
    } else {
        format!("-{text}")
    }
}

fn add_one_to_int(tree: &mut Tree, id: usize) {
    if let Data::Constant {
        value: ConstVal::Int(text),
    } = &mut tree.node_mut(id).data
    {
        *text = decimal_add_one(text);
    }
}

fn fresh(tree: &mut Tree, data: Data, slots: Vec<crate::pyarena::Slot>) -> usize {
    tree.push(data, slots, None)
}

fn fresh_name(tree: &mut Tree, name: &str, ctx: Ctx) -> usize {
    fresh(
        tree,
        Data::Name {
            id: name.to_string(),
            ctx,
        },
        vec![],
    )
}

fn fresh_pass(tree: &mut Tree) -> usize {
    fresh(tree, Data::Pass, vec![])
}

fn one(child: usize) -> crate::pyarena::Slot {
    crate::pyarena::Slot::One(Some(child))
}

/// The exception type names a handler type names, the
/// _handler_type_name_list reading. A bare name names
/// itself, a tuple names its bare name members, and any
/// other shape names nothing.
fn handler_type_name_list(tree: &Tree, type_id: usize) -> Vec<String> {
    match &tree.nodes[type_id].data {
        Data::Name { id, .. } => vec![id.clone()],
        Data::Tuple { .. } => tree
            .many(type_id, "elts")
            .iter()
            .filter_map(|member| match &tree.nodes[*member].data {
                Data::Name { id, .. } => Some(id.clone()),
                _ => None,
            })
            .collect(),
        _ => Vec::new(),
    }
}

const BROAD_HANDLER_NAMES: [&str; 2] = ["Exception", "BaseException"];

/// The type name a raise raises, the _raise_type_name
/// reading. A raise counts only when its exception is a
/// call to a bare name whose first character is uppercase.
fn raise_type_name(tree: &Tree, id: usize) -> Option<String> {
    if !matches!(&tree.nodes[id].data, Data::Raise) {
        return None;
    }
    let exc = tree.one(id, "exc")?;
    if !matches!(&tree.nodes[exc].data, Data::Call) {
        return None;
    }
    let func = tree.one(exc, "func")?;
    match &tree.nodes[func].data {
        Data::Name { id: name, .. } => {
            let first = name.chars().next()?;
            if first.is_uppercase() {
                Some(name.clone())
            } else {
                None
            }
        }
        _ => None,
    }
}

/// The exception names a module names, in the order
/// _module_exception_names gathers them, each name once at
/// the place it first stands.
fn module_exception_names(tree: &Tree) -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    for id in tree.sorted_nodes() {
        let found: Vec<String> = match &tree.nodes[id].data {
            Data::ExceptHandler { .. } => match tree.one(id, "type") {
                Some(type_id) => handler_type_name_list(tree, type_id),
                None => continue,
            },
            _ => match raise_type_name(tree, id) {
                Some(name) => vec![name],
                None => continue,
            },
        };
        for name in found {
            if !names.contains(&name) {
                names.push(name);
            }
        }
    }
    names
}

// One parent carrying entry of the recursive collectors.
struct Hit {
    depth: usize,
    parent: usize,
    field: &'static str,
    index: Option<usize>,
    node: usize,
}

/// Collect the nodes a predicate matches with their parent
/// slots, deepest first, the order the parent carrying
/// collectors of the landed Python return, the sort stable
/// over the field order walk. The walk is recursive and
/// refuses past the depth ceiling of PORT2-7.
fn collect_with_parents(
    tree: &Tree,
    matches: &dyn Fn(&Tree, usize) -> bool,
) -> Result<Vec<Hit>, Refusal> {
    fn visit(
        tree: &Tree,
        node: usize,
        depth: usize,
        matches: &dyn Fn(&Tree, usize) -> bool,
        found: &mut Vec<Hit>,
    ) -> Result<(), Refusal> {
        if depth > COLLECT_LIMIT {
            return Err(refusal(
                "the tree stands past the depth the collectors read",
            ));
        }
        for (field, index, child) in tree.children(node) {
            if matches(tree, child) {
                found.push(Hit {
                    depth: depth + 1,
                    parent: node,
                    field,
                    index,
                    node: child,
                });
            }
            visit(tree, child, depth + 1, matches, found)?;
        }
        Ok(())
    }
    let mut found = Vec::new();
    visit(tree, tree.root, 1, matches, &mut found)?;
    found.sort_by(|a, b| b.depth.cmp(&a.depth));
    Ok(found)
}

/// Replace each hit in its parent slot with one of its own
/// children, the planting of the dropped await, dropped
/// yield, dropped negation, and dropped logical negation
/// seeds.
fn replace_hits_with_child(tree: &mut Tree, hits: Vec<Hit>, child_field: &str) -> bool {
    let planted = !hits.is_empty();
    for hit in hits {
        let child = tree
            .one(hit.node, child_field)
            .expect("the planted child stands");
        *tree.slot_mut(hit.parent, hit.field, hit.index) = child;
    }
    planted
}

fn is_scope_boundary(tree: &Tree, id: usize) -> bool {
    matches!(
        &tree.nodes[id].data,
        Data::FuncDef { .. } | Data::ClassDef { .. } | Data::Lambda
    )
}

// seed-00, stale version string.

const VERSION_NAMES: [&str; 3] = ["VERSION", "__version__", "version"];

fn version_value_constant(tree: &Tree, id: usize) -> Option<usize> {
    match &tree.nodes[id].data {
        Data::Assign => {
            let value = tree.one(id, "value")?;
            if !matches!(&tree.nodes[value].data, Data::Constant { .. }) {
                return None;
            }
            let named = tree.many(id, "targets").iter().any(|target| {
                matches!(&tree.nodes[*target].data, Data::Name { id, .. } if VERSION_NAMES.contains(&id.as_str()))
            });
            if named {
                Some(value)
            } else {
                None
            }
        }
        Data::AnnAssign { .. } => {
            let value = tree.one(id, "value")?;
            if !matches!(&tree.nodes[value].data, Data::Constant { .. }) {
                return None;
            }
            let target = tree.one(id, "target")?;
            match &tree.nodes[target].data {
                Data::Name { id: name, .. } if VERSION_NAMES.contains(&name.as_str()) => {
                    Some(value)
                }
                _ => None,
            }
        }
        _ => None,
    }
}

fn apply_stale_version(tree: &mut Tree) -> bool {
    let mut targets = Vec::new();
    for id in tree.sorted_nodes() {
        if let Some(value) = version_value_constant(tree, id) {
            if str_constant(tree, value).is_some() {
                targets.push(value);
            }
        }
    }
    for value in &targets {
        if let Data::Constant {
            value: ConstVal::Str(text, _),
        } = &mut tree.node_mut(*value).data
        {
            *text = PyText::from_text("9.9.9");
        }
    }
    !targets.is_empty()
}

// seed-01, dropped return value.

fn apply_dropped_return(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(&tree.nodes[*id].data, Data::Return) && tree.one(*id, "value").is_some()
        })
        .collect();
    for id in &targets {
        tree.set_one(*id, "value", None);
    }
    !targets.is_empty()
}

// seed-02, inverted comparison.

fn comparison_flip(op: CmpKind) -> CmpKind {
    match op {
        CmpKind::Eq => CmpKind::NotEq,
        CmpKind::NotEq => CmpKind::Eq,
        CmpKind::Lt => CmpKind::Gt,
        CmpKind::Gt => CmpKind::Lt,
        CmpKind::LtE => CmpKind::GtE,
        CmpKind::GtE => CmpKind::LtE,
        CmpKind::In => CmpKind::NotIn,
        CmpKind::NotIn => CmpKind::In,
        CmpKind::Is => CmpKind::IsNot,
        CmpKind::IsNot => CmpKind::Is,
    }
}

fn apply_inverted_comparison(tree: &mut Tree) -> bool {
    let mut targets = Vec::new();
    for id in tree.sorted_nodes() {
        if let Data::Compare { ops } = &tree.nodes[id].data {
            if let Some(first) = ops.first() {
                targets.push((id, comparison_flip(*first)));
            }
        }
    }
    let planted = !targets.is_empty();
    for (id, flipped) in targets {
        if let Data::Compare { ops } = &mut tree.node_mut(id).data {
            ops[0] = flipped;
        }
    }
    planted
}

// seed-03, off by one constant.

fn apply_off_by_one(tree: &mut Tree) -> bool {
    let mut targets: Vec<usize> = Vec::new();
    let mut seen: HashSet<usize> = HashSet::new();
    for id in tree.sorted_nodes() {
        if !matches!(&tree.nodes[id].data, Data::Return | Data::Compare { .. }) {
            continue;
        }
        // The breadth first walk of ast.walk over the node,
        // the node itself first.
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(id);
        while let Some(current) = queue.pop_front() {
            if is_int_constant(tree, current) && seen.insert(current) {
                targets.push(current);
            }
            for child in tree.child_ids(current) {
                queue.push_back(child);
            }
        }
    }
    for target in &targets {
        add_one_to_int(tree, *target);
    }
    !targets.is_empty()
}

// seed-04, swallowed exception.

fn apply_swallowed_exception(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            if !matches!(&tree.nodes[*id].data, Data::ExceptHandler { .. }) {
                return false;
            }
            let body = tree.many(*id, "body");
            !(body.len() == 1 && matches!(&tree.nodes[body[0]].data, Data::Pass))
        })
        .collect();
    for id in &targets {
        let pass = fresh_pass(tree);
        tree.set_many(*id, "body", vec![pass]);
    }
    !targets.is_empty()
}

// seed-05, inverted boolean literal.

fn apply_inverted_boolean(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| is_bool_constant(tree, *id))
        .collect();
    for id in &targets {
        if let Data::Constant {
            value: ConstVal::Bool(flag),
        } = &mut tree.node_mut(*id).data
        {
            *flag = !*flag;
        }
    }
    !targets.is_empty()
}

// seed-06, swapped arithmetic operator.

fn arithmetic_swap(op: BinOpKind) -> Option<BinOpKind> {
    match op {
        BinOpKind::Add => Some(BinOpKind::Sub),
        BinOpKind::Sub => Some(BinOpKind::Add),
        BinOpKind::Mult => Some(BinOpKind::Add),
        _ => None,
    }
}

fn apply_swapped_arithmetic(tree: &mut Tree) -> bool {
    let mut targets = Vec::new();
    for id in tree.sorted_nodes() {
        if let Data::BinOp { op } = &tree.nodes[id].data {
            if let Some(replacement) = arithmetic_swap(*op) {
                targets.push((id, replacement));
            }
        }
    }
    let planted = !targets.is_empty();
    for (id, replacement) in targets {
        if let Data::BinOp { op } = &mut tree.node_mut(id).data {
            *op = replacement;
        }
    }
    planted
}

// seed-07, dropped call argument.

fn apply_dropped_call_argument(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(&tree.nodes[*id].data, Data::Call) && tree.many(*id, "args").len() >= 2
        })
        .collect();
    for id in &targets {
        let mut args = tree.many(*id, "args").to_vec();
        args.pop();
        tree.set_many(*id, "args", args);
    }
    !targets.is_empty()
}

// seed-08, negated condition.

fn apply_negated_condition(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| matches!(&tree.nodes[*id].data, Data::If))
        .collect();
    for id in &targets {
        let test = tree.one(*id, "test").expect("a test stands");
        let wrapped = fresh(tree, Data::UnaryOp { op: UnaryKind::Not }, vec![one(test)]);
        tree.set_one(*id, "test", Some(wrapped));
    }
    !targets.is_empty()
}

// seed-09, removed collection element.

fn apply_removed_element(tree: &mut Tree) -> bool {
    let mut dicts = Vec::new();
    let mut lists = Vec::new();
    for id in tree.sorted_nodes() {
        match &tree.nodes[id].data {
            Data::Dict if !tree.maybe(id, "keys").is_empty() => dicts.push(id),
            Data::List { .. } if !tree.many(id, "elts").is_empty() => lists.push(id),
            _ => {}
        }
    }
    for id in &dicts {
        let keys = tree.maybe(*id, "keys")[1..].to_vec();
        let values = tree.many(*id, "values")[1..].to_vec();
        tree.set_maybe(*id, "keys", keys);
        tree.set_many(*id, "values", values);
    }
    for id in &lists {
        let elts = tree.many(*id, "elts")[1..].to_vec();
        tree.set_many(*id, "elts", elts);
    }
    !dicts.is_empty() || !lists.is_empty()
}

// seed-10, range bound shifted by one.

fn apply_range_bound_shift(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            if !matches!(&tree.nodes[*id].data, Data::Call) {
                return false;
            }
            let func = tree.one(*id, "func");
            let named_range = match func {
                Some(func) => {
                    matches!(&tree.nodes[func].data, Data::Name { id, .. } if id == "range")
                }
                None => false,
            };
            named_range && !tree.many(*id, "args").is_empty()
        })
        .collect();
    for id in &targets {
        let mut args = tree.many(*id, "args").to_vec();
        let last = *args.last().expect("an argument stands");
        let one_const = fresh(
            tree,
            Data::Constant {
                value: ConstVal::Int("1".to_string()),
            },
            vec![],
        );
        let wrapped = fresh(
            tree,
            Data::BinOp { op: BinOpKind::Add },
            vec![one(last), one(one_const)],
        );
        *args.last_mut().expect("an argument stands") = wrapped;
        tree.set_many(*id, "args", args);
    }
    !targets.is_empty()
}

// seed-11, wrong variable returned.

fn own_returns(tree: &Tree, func: usize) -> Vec<usize> {
    let mut found = Vec::new();
    let mut stack: Vec<usize> = tree.many(func, "body").to_vec();
    while let Some(current) = stack.pop() {
        if is_scope_boundary(tree, current) {
            continue;
        }
        if matches!(&tree.nodes[current].data, Data::Return) {
            found.push(current);
        }
        let mut children = tree.child_ids(current);
        stack.append(&mut children);
    }
    found
}

fn apply_wrong_variable_returned(tree: &mut Tree) -> bool {
    let mut targets: Vec<(usize, String)> = Vec::new();
    let mut planted: HashSet<usize> = HashSet::new();
    for id in tree.sorted_nodes() {
        // FunctionDef only, the async form never judged.
        if !matches!(
            &tree.nodes[id].data,
            Data::FuncDef {
                is_async: false,
                ..
            }
        ) {
            continue;
        }
        let args_node = tree.one(id, "args").expect("args stand");
        let params: Vec<String> = tree
            .many(args_node, "args")
            .iter()
            .map(|arg| match &tree.nodes[*arg].data {
                Data::Arg { name } => name.clone(),
                _ => unreachable!(),
            })
            .collect();
        if params.len() < 2 {
            continue;
        }
        for inner in own_returns(tree, id) {
            let value = match tree.one(inner, "value") {
                Some(value) => value,
                None => continue,
            };
            let returned = match &tree.nodes[value].data {
                Data::Name { id: name, .. } => name.clone(),
                _ => continue,
            };
            if let Some(index) = params.iter().position(|param| *param == returned) {
                if planted.insert(inner) {
                    targets.push((value, params[(index + 1) % params.len()].clone()));
                }
            }
        }
    }
    let planted = !targets.is_empty();
    for (value, name) in targets {
        if let Data::Name { id, .. } = &mut tree.node_mut(value).data {
            *id = name;
        }
    }
    planted
}

// seed-12, swapped operand order.

fn apply_swapped_operand_order(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(&tree.nodes[*id].data, Data::BinOp { op }
                if matches!(op, BinOpKind::Sub | BinOpKind::Div | BinOpKind::FloorDiv | BinOpKind::Mod))
        })
        .collect();
    for id in &targets {
        let left = tree.one(*id, "left").expect("a left stands");
        let right = tree.one(*id, "right").expect("a right stands");
        tree.set_one(*id, "left", Some(right));
        tree.set_one(*id, "right", Some(left));
    }
    !targets.is_empty()
}

// seed-13, dropped else branch.

fn apply_dropped_else(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(&tree.nodes[*id].data, Data::If) && !tree.many(*id, "orelse").is_empty()
        })
        .collect();
    for id in &targets {
        tree.set_many(*id, "orelse", vec![]);
    }
    !targets.is_empty()
}

// seed-14, swapped boolean operator.

fn apply_swapped_boolean_operator(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| matches!(&tree.nodes[*id].data, Data::BoolOp { .. }))
        .collect();
    for id in &targets {
        if let Data::BoolOp { op } = &mut tree.node_mut(*id).data {
            *op = if *op == BoolKind::And {
                BoolKind::Or
            } else {
                BoolKind::And
            };
        }
    }
    !targets.is_empty()
}

// seed-15, dropped await.

fn apply_dropped_await(tree: &mut Tree) -> Result<bool, Refusal> {
    let hits = collect_with_parents(tree, &|tree, id| {
        matches!(&tree.nodes[id].data, Data::Await)
    })?;
    Ok(replace_hits_with_child(tree, hits, "value"))
}

// seed-16, widened exception handler.

fn apply_widened_exception_handler(tree: &mut Tree) -> bool {
    let mut targets = Vec::new();
    for id in tree.sorted_nodes() {
        if !matches!(&tree.nodes[id].data, Data::ExceptHandler { .. }) {
            continue;
        }
        let type_id = match tree.one(id, "type") {
            Some(type_id) => type_id,
            None => continue,
        };
        let names: HashSet<String> = handler_type_name_list(tree, type_id).into_iter().collect();
        if BROAD_HANDLER_NAMES
            .iter()
            .any(|broad| names.contains(*broad))
        {
            continue;
        }
        targets.push(id);
    }
    for id in &targets {
        let name = fresh_name(tree, "Exception", Ctx::Load);
        tree.set_one(*id, "type", Some(name));
    }
    !targets.is_empty()
}

// seed-17, swapped dictionary entry.

fn apply_swapped_dictionary_entry(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(&tree.nodes[*id].data, Data::Dict)
                && tree.maybe(*id, "keys").iter().any(|key| key.is_some())
        })
        .collect();
    for id in &targets {
        let keys = tree.maybe(*id, "keys").to_vec();
        let values = tree.many(*id, "values").to_vec();
        let mut new_keys: Vec<Option<usize>> = Vec::new();
        let mut new_values: Vec<usize> = Vec::new();
        for (key, value) in keys.iter().zip(values.iter()) {
            match key {
                None => {
                    new_keys.push(None);
                    new_values.push(*value);
                }
                Some(key) => {
                    new_keys.push(Some(*value));
                    new_values.push(*key);
                }
            }
        }
        tree.set_maybe(*id, "keys", new_keys);
        tree.set_many(*id, "values", new_values);
    }
    !targets.is_empty()
}

// seed-18, inverted truth test.

fn is_negation(tree: &Tree, id: usize) -> bool {
    matches!(&tree.nodes[id].data, Data::UnaryOp { op: UnaryKind::Not })
}

fn apply_inverted_truth_test(tree: &mut Tree) -> bool {
    let mut targets = Vec::new();
    for id in tree.sorted_nodes() {
        match &tree.nodes[id].data {
            Data::While => targets.push(id),
            Data::If | Data::IfExp => {
                let test = tree.one(id, "test").expect("a test stands");
                if is_negation(tree, test) {
                    targets.push(id);
                }
            }
            _ => {}
        }
    }
    for id in &targets {
        let test = tree.one(*id, "test").expect("a test stands");
        if is_negation(tree, test) {
            let operand = tree.one(test, "operand").expect("an operand stands");
            tree.set_one(*id, "test", Some(operand));
        } else {
            let wrapped = fresh(tree, Data::UnaryOp { op: UnaryKind::Not }, vec![one(test)]);
            tree.set_one(*id, "test", Some(wrapped));
        }
    }
    !targets.is_empty()
}

// seed-19, reassigned loop variable.

fn scope_parameter_names(tree: &Tree, scope: usize) -> Vec<String> {
    let args_node = match &tree.nodes[scope].data {
        Data::FuncDef { .. } | Data::Lambda => tree.one(scope, "args"),
        _ => None,
    };
    let mut names: Vec<String> = Vec::new();
    if let Some(args_node) = args_node {
        let mut all: Vec<usize> = Vec::new();
        all.extend(tree.many(args_node, "posonlyargs").iter().copied());
        all.extend(tree.many(args_node, "args").iter().copied());
        if let Some(vararg) = tree.one(args_node, "vararg") {
            all.push(vararg);
        }
        all.extend(tree.many(args_node, "kwonlyargs").iter().copied());
        if let Some(kwarg) = tree.one(args_node, "kwarg") {
            all.push(kwarg);
        }
        for arg in all {
            if let Data::Arg { name } = &tree.nodes[arg].data {
                if !names.contains(name) {
                    names.push(name.clone());
                }
            }
        }
    }
    names
}

/// The names one scope binds, parameters first in signature
/// order, then body bindings in source position order, the
/// _bound_names_in_scope reading over its explicit stack.
/// The reading takes the position of every binding it
/// notes, so a binding a planting built by hand, carrying
/// no position, is a refusal here in the port's own
/// wording, where the interpreter raises its attribute
/// error over the same reading.
fn bound_names_in_scope(tree: &Tree, scope: usize) -> Result<Vec<String>, Refusal> {
    let mut found: Vec<((u32, u32, usize), String)> = Vec::new();
    if matches!(&tree.nodes[scope].data, Data::FuncDef { .. }) {
        let args_node = tree.one(scope, "args").expect("args stand");
        let mut params: Vec<String> = Vec::new();
        let mut ordered: Vec<usize> = Vec::new();
        ordered.extend(tree.many(args_node, "posonlyargs").iter().copied());
        ordered.extend(tree.many(args_node, "args").iter().copied());
        if let Some(vararg) = tree.one(args_node, "vararg") {
            ordered.push(vararg);
        }
        ordered.extend(tree.many(args_node, "kwonlyargs").iter().copied());
        if let Some(kwarg) = tree.one(args_node, "kwarg") {
            ordered.push(kwarg);
        }
        for arg in ordered {
            if let Data::Arg { name } = &tree.nodes[arg].data {
                params.push(name.clone());
            }
        }
        for (index, name) in params.into_iter().enumerate() {
            found.push(((0, 0, index), name));
        }
    }
    let mut stack: Vec<usize> = tree.many(scope, "body").to_vec();
    while let Some(current) = stack.pop() {
        match &tree.nodes[current].data {
            Data::FuncDef { name, .. } | Data::ClassDef { name } => {
                let pos = binding_position(tree, current)?;
                found.push(((pos.line, pos.col, 0), name.clone()));
                continue;
            }
            Data::Lambda | Data::ListComp | Data::SetComp | Data::DictComp | Data::GeneratorExp => {
                continue;
            }
            Data::Name {
                id: name,
                ctx: Ctx::Store,
            } => {
                let pos = binding_position(tree, current)?;
                found.push(((pos.line, pos.col, 0), name.clone()));
            }
            Data::Import => {
                let pos = binding_position(tree, current)?;
                for (index, alias) in tree.many(current, "names").iter().enumerate() {
                    if let Data::Alias { name, asname } = &tree.nodes[*alias].data {
                        let bound = match asname {
                            Some(asname) => asname.clone(),
                            None => name
                                .split('.')
                                .next()
                                .expect("a segment stands")
                                .to_string(),
                        };
                        found.push(((pos.line, pos.col, index), bound));
                    }
                }
            }
            Data::ImportFrom { .. } => {
                let pos = binding_position(tree, current)?;
                for (index, alias) in tree.many(current, "names").iter().enumerate() {
                    if let Data::Alias { name, asname } = &tree.nodes[*alias].data {
                        let bound = asname.clone().unwrap_or_else(|| name.clone());
                        found.push(((pos.line, pos.col, index), bound));
                    }
                }
            }
            Data::ExceptHandler { name: Some(name) } => {
                let pos = binding_position(tree, current)?;
                found.push(((pos.line, pos.col, 0), name.clone()));
            }
            _ => {}
        }
        let mut children = tree.child_ids(current);
        stack.append(&mut children);
    }
    found.sort();
    let mut names: Vec<String> = Vec::new();
    for (_position, name) in found {
        if !names.contains(&name) {
            names.push(name);
        }
    }
    Ok(names)
}

/// The position of a binding the scope reading notes, or
/// the refusal of the position family when a planting
/// built the binding by hand and it carries none.
fn binding_position(tree: &Tree, id: usize) -> Result<crate::pyast::Pos, Refusal> {
    tree.nodes[id]
        .pos
        .ok_or_else(|| refusal("a binding the planting built by hand carries no position to read"))
}

fn collect_loop_retargets(tree: &Tree) -> Result<Vec<(usize, String)>, Refusal> {
    fn visit(
        tree: &Tree,
        node: usize,
        depth: usize,
        scope: usize,
        found: &mut Vec<(usize, String)>,
    ) -> Result<(), Refusal> {
        if depth > COLLECT_LIMIT {
            return Err(refusal(
                "the tree stands past the depth the collectors read",
            ));
        }
        let scope = if matches!(
            &tree.nodes[node].data,
            Data::FuncDef { .. } | Data::ClassDef { .. }
        ) {
            node
        } else {
            scope
        };
        if matches!(&tree.nodes[node].data, Data::For { .. }) {
            if let Some(target) = tree.one(node, "target") {
                if let Data::Name {
                    id: target_name, ..
                } = &tree.nodes[target].data
                {
                    for name in bound_names_in_scope(tree, scope)? {
                        if name != *target_name {
                            found.push((node, name));
                            break;
                        }
                    }
                }
            }
        }
        for child in tree.child_ids(node) {
            visit(tree, child, depth + 1, scope, found)?;
        }
        Ok(())
    }
    let mut found = Vec::new();
    visit(tree, tree.root, 1, tree.root, &mut found)?;
    Ok(found)
}

fn apply_reassigned_loop_variable(tree: &mut Tree) -> Result<bool, Refusal> {
    let targets = collect_loop_retargets(tree)?;
    for (node, name) in &targets {
        let fresh_target = fresh_name(tree, name, Ctx::Store);
        tree.set_one(*node, "target", Some(fresh_target));
    }
    Ok(!targets.is_empty())
}

// seed-20, swapped call argument order.

fn apply_swapped_call_argument_order(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(&tree.nodes[*id].data, Data::Call) && tree.many(*id, "args").len() >= 2
        })
        .collect();
    for id in &targets {
        let mut args = tree.many(*id, "args").to_vec();
        args.swap(0, 1);
        tree.set_many(*id, "args", args);
    }
    !targets.is_empty()
}

// seed-21, altered string literal.

fn out_of_scope_string_ids(tree: &Tree) -> HashSet<usize> {
    let mut excluded: HashSet<usize> = HashSet::new();
    for id in tree.walk() {
        match &tree.nodes[id].data {
            Data::Module | Data::FuncDef { .. } | Data::ClassDef { .. } => {
                let body = tree.many(id, "body");
                if let Some(first) = body.first() {
                    if matches!(&tree.nodes[*first].data, Data::ExprStmt) {
                        if let Some(value) = tree.one(*first, "value") {
                            if str_constant(tree, value).is_some() {
                                excluded.insert(value);
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        if let Some(value) = version_value_constant(tree, id) {
            if str_constant(tree, value).is_some() {
                excluded.insert(value);
            }
        }
    }
    excluded
}

fn apply_altered_string_literal(tree: &mut Tree) -> bool {
    let excluded = out_of_scope_string_ids(tree);
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            !excluded.contains(id)
                && matches!(str_constant(tree, *id), Some(text) if !text.is_empty())
        })
        .collect();
    for id in &targets {
        if let Data::Constant {
            value: ConstVal::Str(text, _),
        } = &mut tree.node_mut(*id).data
        {
            let mut codepoints = text.codepoints().to_vec();
            codepoints[0] = if codepoints[0] == 'x' as u32 {
                'z' as u32
            } else {
                'x' as u32
            };
            *text = PyText::from_codepoints(codepoints);
        }
    }
    !targets.is_empty()
}

// seed-22, dropped numeric negation.

fn apply_dropped_numeric_negation(tree: &mut Tree) -> Result<bool, Refusal> {
    let hits = collect_with_parents(tree, &|tree, id| {
        matches!(
            &tree.nodes[id].data,
            Data::UnaryOp {
                op: UnaryKind::USub
            }
        )
    })?;
    Ok(replace_hits_with_child(tree, hits, "operand"))
}

// seed-23 and seed-41, the loop body collectors.

/// The breaks or continues standing in the body of a loop,
/// with their parent slots, the _collect_loop_breaks and
/// _collect_loop_continues readings. The inside a loop body
/// reading stands for a loop body, falls for its else
/// clause, and never crosses a function or class or lambda.
fn collect_loop_control(tree: &Tree, breaks: bool) -> Result<Vec<Hit>, Refusal> {
    fn visit(
        tree: &Tree,
        node: usize,
        depth: usize,
        parent: usize,
        field: &'static str,
        index: Option<usize>,
        inside: bool,
        breaks: bool,
        found: &mut Vec<Hit>,
    ) -> Result<(), Refusal> {
        if depth > COLLECT_LIMIT {
            return Err(refusal(
                "the tree stands past the depth the collectors read",
            ));
        }
        let inside = if is_scope_boundary(tree, node) {
            false
        } else {
            inside
        };
        let wanted = if breaks {
            matches!(&tree.nodes[node].data, Data::Break)
        } else {
            matches!(&tree.nodes[node].data, Data::Continue)
        };
        if wanted && inside {
            found.push(Hit {
                depth,
                parent,
                field,
                index,
                node,
            });
        }
        if matches!(&tree.nodes[node].data, Data::For { .. } | Data::While) {
            for (position, statement) in tree.many(node, "body").iter().enumerate() {
                visit(
                    tree,
                    *statement,
                    depth + 1,
                    node,
                    "body",
                    Some(position),
                    true,
                    breaks,
                    found,
                )?;
            }
            for (position, statement) in tree.many(node, "orelse").iter().enumerate() {
                visit(
                    tree,
                    *statement,
                    depth + 1,
                    node,
                    "orelse",
                    Some(position),
                    false,
                    breaks,
                    found,
                )?;
            }
            return Ok(());
        }
        for (child_field, child_index, child) in tree.children(node) {
            visit(
                tree,
                child,
                depth + 1,
                node,
                child_field,
                child_index,
                inside,
                breaks,
                found,
            )?;
        }
        Ok(())
    }
    let mut found = Vec::new();
    visit(
        tree, tree.root, 1, tree.root, "", None, false, breaks, &mut found,
    )?;
    Ok(found)
}

fn apply_swapped_loop_control(tree: &mut Tree) -> Result<bool, Refusal> {
    let hits = collect_loop_control(tree, true)?;
    let planted = !hits.is_empty();
    for hit in hits {
        let replacement = fresh(tree, Data::Continue, vec![]);
        *tree.slot_mut(hit.parent, hit.field, hit.index) = replacement;
    }
    Ok(planted)
}

// seed-24, swapped augmented assignment operator.

fn apply_swapped_augmented_operator(tree: &mut Tree) -> bool {
    let mut targets = Vec::new();
    for id in tree.sorted_nodes() {
        if let Data::AugAssign { op } = &tree.nodes[id].data {
            if let Some(replacement) = arithmetic_swap(*op) {
                targets.push((id, replacement));
            }
        }
    }
    let planted = !targets.is_empty();
    for (id, replacement) in targets {
        if let Data::AugAssign { op } = &mut tree.node_mut(id).data {
            *op = replacement;
        }
    }
    planted
}

// seed-25, identity test replaced by equality.

fn apply_identity_comparison_as_equality(tree: &mut Tree) -> bool {
    let mut targets = Vec::new();
    for id in tree.sorted_nodes() {
        if let Data::Compare { ops } = &tree.nodes[id].data {
            if let Some(first) = ops.first() {
                let swapped = match first {
                    CmpKind::Is => Some(CmpKind::Eq),
                    CmpKind::IsNot => Some(CmpKind::NotEq),
                    _ => None,
                };
                if let Some(swapped) = swapped {
                    targets.push((id, swapped));
                }
            }
        }
    }
    let planted = !targets.is_empty();
    for (id, swapped) in targets {
        if let Data::Compare { ops } = &mut tree.node_mut(id).data {
            ops[0] = swapped;
        }
    }
    planted
}

// seed-26, subscript index shifted by one.

fn apply_subscript_index_shift(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter_map(|id| {
            if matches!(&tree.nodes[id].data, Data::Subscript { .. }) {
                let slice = tree.one(id, "slice")?;
                if is_int_constant(tree, slice) {
                    return Some(slice);
                }
            }
            None
        })
        .collect();
    for target in &targets {
        add_one_to_int(tree, *target);
    }
    !targets.is_empty()
}

// seed-27, swapped keyword argument values.

fn apply_swapped_keyword_argument_values(tree: &mut Tree) -> bool {
    let mut targets: Vec<(usize, usize)> = Vec::new();
    for id in tree.sorted_nodes() {
        if !matches!(&tree.nodes[id].data, Data::Call) {
            continue;
        }
        let named: Vec<usize> = tree
            .many(id, "keywords")
            .iter()
            .copied()
            .filter(|kw| matches!(&tree.nodes[*kw].data, Data::Keyword { arg: Some(_) }))
            .collect();
        if named.len() >= 2 {
            targets.push((named[0], named[1]));
        }
    }
    let planted = !targets.is_empty();
    for (first, second) in targets {
        let first_value = tree.one(first, "value").expect("a value stands");
        let second_value = tree.one(second, "value").expect("a value stands");
        tree.set_one(first, "value", Some(second_value));
        tree.set_one(second, "value", Some(first_value));
    }
    planted
}

// seed-28, swapped raised error.

fn apply_swapped_raised_error(tree: &mut Tree) -> bool {
    let names = module_exception_names(tree);
    let mut targets: Vec<(usize, String)> = Vec::new();
    for id in tree.sorted_nodes() {
        if raise_type_name(tree, id).is_none() {
            continue;
        }
        let exc = tree.one(id, "exc").expect("an exception stands");
        let func = tree.one(exc, "func").expect("a func stands");
        let raised = raise_type_name(tree, id).expect("a name stands");
        for name in &names {
            if *name != raised {
                targets.push((func, name.clone()));
                break;
            }
        }
    }
    let planted = !targets.is_empty();
    for (func, replacement) in targets {
        if let Data::Name { id, .. } = &mut tree.node_mut(func).data {
            *id = replacement;
        }
    }
    planted
}

// seed-29, slice stop shifted by one.

fn apply_slice_stop_shift(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter_map(|id| {
            if matches!(&tree.nodes[id].data, Data::Slice) {
                let upper = tree.one(id, "upper")?;
                if is_int_constant(tree, upper) {
                    return Some(upper);
                }
            }
            None
        })
        .collect();
    for target in &targets {
        add_one_to_int(tree, *target);
    }
    !targets.is_empty()
}

// seed-30, shifted numeric default.

fn apply_shifted_numeric_default(tree: &mut Tree) -> bool {
    let mut targets: Vec<usize> = Vec::new();
    for id in tree.sorted_nodes() {
        if !matches!(&tree.nodes[id].data, Data::FuncDef { .. } | Data::Lambda) {
            continue;
        }
        let args_node = tree.one(id, "args").expect("args stand");
        for default in tree.many(args_node, "defaults").iter().copied() {
            if is_int_constant(tree, default) {
                targets.push(default);
            }
        }
        for default in tree.maybe(args_node, "kw_defaults").iter().copied() {
            if let Some(default) = default {
                if is_int_constant(tree, default) {
                    targets.push(default);
                }
            }
        }
    }
    for target in &targets {
        add_one_to_int(tree, *target);
    }
    !targets.is_empty()
}

// seed-31, swapped sibling attribute.

struct ScopeRecord {
    parameters: HashSet<String>,
    stores: HashMap<String, usize>,
    loads: Vec<usize>,
}

fn collect_sibling_attribute_swaps(tree: &Tree) -> Result<Vec<(usize, String)>, Refusal> {
    fn visit(
        tree: &Tree,
        node: usize,
        depth: usize,
        record: usize,
        records: &mut Vec<ScopeRecord>,
    ) -> Result<(), Refusal> {
        if depth > COLLECT_LIMIT {
            return Err(refusal(
                "the tree stands past the depth the collectors read",
            ));
        }
        if is_scope_boundary(tree, node) {
            // The header of a nested definition reads in the
            // enclosing record, its body in a new one.
            let (headers, body): (Vec<usize>, Vec<usize>) = match &tree.nodes[node].data {
                Data::FuncDef { .. } => {
                    let mut headers: Vec<usize> =
                        tree.many(node, "decorator_list").iter().copied().collect();
                    if let Some(args) = tree.one(node, "args") {
                        headers.push(args);
                    }
                    if let Some(returns) = tree.one(node, "returns") {
                        headers.push(returns);
                    }
                    (headers, tree.many(node, "body").to_vec())
                }
                Data::ClassDef { .. } => {
                    let mut headers: Vec<usize> =
                        tree.many(node, "decorator_list").iter().copied().collect();
                    headers.extend(tree.many(node, "bases").iter().copied());
                    headers.extend(tree.many(node, "keywords").iter().copied());
                    (headers, tree.many(node, "body").to_vec())
                }
                Data::Lambda => {
                    let mut headers = Vec::new();
                    if let Some(args) = tree.one(node, "args") {
                        headers.push(args);
                    }
                    let body = tree
                        .one(node, "body")
                        .map(|body| vec![body])
                        .unwrap_or_default();
                    (headers, body)
                }
                _ => unreachable!(),
            };
            for child in headers {
                visit(tree, child, depth + 1, record, records)?;
            }
            let parameters: HashSet<String> =
                scope_parameter_names(tree, node).into_iter().collect();
            records.push(ScopeRecord {
                parameters,
                stores: HashMap::new(),
                loads: Vec::new(),
            });
            let inner = records.len() - 1;
            for child in body {
                visit(tree, child, depth + 1, inner, records)?;
            }
            return Ok(());
        }
        match &tree.nodes[node].data {
            Data::Name {
                id,
                ctx: Ctx::Store,
            } => {
                *records[record].stores.entry(id.clone()).or_insert(0) += 1;
            }
            Data::Attribute {
                attr: _,
                ctx: Ctx::Load,
            } => {
                if let Some(base) = tree.one(node, "value") {
                    if matches!(&tree.nodes[base].data, Data::Name { .. }) {
                        records[record].loads.push(node);
                    }
                }
            }
            _ => {}
        }
        for child in tree.child_ids(node) {
            visit(tree, child, depth + 1, record, records)?;
        }
        Ok(())
    }
    let mut records = vec![ScopeRecord {
        parameters: scope_parameter_names(tree, tree.root).into_iter().collect(),
        stores: HashMap::new(),
        loads: Vec::new(),
    }];
    visit(tree, tree.root, 1, 0, &mut records)?;
    let mut targets: Vec<(usize, String)> = Vec::new();
    for record in &records {
        let mut bases: Vec<String> = Vec::new();
        let mut loads_by_base: Vec<Vec<usize>> = Vec::new();
        for load in &record.loads {
            let base = match &tree.nodes[tree.one(*load, "value").expect("a base stands")].data {
                Data::Name { id, .. } => id.clone(),
                _ => unreachable!(),
            };
            match bases.iter().position(|name| *name == base) {
                Some(at) => loads_by_base[at].push(*load),
                None => {
                    bases.push(base);
                    loads_by_base.push(vec![*load]);
                }
            }
        }
        for (base, loads) in bases.iter().zip(loads_by_base.iter()) {
            let store_count = record.stores.get(base).copied().unwrap_or(0);
            if record.parameters.contains(base) {
                if store_count != 0 {
                    continue;
                }
            } else if store_count != 1 {
                continue;
            }
            let mut ordered: Vec<usize> = loads.clone();
            ordered.sort_by(|a, b| {
                let pa = tree.nodes[*a].pos.expect("a position stands");
                let pb = tree.nodes[*b].pos.expect("a position stands");
                (pa.line, pa.col).cmp(&(pb.line, pb.col))
            });
            let mut distinct: Vec<String> = Vec::new();
            for load in &ordered {
                if let Data::Attribute { attr, .. } = &tree.nodes[*load].data {
                    if !distinct.contains(attr) {
                        distinct.push(attr.clone());
                    }
                }
            }
            if distinct.len() < 2 {
                continue;
            }
            for load in loads {
                if let Data::Attribute { attr, .. } = &tree.nodes[*load].data {
                    if *attr == distinct[0] {
                        targets.push((*load, distinct[1].clone()));
                    }
                }
            }
        }
    }
    Ok(targets)
}

fn apply_swapped_sibling_attribute(tree: &mut Tree) -> Result<bool, Refusal> {
    let targets = collect_sibling_attribute_swaps(tree)?;
    let planted = !targets.is_empty();
    for (load, replacement) in targets {
        if let Data::Attribute { attr, .. } = &mut tree.node_mut(load).data {
            *attr = replacement;
        }
    }
    Ok(planted)
}

// seed-32, narrowed exception handler.

fn specific_raise_names(tree: &Tree, try_id: usize) -> Result<Vec<String>, Refusal> {
    fn visit(
        tree: &Tree,
        node: usize,
        depth: usize,
        names: &mut Vec<String>,
    ) -> Result<(), Refusal> {
        if depth > COLLECT_LIMIT {
            return Err(refusal(
                "the tree stands past the depth the collectors read",
            ));
        }
        if is_scope_boundary(tree, node) {
            return Ok(());
        }
        if let Some(name) = raise_type_name(tree, node) {
            if !BROAD_HANDLER_NAMES.contains(&name.as_str()) && !names.contains(&name) {
                names.push(name);
            }
        }
        for child in tree.child_ids(node) {
            visit(tree, child, depth + 1, names)?;
        }
        Ok(())
    }
    // The absolute depth of the try node, so the collector
    // ceiling reads on the same measure as the others.
    let parents = tree.parent_map();
    let mut depth = 1usize;
    let mut cursor = try_id;
    while let Some(parent) = parents.get(&cursor) {
        depth += 1;
        cursor = *parent;
    }
    let mut names = Vec::new();
    for statement in tree.many(try_id, "body").to_vec() {
        visit(tree, statement, depth + 1, &mut names)?;
    }
    Ok(names)
}

fn apply_narrowed_exception_handler(tree: &mut Tree) -> Result<bool, Refusal> {
    let mut targets: Vec<(usize, String)> = Vec::new();
    for id in tree.sorted_nodes() {
        if !matches!(&tree.nodes[id].data, Data::Try { .. }) {
            continue;
        }
        let specific = specific_raise_names(tree, id)?;
        if specific.is_empty() {
            continue;
        }
        for handler in tree.many(id, "handlers").to_vec() {
            let type_id = match tree.one(handler, "type") {
                Some(type_id) => type_id,
                None => continue,
            };
            let names: HashSet<String> =
                handler_type_name_list(tree, type_id).into_iter().collect();
            if !BROAD_HANDLER_NAMES
                .iter()
                .any(|broad| names.contains(*broad))
            {
                continue;
            }
            targets.push((handler, specific[0].clone()));
        }
    }
    let planted = !targets.is_empty();
    for (handler, replacement) in targets {
        let name = fresh_name(tree, &replacement, Ctx::Load);
        tree.set_one(handler, "type", Some(name));
    }
    Ok(planted)
}

// seed-33, swapped parameter defaults.

fn apply_swapped_parameter_defaults(tree: &mut Tree) -> bool {
    #[derive(Clone, Copy)]
    enum HolderList {
        Defaults,
        KwDefaults,
    }
    let mut targets: Vec<((usize, HolderList, usize), (usize, HolderList, usize))> = Vec::new();
    for id in tree.sorted_nodes() {
        if !matches!(&tree.nodes[id].data, Data::FuncDef { .. } | Data::Lambda) {
            continue;
        }
        let args_node = tree.one(id, "args").expect("args stand");
        let mut holders: Vec<(HolderList, usize)> = Vec::new();
        for index in 0..tree.many(args_node, "defaults").len() {
            holders.push((HolderList::Defaults, index));
        }
        for (index, default) in tree.maybe(args_node, "kw_defaults").iter().enumerate() {
            if default.is_some() {
                holders.push((HolderList::KwDefaults, index));
            }
        }
        if holders.len() >= 2 {
            targets.push((
                (args_node, holders[0].0, holders[0].1),
                (args_node, holders[1].0, holders[1].1),
            ));
        }
    }
    let planted = !targets.is_empty();
    for ((args_a, list_a, index_a), (args_b, list_b, index_b)) in targets {
        let read = |tree: &Tree, args: usize, list: HolderList, index: usize| -> usize {
            match list {
                HolderList::Defaults => tree.many(args, "defaults")[index],
                HolderList::KwDefaults => {
                    tree.maybe(args, "kw_defaults")[index].expect("a default stands")
                }
            }
        };
        let first = read(tree, args_a, list_a, index_a);
        let second = read(tree, args_b, list_b, index_b);
        let write =
            |tree: &mut Tree, args: usize, list: HolderList, index: usize, value: usize| match list
            {
                HolderList::Defaults => {
                    let mut defaults = tree.many(args, "defaults").to_vec();
                    defaults[index] = value;
                    tree.set_many(args, "defaults", defaults);
                }
                HolderList::KwDefaults => {
                    let mut defaults = tree.maybe(args, "kw_defaults").to_vec();
                    defaults[index] = Some(value);
                    tree.set_maybe(args, "kw_defaults", defaults);
                }
            };
        write(tree, args_a, list_a, index_a, second);
        write(tree, args_b, list_b, index_b, first);
    }
    planted
}

// seed-34, dropped finally body.

fn apply_dropped_finally_body(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(&tree.nodes[*id].data, Data::Try { .. })
                && !tree.many(*id, "finalbody").is_empty()
        })
        .collect();
    for id in &targets {
        if tree.many(*id, "handlers").is_empty() {
            let pass = fresh_pass(tree);
            tree.set_many(*id, "finalbody", vec![pass]);
        } else {
            tree.set_many(*id, "finalbody", vec![]);
        }
    }
    !targets.is_empty()
}

// seed-35, dropped decorator.

fn apply_dropped_decorator(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(
                &tree.nodes[*id].data,
                Data::FuncDef { .. } | Data::ClassDef { .. }
            ) && !tree.many(*id, "decorator_list").is_empty()
        })
        .collect();
    for id in &targets {
        let rest = tree.many(*id, "decorator_list")[1..].to_vec();
        tree.set_many(*id, "decorator_list", rest);
    }
    !targets.is_empty()
}

// seed-36, dropped exception handler.

fn apply_dropped_exception_handler(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            if !matches!(&tree.nodes[*id].data, Data::Try { .. }) {
                return false;
            }
            let handlers = tree.many(*id, "handlers");
            !handlers.is_empty()
                && (handlers.len() >= 2
                    || (!tree.many(*id, "finalbody").is_empty()
                        && tree.many(*id, "orelse").is_empty()))
        })
        .collect();
    for id in &targets {
        let rest = tree.many(*id, "handlers")[1..].to_vec();
        tree.set_many(*id, "handlers", rest);
    }
    !targets.is_empty()
}

// seed-37, dropped yield.

fn apply_dropped_yield(tree: &mut Tree) -> Result<bool, Refusal> {
    let hits = collect_with_parents(tree, &|tree, id| {
        matches!(&tree.nodes[id].data, Data::Yield)
    })?;
    let planted = !hits.is_empty();
    for hit in hits {
        let replacement = match tree.one(hit.node, "value") {
            Some(value) => value,
            None => fresh(
                tree,
                Data::Constant {
                    value: ConstVal::NoneVal,
                },
                vec![],
            ),
        };
        *tree.slot_mut(hit.parent, hit.field, hit.index) = replacement;
    }
    Ok(planted)
}

// seed-38, dropped condition term.

fn apply_dropped_condition_term(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(&tree.nodes[*id].data, Data::BoolOp { .. })
                && tree.many(*id, "values").len() >= 2
        })
        .collect();
    for id in &targets {
        let mut values = tree.many(*id, "values").to_vec();
        values.pop();
        tree.set_many(*id, "values", values);
    }
    !targets.is_empty()
}

// seed-39, dropped base class.

fn apply_dropped_base_class(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(&tree.nodes[*id].data, Data::ClassDef { .. })
                && !tree.many(*id, "bases").is_empty()
        })
        .collect();
    for id in &targets {
        let rest = tree.many(*id, "bases")[1..].to_vec();
        tree.set_many(*id, "bases", rest);
    }
    !targets.is_empty()
}

// seed-40, slice start shifted by one.

fn apply_slice_start_shift(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter_map(|id| {
            if matches!(&tree.nodes[id].data, Data::Slice) {
                let lower = tree.one(id, "lower")?;
                if is_int_constant(tree, lower) {
                    return Some(lower);
                }
            }
            None
        })
        .collect();
    for target in &targets {
        add_one_to_int(tree, *target);
    }
    !targets.is_empty()
}

// seed-41, reversed loop control.

fn apply_reversed_loop_control(tree: &mut Tree) -> Result<bool, Refusal> {
    let hits = collect_loop_control(tree, false)?;
    let planted = !hits.is_empty();
    for hit in hits {
        let replacement = fresh(tree, Data::Break, vec![]);
        *tree.slot_mut(hit.parent, hit.field, hit.index) = replacement;
    }
    Ok(planted)
}

// seed-42, dropped scope declaration.

fn collect_scope_declarations(tree: &Tree) -> Result<Vec<Hit>, Refusal> {
    fn visit(tree: &Tree, node: usize, depth: usize, found: &mut Vec<Hit>) -> Result<(), Refusal> {
        if depth > COLLECT_LIMIT {
            return Err(refusal(
                "the tree stands past the depth the collectors read",
            ));
        }
        for (field, index, child) in tree.children(node) {
            if let Some(index) = index {
                if matches!(
                    &tree.nodes[child].data,
                    Data::Global { .. } | Data::Nonlocal { .. }
                ) {
                    found.push(Hit {
                        depth: depth + 1,
                        parent: node,
                        field,
                        index: Some(index),
                        node: child,
                    });
                }
            }
            visit(tree, child, depth + 1, found)?;
        }
        Ok(())
    }
    let mut found = Vec::new();
    visit(tree, tree.root, 1, &mut found)?;
    Ok(found)
}

fn apply_dropped_scope_declaration(tree: &mut Tree) -> Result<bool, Refusal> {
    let hits = collect_scope_declarations(tree)?;
    // Group by parent and field, in first standing order.
    let mut groups: Vec<(usize, &'static str, Vec<usize>)> = Vec::new();
    for hit in hits {
        match groups
            .iter_mut()
            .find(|(parent, field, _)| *parent == hit.parent && *field == hit.field)
        {
            Some((_, _, indices)) => indices.push(hit.index.expect("an index stands")),
            None => groups.push((
                hit.parent,
                hit.field,
                vec![hit.index.expect("an index stands")],
            )),
        }
    }
    let mut planted = false;
    for (parent, field, indices) in groups {
        let statements = tree.many(parent, field).to_vec();
        if statements.iter().all(|statement| {
            matches!(
                &tree.nodes[*statement].data,
                Data::Global { .. } | Data::Nonlocal { .. }
            )
        }) {
            continue;
        }
        let mut kept: Vec<usize> = statements.clone();
        let mut descending = indices.clone();
        descending.sort_unstable_by(|a, b| b.cmp(a));
        for index in descending {
            kept.remove(index);
        }
        tree.set_many(parent, field, kept);
        planted = true;
    }
    Ok(planted)
}

// seed-43, swapped assignment values.

fn apply_swapped_assignment_values(tree: &mut Tree) -> bool {
    let mut targets: Vec<usize> = Vec::new();
    for id in tree.sorted_nodes() {
        let value = match &tree.nodes[id].data {
            Data::Assign => tree.one(id, "value"),
            Data::AnnAssign { .. } => tree.one(id, "value"),
            _ => None,
        };
        if let Some(value) = value {
            if matches!(&tree.nodes[value].data, Data::Tuple { .. })
                && tree.many(value, "elts").len() >= 2
            {
                targets.push(value);
            }
        }
    }
    for value in &targets {
        let mut elts = tree.many(*value, "elts").to_vec();
        elts.swap(0, 1);
        tree.set_many(*value, "elts", elts);
    }
    !targets.is_empty()
}

// seed-44, dropped loop else clause.

fn apply_dropped_loop_else(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(&tree.nodes[*id].data, Data::For { .. } | Data::While)
                && !tree.many(*id, "orelse").is_empty()
        })
        .collect();
    for id in &targets {
        tree.set_many(*id, "orelse", vec![]);
    }
    !targets.is_empty()
}

// seed-45, dropped keyword argument.

fn apply_dropped_keyword_argument(tree: &mut Tree) -> bool {
    let mut targets: Vec<(usize, usize)> = Vec::new();
    for id in tree.sorted_nodes() {
        if !matches!(&tree.nodes[id].data, Data::Call) {
            continue;
        }
        let named = tree
            .many(id, "keywords")
            .iter()
            .copied()
            .find(|kw| matches!(&tree.nodes[*kw].data, Data::Keyword { arg: Some(_) }));
        if let Some(keyword) = named {
            targets.push((id, keyword));
        }
    }
    let planted = !targets.is_empty();
    for (id, keyword) in targets {
        let mut keywords = tree.many(id, "keywords").to_vec();
        keywords.retain(|kw| *kw != keyword);
        tree.set_many(id, "keywords", keywords);
    }
    planted
}

// seed-46, flipped range step sign.

fn apply_flipped_range_step(tree: &mut Tree) -> bool {
    let mut targets: Vec<usize> = Vec::new();
    for id in tree.sorted_nodes() {
        if !matches!(&tree.nodes[id].data, Data::Call) {
            continue;
        }
        let func = tree.one(id, "func");
        let named_range = match func {
            Some(func) => matches!(&tree.nodes[func].data, Data::Name { id, .. } if id == "range"),
            None => false,
        };
        if !named_range {
            continue;
        }
        let args = tree.many(id, "args");
        if args.len() != 3 {
            continue;
        }
        let step = args[2];
        if matches!(&tree.nodes[step].data, Data::Starred { .. }) {
            continue;
        }
        if matches!(
            &tree.nodes[step].data,
            Data::UnaryOp {
                op: UnaryKind::USub
            }
        ) {
            continue;
        }
        if is_bool_constant(tree, step) {
            continue;
        }
        if is_zero_literal(tree, step) {
            continue;
        }
        targets.push(id);
    }
    for id in &targets {
        let mut args = tree.many(*id, "args").to_vec();
        let step = args[2];
        if is_int_constant(tree, step) {
            let negated = match &tree.nodes[step].data {
                Data::Constant {
                    value: ConstVal::Int(text),
                } => decimal_negate(text),
                _ => unreachable!(),
            };
            let replacement = fresh(
                tree,
                Data::Constant {
                    value: ConstVal::Int(negated),
                },
                vec![],
            );
            args[2] = replacement;
        } else {
            let wrapped = fresh(
                tree,
                Data::UnaryOp {
                    op: UnaryKind::USub,
                },
                vec![one(step)],
            );
            args[2] = wrapped;
        }
        tree.set_many(*id, "args", args);
    }
    !targets.is_empty()
}

// seed-47, dropped logical negation.

fn collect_outer_negations(tree: &Tree) -> Result<Vec<Hit>, Refusal> {
    fn visit(
        tree: &Tree,
        node: usize,
        depth: usize,
        in_test: bool,
        found: &mut Vec<Hit>,
    ) -> Result<(), Refusal> {
        if depth > COLLECT_LIMIT {
            return Err(refusal(
                "the tree stands past the depth the collectors read",
            ));
        }
        for (field, index, child) in tree.children(node) {
            let child_in_test = in_test
                || (field == "test"
                    && matches!(&tree.nodes[node].data, Data::If | Data::While | Data::IfExp))
                || (field == "guard" && matches!(&tree.nodes[node].data, Data::MatchCase));
            if !child_in_test
                && matches!(
                    &tree.nodes[child].data,
                    Data::UnaryOp { op: UnaryKind::Not }
                )
            {
                found.push(Hit {
                    depth: depth + 1,
                    parent: node,
                    field,
                    index,
                    node: child,
                });
            }
            visit(tree, child, depth + 1, child_in_test, found)?;
        }
        Ok(())
    }
    let mut found = Vec::new();
    visit(tree, tree.root, 1, false, &mut found)?;
    found.sort_by(|a, b| b.depth.cmp(&a.depth));
    Ok(found)
}

fn apply_dropped_logical_negation(tree: &mut Tree) -> Result<bool, Refusal> {
    let hits = collect_outer_negations(tree)?;
    Ok(replace_hits_with_child(tree, hits, "operand"))
}
