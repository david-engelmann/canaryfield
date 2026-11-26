//! The fifty two seed applies, the first forty eight
//! reproducing seed.apply of the landed Python over the arena
//! under PORT2-1 and the four feature slice 2 grows the
//! registry with implementing the obligations of the
//! contract's feature slice 2 section, their battery entries
//! recorded by the independent implementation that section
//! names. One call
//! plants at every match, the matches are collected from the
//! tree as parsed before any change is made, and each matched
//! node is planted exactly once, the standing the module
//! states. The recursive collectors carry the depth ceiling
//! of PORT2-7 and refuse past it in the port's own wording,
//! where the interpreter raises its recursion error. The
//! readings over the flat walk and the explicit stacks carry
//! no ceiling, as theirs carry none.

use crate::pyarena::{refusal, run_big_stack, Ctx, Data, Refusal, Slot, Tree, COLLECT_LIMIT};
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
        "seed-48" => apply_dropped_try_else(tree),
        "seed-49" => apply_slice_step_shift(tree),
        "seed-50" => apply_swapped_conditional_arms(tree),
        "seed-51" => apply_dropped_exception_chain(tree),
        "seed-52" => apply_dropped_slice_bound(tree),
        "seed-53" => apply_dropped_augmentation(tree),
        "seed-54" => apply_swapped_parameter_order(tree),
        "seed-55" => apply_dropped_import(tree),
        "seed-56" => apply_dropped_return_statement(tree),
        "seed-57" => apply_swapped_boolean_operands(tree),
        "seed-58" => apply_dropped_comprehension_condition(tree),
        "seed-59" => apply_flipped_later_comparison_operator(tree),
        "seed-60" => apply_swapped_dictionary_entry_values(tree),
        "seed-61" => apply_swapped_sequence_literal_elements(tree),
        "seed-62" => apply_dropped_assert_statement(tree),
        "seed-63" => apply_added_numeric_negation(tree)?,
        "seed-64" => apply_dropped_with_target(tree),
        "seed-65" => apply_dropped_assignment_expression(tree)?,
        "seed-66" => apply_dropped_delete_statement(tree),
        "seed-67" => apply_dropped_match_case(tree),
        "seed-68" => apply_swapped_slice_bounds(tree),
        "seed-69" => apply_swapped_branch_bodies(tree),
        "seed-70" => apply_dropped_match_guard(tree),
        "seed-71" => apply_dropped_yield_from_delegation(tree),
        "seed-72" => apply_swapped_comprehension_generator_order(tree),
        "seed-73" => apply_dropped_argument_unpacking(tree)?,
        "seed-74" => apply_swapped_formatted_string_values(tree),
        "seed-75" => apply_dropped_call_statement(tree),
        "seed-76" => apply_swapped_decorator_order(tree),
        "seed-77" => apply_dropped_parameter_default(tree),
        "seed-78" => apply_swapped_base_class_order(tree),
        "seed-79" => apply_swapped_match_case_order(tree),
        "seed-80" => apply_swapped_with_items(tree),
        "seed-81" => apply_dropped_asynchronous_modifier(tree),
        "seed-82" => apply_swapped_keyword_argument_order(tree),
        "seed-83" => apply_swapped_import_names(tree),
        "seed-84" => apply_dropped_with_item(tree),
        "seed-85" => apply_swapped_try_handler_order(tree),
        "seed-86" => apply_dropped_argument_annotation(tree),
        "seed-87" => apply_swapped_assignment_target_order(tree),
        "seed-88" => apply_dropped_return_annotation(tree),
        "seed-89" => apply_swapped_statement_order(tree),
        "seed-90" => apply_swapped_comprehension_condition_order(tree),
        "seed-91" => apply_dropped_starred_argument(tree),
        "seed-92" => apply_dropped_raise_statement(tree),
        "seed-93" => apply_dropped_starred_display_element(tree),
        "seed-94" => apply_dropped_loop_control_statement(tree)?,
        "seed-95" => apply_swapped_try_else_bodies(tree),
        "seed-96" => apply_dropped_class_keyword_argument(tree),
        "seed-97" => apply_swapped_loop_else_bodies(tree),
        "seed-98" => apply_dropped_starred_parameter(tree),
        "seed-99" => apply_dropped_docstring(tree),
        "seed-100" => apply_dropped_parameter(tree),
        "seed-101" => apply_dropped_elif_branch(tree),
        "seed-102" => apply_dropped_import_alias(tree),
        "seed-103" => apply_dropped_yield_value(tree),
        "seed-104" => apply_dropped_assert_message(tree),
        "seed-105" => apply_dropped_formatted_value_conversion(tree),
        "seed-106" => apply_swapped_loop_target_elements(tree),
        "seed-107" => apply_dropped_keyword_only_parameter(tree),
        "seed-108" => apply_dropped_format_spec(tree),
        "seed-109" => apply_dropped_comparison_tail(tree),
        "seed-110" => apply_dropped_imported_name(tree),
        "seed-111" => apply_dropped_keyword_only_default(tree),
        "seed-112" => apply_dropped_slice_step(tree),
        "seed-113" => apply_dropped_annotation_value(tree),
        "seed-114" => apply_dropped_formatted_value(tree),
        "seed-115" => apply_dropped_raised_error_message(tree),
        "seed-116" => apply_dropped_set_element(tree),
        "seed-117" => apply_dropped_decorator_call(tree),
        "seed-118" => apply_dropped_attribute(tree)?,
        "seed-119" => apply_dropped_exception_type(tree),
        "seed-120" => apply_dropped_subscript(tree)?,
        "seed-121" => apply_dropped_arithmetic_operand(tree)?,
        "seed-122" => apply_dropped_first_call_argument(tree),
        "seed-123" => apply_dropped_first_condition_term(tree),
        "seed-124" => apply_dropped_tuple_element(tree),
        "seed-125" => apply_dropped_conditional_test(tree)?,
        "seed-126" => apply_dropped_comparison_head(tree),
        "seed-127" => apply_dropped_assignment(tree),
        "seed-128" => apply_dropped_annotated_assignment(tree),
        "seed-129" => apply_dropped_function_definition(tree),
        "seed-130" => apply_dropped_class_definition(tree),
        "seed-131" => apply_dropped_exception_handler_name(tree),
        "seed-132" => apply_dropped_assignment_target(tree),
        "seed-133" => apply_dropped_last_tuple_element(tree),
        "seed-134" => apply_dropped_last_base_class(tree),
        "seed-135" => apply_dropped_decorator_argument(tree),
        "seed-136" => apply_dropped_unpacking_target_element(tree),
        "seed-137" => apply_dropped_last_unpacking_target_element(tree),
        "seed-138" => apply_dropped_keyword_unpacking(tree),
        "seed-139" => apply_dropped_last_dictionary_entry(tree),
        "seed-140" => apply_dropped_last_keyword_argument(tree),
        "seed-141" => apply_dropped_last_list_element(tree),
        "seed-142" => apply_dropped_first_imported_name(tree),
        "seed-143" => apply_dropped_first_boolean_operand(tree),
        "seed-144" => apply_dropped_last_call_argument(tree),
        "seed-145" => apply_dropped_last_parameter(tree),
        "seed-146" => apply_dropped_subscript_index_element(tree),
        "seed-147" => apply_dropped_last_exception_handler(tree),
        "seed-148" => apply_dropped_last_fstring_element(tree),
        "seed-149" => apply_dropped_last_return_element(tree),
        "seed-150" => apply_dropped_first_subscript_index_element(tree),
        "seed-151" => apply_dropped_last_set_element(tree),
        "seed-152" => apply_dropped_first_fstring_element(tree),
        "seed-153" => apply_dropped_first_return_element(tree),
        "seed-154" => apply_dropped_last_match_case(tree),
        "seed-155" => apply_dropped_last_decorator(tree),
        "seed-156" => apply_dropped_last_assignment_target(tree),
        "seed-157" => apply_dropped_dictionary_unpacking_entry(tree),
        "seed-158" => apply_dropped_first_assignment_value_element(tree),
        "seed-159" => apply_dropped_last_assignment_value_element(tree),
        "seed-160" => apply_dropped_last_tuple_argument_element(tree),
        "seed-161" => apply_dropped_first_tuple_argument_element(tree),
        "seed-162" => apply_dropped_last_for_loop_tuple_element(tree),
        "seed-163" => apply_dropped_first_for_loop_tuple_element(tree),
        "seed-164" => apply_dropped_last_annotated_assignment_value_element(tree),
        "seed-165" => apply_dropped_first_annotated_assignment_value_element(tree),
        "seed-166" => apply_dropped_last_comprehension_tuple_element(tree),
        "seed-167" => apply_dropped_first_comprehension_tuple_element(tree),
        "seed-168" => apply_dropped_last_lambda_body_tuple_element(tree),
        "seed-169" => apply_dropped_first_lambda_body_tuple_element(tree),
        "seed-170" => apply_dropped_last_comparison_operand_tuple_element(tree),
        "seed-171" => apply_dropped_first_comparison_operand_tuple_element(tree),
        "seed-172" => apply_dropped_last_parameter_default_tuple_element(tree),
        "seed-173" => apply_dropped_first_parameter_default_tuple_element(tree),
        "seed-174" => apply_dropped_last_conditional_expression_tuple_element(tree),
        "seed-175" => apply_dropped_first_conditional_expression_tuple_element(tree),
        "seed-176" => apply_dropped_last_exception_handler_type_tuple_element(tree),
        "seed-177" => apply_dropped_first_exception_handler_type_tuple_element(tree),
        "seed-178" => apply_dropped_last_keyword_value_tuple_element(tree),
        "seed-179" => apply_dropped_first_keyword_value_tuple_element(tree),
        _ => return Err(refusal("the seed id stands outside the registry")),
    })
}

// seed-100, dropped parameter.

fn apply_dropped_parameter(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            if !matches!(&tree.nodes[*id].data, Data::FuncDef { .. } | Data::Lambda) {
                return false;
            }
            let Some(args_node) = tree.one(*id, "args") else {
                return false;
            };
            !tree.many(args_node, "posonlyargs").is_empty()
                || !tree.many(args_node, "args").is_empty()
        })
        .collect();
    for id in &targets {
        let args_node = tree.one(*id, "args").expect("the arguments stand");
        let posonly = tree.many(args_node, "posonlyargs").to_vec();
        let ordinary = tree.many(args_node, "args").to_vec();
        let mut defaults = tree.many(args_node, "defaults").to_vec();
        if defaults.len() == posonly.len() + ordinary.len() {
            defaults.remove(0);
            tree.set_many(args_node, "defaults", defaults);
        }
        if !posonly.is_empty() {
            tree.set_many(args_node, "posonlyargs", posonly[1..].to_vec());
        } else {
            tree.set_many(args_node, "args", ordinary[1..].to_vec());
        }
    }
    !targets.is_empty()
}

// seed-101, dropped elif branch.

fn apply_dropped_elif_branch(tree: &mut Tree) -> bool {
    let mut planted = false;
    loop {
        let targets: Vec<usize> = tree
            .sorted_nodes()
            .into_iter()
            .filter(|id| {
                if !matches!(&tree.nodes[*id].data, Data::If { .. }) {
                    return false;
                }
                let orelse = tree.many(*id, "orelse");
                orelse.len() == 1 && matches!(&tree.nodes[orelse[0]].data, Data::If { .. })
            })
            .collect();
        if targets.is_empty() {
            break;
        }
        planted = true;
        for id in &targets {
            let inner = tree.many(*id, "orelse")[0];
            let rest = tree.many(inner, "orelse").to_vec();
            tree.set_many(*id, "orelse", rest);
        }
    }
    planted
}

// seed-102, dropped import alias.

fn apply_dropped_import_alias(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(
                &tree.nodes[*id].data,
                Data::Import | Data::ImportFrom { .. }
            ) && tree.many(*id, "names").iter().any(|alias| {
                matches!(
                    &tree.nodes[*alias].data,
                    Data::Alias {
                        asname: Some(_),
                        ..
                    }
                )
            })
        })
        .collect();
    for id in &targets {
        let names = tree.many(*id, "names").to_vec();
        for alias in names {
            if matches!(
                &tree.nodes[alias].data,
                Data::Alias {
                    asname: Some(_),
                    ..
                }
            ) {
                if let Data::Alias { asname, .. } = &mut tree.node_mut(alias).data {
                    *asname = None;
                }
                break;
            }
        }
    }
    !targets.is_empty()
}

// seed-103, dropped yield value.

fn apply_dropped_yield_value(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(&tree.nodes[*id].data, Data::Yield) && tree.one(*id, "value").is_some()
        })
        .collect();
    for id in &targets {
        tree.set_one(*id, "value", None);
    }
    !targets.is_empty()
}

// seed-104, dropped assert message. The msg child of an
// assert clears the way seed-103 clears a yield's value,
// the test standing exactly as written.

fn apply_dropped_assert_message(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(&tree.nodes[*id].data, Data::Assert) && tree.one(*id, "msg").is_some()
        })
        .collect();
    for id in &targets {
        tree.set_one(*id, "msg", None);
    }
    !targets.is_empty()
}

// seed-105, dropped formatted value conversion. The
// conversion a formatted value carries is a scalar on the
// node, and it clears to the plain rendering the way
// seed-102 clears an alias name.

fn apply_dropped_formatted_value_conversion(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(
                &tree.nodes[*id].data,
                Data::FormattedValue { conversion } if *conversion != -1
            )
        })
        .collect();
    for id in &targets {
        if let Data::FormattedValue { conversion, .. } = &mut tree.node_mut(*id).data {
            *conversion = -1;
        }
    }
    !targets.is_empty()
}

// seed-106, swapped loop target elements. The exchange
// seed-87 performs on an assignment target stands performed
// on a loop statement's own target, the comprehension
// targets standing outside both readings.

fn apply_swapped_loop_target_elements(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| matches!(&tree.nodes[*id].data, Data::For { .. }))
        .collect();
    let mut planted = false;
    for id in &targets {
        let Some(target) = tree.one(*id, "target") else {
            continue;
        };
        if !matches!(
            &tree.nodes[target].data,
            Data::Tuple { .. } | Data::List { .. }
        ) {
            continue;
        }
        let elts = tree.many(target, "elts");
        if elts.len() < 2 {
            continue;
        }
        let mut swapped = elts.to_vec();
        swapped.swap(0, 1);
        tree.set_many(target, "elts", swapped);
        planted = true;
    }
    planted
}

// seed-107, dropped keyword-only parameter. The removal
// seed-100 performs on the first positional parameter
// stands performed on the first keyword-only parameter,
// its entry among the keyword-only defaults traveling
// with it the way a positional default travels.

fn apply_dropped_keyword_only_parameter(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            if !matches!(&tree.nodes[*id].data, Data::FuncDef { .. } | Data::Lambda) {
                return false;
            }
            let Some(args_node) = tree.one(*id, "args") else {
                return false;
            };
            !tree.many(args_node, "kwonlyargs").is_empty()
        })
        .collect();
    for id in &targets {
        let args_node = tree.one(*id, "args").expect("the arguments stand");
        let kwonly = tree.many(args_node, "kwonlyargs").to_vec();
        tree.set_many(args_node, "kwonlyargs", kwonly[1..].to_vec());
        let mut kw_defaults = tree.maybe(args_node, "kw_defaults").to_vec();
        if !kw_defaults.is_empty() {
            kw_defaults.remove(0);
            tree.set_maybe(args_node, "kw_defaults", kw_defaults);
        }
    }
    !targets.is_empty()
}

// seed-108, dropped format spec. The spec a formatted
// value carries is a child standing beside the value, and
// it clears the way seed-105 leaves it standing, the two
// plantings sharing the seat and each touching only its
// own part of the node.

fn apply_dropped_format_spec(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(&tree.nodes[*id].data, Data::FormattedValue { .. })
                && tree.one(*id, "format_spec").is_some()
        })
        .collect();
    for id in &targets {
        tree.set_one(*id, "format_spec", None);
    }
    !targets.is_empty()
}

// seed-109, dropped comparison tail. The last comparator
// and the operator standing before it leave together,
// once, the chain keeping its left operand and its first
// terms the way seed-59 leaves every standing operator
// as written while it exchanges the later ones.

fn apply_dropped_comparison_tail(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| match &tree.nodes[*id].data {
            Data::Compare { ops } => ops.len() >= 2,
            _ => false,
        })
        .collect();
    for id in &targets {
        if let Data::Compare { ops } = &mut tree.node_mut(*id).data {
            ops.pop();
        }
        let comparators = tree.many(*id, "comparators").to_vec();
        let kept = comparators.len() - 1;
        tree.set_many(*id, "comparators", comparators[..kept].to_vec());
    }
    !targets.is_empty()
}

// seed-110, dropped imported name. The last name of a
// from-import carrying at least two names leaves whole,
// its alias traveling with it, where seed-55 removes the
// statement whole and seed-102 clears only an alias.

fn apply_dropped_imported_name(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(&tree.nodes[*id].data, Data::ImportFrom { .. })
                && tree.many(*id, "names").len() >= 2
        })
        .collect();
    for id in &targets {
        let names = tree.many(*id, "names").to_vec();
        let kept = names.len() - 1;
        tree.set_many(*id, "names", names[..kept].to_vec());
    }
    !targets.is_empty()
}

// seed-111, dropped keyword-only default. The first
// keyword-only default standing present clears to no
// default, the parameter standing in its place, where
// seed-77 removes the first positional default and
// seed-107 removes the keyword-only parameter whole.

fn apply_dropped_keyword_only_default(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            if !matches!(&tree.nodes[*id].data, Data::FuncDef { .. } | Data::Lambda) {
                return false;
            }
            let Some(args_node) = tree.one(*id, "args") else {
                return false;
            };
            tree.maybe(args_node, "kw_defaults")
                .iter()
                .any(|entry| entry.is_some())
        })
        .collect();
    for id in &targets {
        let args_node = tree.one(*id, "args").expect("the arguments stand");
        let mut kw_defaults = tree.maybe(args_node, "kw_defaults").to_vec();
        if let Some(first) = kw_defaults.iter_mut().find(|entry| entry.is_some()) {
            *first = None;
        }
        tree.set_maybe(args_node, "kw_defaults", kw_defaults);
    }
    !targets.is_empty()
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

// seed-48, dropped try else clause.

fn apply_dropped_try_else(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(&tree.nodes[*id].data, Data::Try { .. })
                && !tree.many(*id, "orelse").is_empty()
        })
        .collect();
    for id in &targets {
        tree.set_many(*id, "orelse", vec![]);
    }
    !targets.is_empty()
}

// seed-49, shifted slice step.

fn apply_slice_step_shift(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter_map(|id| {
            if matches!(&tree.nodes[id].data, Data::Slice) {
                let step = tree.one(id, "step")?;
                if is_int_constant(tree, step) {
                    return Some(step);
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

// seed-50, swapped conditional expression arms.

fn apply_swapped_conditional_arms(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| matches!(&tree.nodes[*id].data, Data::IfExp))
        .collect();
    for id in &targets {
        let body = tree.one(*id, "body").expect("a body stands");
        let orelse = tree.one(*id, "orelse").expect("an orelse stands");
        tree.set_one(*id, "body", Some(orelse));
        tree.set_one(*id, "orelse", Some(body));
    }
    !targets.is_empty()
}

// seed-51, dropped exception chain.

fn apply_dropped_exception_chain(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(&tree.nodes[*id].data, Data::Raise) && tree.one(*id, "cause").is_some()
        })
        .collect();
    for id in &targets {
        tree.set_one(*id, "cause", None);
    }
    !targets.is_empty()
}

// seed-52, dropped slice bound.

fn apply_dropped_slice_bound(tree: &mut Tree) -> bool {
    let targets: Vec<(usize, &'static str)> = tree
        .sorted_nodes()
        .into_iter()
        .filter_map(|id| {
            if matches!(&tree.nodes[id].data, Data::Slice) {
                if tree.one(id, "lower").is_some() {
                    return Some((id, "lower"));
                }
                if tree.one(id, "upper").is_some() {
                    return Some((id, "upper"));
                }
            }
            None
        })
        .collect();
    for (id, field) in &targets {
        tree.set_one(*id, field, None);
    }
    !targets.is_empty()
}

// seed-53, dropped augmentation.

fn apply_dropped_augmentation(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| matches!(&tree.nodes[*id].data, Data::AugAssign { .. }))
        .collect();
    for id in &targets {
        let target = tree.one(*id, "target").expect("a target stands");
        let value = tree.one(*id, "value").expect("a value stands");
        let node = tree.node_mut(*id);
        node.data = Data::Assign;
        node.slots = vec![Slot::Many(vec![target]), Slot::One(Some(value))];
    }
    !targets.is_empty()
}

// seed-54, swapped parameter order.

fn apply_swapped_parameter_order(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            if !matches!(&tree.nodes[*id].data, Data::FuncDef { .. } | Data::Lambda) {
                return false;
            }
            let Some(args_node) = tree.one(*id, "args") else {
                return false;
            };
            tree.many(args_node, "posonlyargs").len() + tree.many(args_node, "args").len() >= 2
        })
        .collect();
    for id in &targets {
        let args_node = tree.one(*id, "args").expect("the arguments stand");
        let posonly = tree.many(args_node, "posonlyargs").to_vec();
        let ordinary = tree.many(args_node, "args").to_vec();
        if posonly.len() >= 2 {
            let mut swapped = posonly.clone();
            swapped.swap(0, 1);
            tree.set_many(args_node, "posonlyargs", swapped);
        } else if posonly.len() == 1 {
            let first = posonly[0];
            let second = ordinary[0];
            tree.set_many(args_node, "posonlyargs", vec![second]);
            let mut rest = ordinary.clone();
            rest[0] = first;
            tree.set_many(args_node, "args", rest);
        } else {
            let mut swapped = ordinary.clone();
            swapped.swap(0, 1);
            tree.set_many(args_node, "args", swapped);
        }
    }
    !targets.is_empty()
}

// seed-55, dropped import.

fn apply_dropped_import(tree: &mut Tree) -> bool {
    let hits = match collect_with_parents(tree, &|tree, id| {
        matches!(&tree.nodes[id].data, Data::Import | Data::ImportFrom { .. })
    }) {
        Ok(hits) => hits,
        Err(_) => return false,
    };
    let mut planted = false;
    // The removals run deepest first so a removal never moves
    // the standing of a hit still to come in the same body, and
    // hits in one list run from its end for the same reason.
    let mut hits = hits;
    hits.sort_by(|a, b| {
        b.depth
            .cmp(&a.depth)
            .then(b.parent.cmp(&a.parent))
            .then(b.index.cmp(&a.index))
    });
    for hit in hits {
        let Some(index) = hit.index else {
            continue;
        };
        let siblings = tree.many(hit.parent, hit.field).to_vec();
        if siblings.len() <= 1 && !matches!(&tree.nodes[hit.parent].data, Data::Module) {
            continue;
        }
        if siblings.get(index) != Some(&hit.node) {
            continue;
        }
        let mut rest = siblings;
        rest.remove(index);
        tree.set_many(hit.parent, hit.field, rest);
        planted = true;
    }
    planted
}

// seed-56, dropped return statement.

fn apply_dropped_return_statement(tree: &mut Tree) -> bool {
    let hits = match collect_with_parents(tree, &|tree, id| {
        matches!(&tree.nodes[id].data, Data::Return)
    }) {
        Ok(hits) => hits,
        Err(_) => return false,
    };
    let mut planted = false;
    // The removals run deepest first so a removal never moves
    // the standing of a hit still to come in the same body, and
    // hits in one list run from its end for the same reason.
    let mut hits = hits;
    hits.sort_by(|a, b| {
        b.depth
            .cmp(&a.depth)
            .then(b.parent.cmp(&a.parent))
            .then(b.index.cmp(&a.index))
    });
    for hit in hits {
        let Some(index) = hit.index else {
            continue;
        };
        let siblings = tree.many(hit.parent, hit.field).to_vec();
        if siblings.len() <= 1 && !matches!(&tree.nodes[hit.parent].data, Data::Module) {
            continue;
        }
        if siblings.get(index) != Some(&hit.node) {
            continue;
        }
        let mut rest = siblings;
        rest.remove(index);
        tree.set_many(hit.parent, hit.field, rest);
        planted = true;
    }
    planted
}

// seed-57, swapped boolean operands.

fn apply_swapped_boolean_operands(tree: &mut Tree) -> bool {
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
        values.swap(0, 1);
        tree.set_many(*id, "values", values);
    }
    !targets.is_empty()
}

// seed-58, dropped comprehension condition.

fn apply_dropped_comprehension_condition(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(
                &tree.nodes[*id].data,
                Data::ListComp | Data::SetComp | Data::GeneratorExp | Data::DictComp
            )
        })
        .collect();
    let mut planted = false;
    for id in &targets {
        for clause in tree.many(*id, "generators").to_vec() {
            if !tree.many(clause, "ifs").is_empty() {
                let rest = tree.many(clause, "ifs")[1..].to_vec();
                tree.set_many(clause, "ifs", rest);
                planted = true;
            }
        }
    }
    planted
}

// seed-59, flipped later comparison operator.

fn apply_flipped_later_comparison_operator(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| match &tree.nodes[*id].data {
            Data::Compare { ops } => ops.len() >= 2,
            _ => false,
        })
        .collect();
    for id in &targets {
        if let Data::Compare { ops } = &mut tree.node_mut(*id).data {
            for op in ops.iter_mut().skip(1) {
                *op = comparison_flip(*op);
            }
        }
    }
    !targets.is_empty()
}

// seed-60, swapped dictionary entry values.

fn apply_swapped_dictionary_entry_values(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(&tree.nodes[*id].data, Data::Dict)
                && tree
                    .maybe(*id, "keys")
                    .iter()
                    .filter(|key| key.is_some())
                    .count()
                    >= 2
        })
        .collect();
    for id in &targets {
        let keys = tree.maybe(*id, "keys").to_vec();
        let keyed: Vec<usize> = keys
            .iter()
            .enumerate()
            .filter(|(_, key)| key.is_some())
            .map(|(at, _)| at)
            .collect();
        let mut values = tree.many(*id, "values").to_vec();
        values.swap(keyed[0], keyed[1]);
        tree.set_many(*id, "values", values);
    }
    !targets.is_empty()
}

// seed-61, swapped sequence literal elements.

fn apply_swapped_sequence_literal_elements(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(
                &tree.nodes[*id].data,
                Data::List { .. } | Data::Tuple { .. }
            ) && tree.many(*id, "elts").len() >= 2
        })
        .collect();
    for id in &targets {
        let mut elts = tree.many(*id, "elts").to_vec();
        elts.swap(0, 1);
        tree.set_many(*id, "elts", elts);
    }
    !targets.is_empty()
}

// seed-62, dropped assert statement.

fn apply_dropped_assert_statement(tree: &mut Tree) -> bool {
    let hits = match collect_with_parents(tree, &|tree, id| {
        matches!(&tree.nodes[id].data, Data::Assert)
    }) {
        Ok(hits) => hits,
        Err(_) => return false,
    };
    let mut planted = false;
    let mut hits = hits;
    hits.sort_by(|a, b| {
        b.depth
            .cmp(&a.depth)
            .then(b.parent.cmp(&a.parent))
            .then(b.index.cmp(&a.index))
    });
    for hit in hits {
        let Some(index) = hit.index else {
            continue;
        };
        let siblings = tree.many(hit.parent, hit.field).to_vec();
        if siblings.len() <= 1 && !matches!(&tree.nodes[hit.parent].data, Data::Module) {
            continue;
        }
        if siblings.get(index) != Some(&hit.node) {
            continue;
        }
        let mut rest = siblings;
        rest.remove(index);
        tree.set_many(hit.parent, hit.field, rest);
        planted = true;
    }
    planted
}

// seed-63, added numeric negation.

fn apply_added_numeric_negation(tree: &mut Tree) -> Result<bool, Refusal> {
    let hits = collect_with_parents(tree, &|tree, id| {
        let numeric = match &tree.nodes[id].data {
            Data::Constant { value } => {
                matches!(value, ConstVal::Int(_))
                    || matches!(value, ConstVal::Float(value) if value.is_finite())
            }
            _ => false,
        };
        numeric && !is_zero_literal(tree, id)
    })?;
    let mut planted = false;
    for hit in hits {
        let negated = matches!(
            &tree.nodes[hit.parent].data,
            Data::UnaryOp {
                op: UnaryKind::USub
            }
        ) && hit.field == "operand";
        if negated {
            continue;
        }
        if *tree.slot_mut(hit.parent, hit.field, hit.index) != hit.node {
            continue;
        }
        let wrapped = fresh(
            tree,
            Data::UnaryOp {
                op: UnaryKind::USub,
            },
            vec![one(hit.node)],
        );
        *tree.slot_mut(hit.parent, hit.field, hit.index) = wrapped;
        planted = true;
    }
    Ok(planted)
}

// seed-64, dropped with target.

fn apply_dropped_with_target(tree: &mut Tree) -> bool {
    let hits = match collect_with_parents(tree, &|tree, id| {
        matches!(&tree.nodes[id].data, Data::WithItem) && tree.one(id, "optional_vars").is_some()
    }) {
        Ok(hits) => hits,
        Err(_) => return false,
    };
    let planted = !hits.is_empty();
    for hit in hits {
        tree.set_one(hit.node, "optional_vars", None);
    }
    planted
}

// seed-65, dropped assignment expression.

fn apply_dropped_assignment_expression(tree: &mut Tree) -> Result<bool, Refusal> {
    let hits = collect_with_parents(tree, &|tree, id| {
        matches!(&tree.nodes[id].data, Data::NamedExpr)
    })?;
    Ok(replace_hits_with_child(tree, hits, "value"))
}

// seed-66, dropped delete statement.

fn apply_dropped_delete_statement(tree: &mut Tree) -> bool {
    let hits = match collect_with_parents(tree, &|tree, id| {
        matches!(&tree.nodes[id].data, Data::Delete)
    }) {
        Ok(hits) => hits,
        Err(_) => return false,
    };
    let mut planted = false;
    let mut hits = hits;
    hits.sort_by(|a, b| {
        b.depth
            .cmp(&a.depth)
            .then(b.parent.cmp(&a.parent))
            .then(b.index.cmp(&a.index))
    });
    for hit in hits {
        let Some(index) = hit.index else {
            continue;
        };
        let siblings = tree.many(hit.parent, hit.field).to_vec();
        if siblings.len() <= 1 && !matches!(&tree.nodes[hit.parent].data, Data::Module) {
            continue;
        }
        if siblings.get(index) != Some(&hit.node) {
            continue;
        }
        let mut rest = siblings;
        rest.remove(index);
        tree.set_many(hit.parent, hit.field, rest);
        planted = true;
    }
    planted
}

// seed-67, dropped match case.

fn apply_dropped_match_case(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(&tree.nodes[*id].data, Data::Match) && tree.many(*id, "cases").len() >= 2
        })
        .collect();
    for id in &targets {
        let rest = tree.many(*id, "cases")[1..].to_vec();
        tree.set_many(*id, "cases", rest);
    }
    !targets.is_empty()
}

// seed-68, swapped slice bounds.

fn apply_swapped_slice_bounds(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(&tree.nodes[*id].data, Data::Slice)
                && tree.one(*id, "lower").is_some()
                && tree.one(*id, "upper").is_some()
        })
        .collect();
    for id in &targets {
        let lower = tree.one(*id, "lower").expect("a lower bound stands");
        let upper = tree.one(*id, "upper").expect("an upper bound stands");
        tree.set_one(*id, "lower", Some(upper));
        tree.set_one(*id, "upper", Some(lower));
    }
    !targets.is_empty()
}

// seed-69, swapped branch bodies.

fn apply_swapped_branch_bodies(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(&tree.nodes[*id].data, Data::If)
                && !tree.many(*id, "body").is_empty()
                && !tree.many(*id, "orelse").is_empty()
        })
        .collect();
    for id in &targets {
        let body = tree.many(*id, "body").to_vec();
        let orelse = tree.many(*id, "orelse").to_vec();
        tree.set_many(*id, "body", orelse);
        tree.set_many(*id, "orelse", body);
    }
    !targets.is_empty()
}

// seed-70, dropped match guard.

fn apply_dropped_match_guard(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| matches!(&tree.nodes[*id].data, Data::Match))
        .flat_map(|id| tree.many(id, "cases").to_vec())
        .filter(|case| tree.one(*case, "guard").is_some())
        .collect();
    for id in &targets {
        tree.set_one(*id, "guard", None);
    }
    !targets.is_empty()
}

// seed-71, dropped yield-from delegation.

fn apply_dropped_yield_from_delegation(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| matches!(&tree.nodes[*id].data, Data::YieldFrom))
        .collect();
    for id in &targets {
        tree.node_mut(*id).data = Data::Yield;
    }
    !targets.is_empty()
}

/// The names standing anywhere under a node, each name
/// once, the reading the comprehension generator order
/// seed makes of a target and of an iterable.
fn subtree_names(tree: &Tree, root: usize) -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    let mut stack = vec![root];
    while let Some(id) = stack.pop() {
        if let Data::Name { id: name, .. } = &tree.nodes[id].data {
            if !names.contains(name) {
                names.push(name.clone());
            }
        }
        for child in tree.child_ids(id) {
            stack.push(child);
        }
    }
    names
}

// seed-72, swapped comprehension generator order.

fn apply_swapped_comprehension_generator_order(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(
                &tree.nodes[*id].data,
                Data::ListComp | Data::SetComp | Data::GeneratorExp | Data::DictComp
            ) && tree.many(*id, "generators").len() >= 2
        })
        .collect();
    let mut planted = false;
    for id in &targets {
        let generators = tree.many(*id, "generators").to_vec();
        let target = tree
            .one(generators[0], "target")
            .expect("a generator target stands");
        let iter = tree
            .one(generators[1], "iter")
            .expect("a generator iterable stands");
        let bound = subtree_names(tree, target);
        let loaded = subtree_names(tree, iter);
        if loaded.iter().any(|name| bound.contains(name)) {
            continue;
        }
        let mut swapped = generators;
        swapped.swap(0, 1);
        tree.set_many(*id, "generators", swapped);
        planted = true;
    }
    planted
}

// seed-73, dropped argument unpacking.

fn apply_dropped_argument_unpacking(tree: &mut Tree) -> Result<bool, Refusal> {
    let hits = collect_with_parents(tree, &|tree, id| {
        matches!(&tree.nodes[id].data, Data::Starred { .. })
    })?;
    let hits: Vec<Hit> = hits
        .into_iter()
        .filter(|hit| hit.field == "args" && matches!(&tree.nodes[hit.parent].data, Data::Call))
        .collect();
    Ok(replace_hits_with_child(tree, hits, "value"))
}

// seed-74, swapped formatted string values.

fn apply_swapped_formatted_string_values(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| matches!(&tree.nodes[*id].data, Data::JoinedStr))
        .collect();
    let mut planted = false;
    for id in &targets {
        let positions: Vec<usize> = tree
            .many(*id, "values")
            .iter()
            .enumerate()
            .filter(|(_, value)| matches!(&tree.nodes[**value].data, Data::FormattedValue { .. }))
            .map(|(index, _)| index)
            .collect();
        if positions.len() >= 2 {
            let mut values = tree.many(*id, "values").to_vec();
            values.swap(positions[0], positions[1]);
            tree.set_many(*id, "values", values);
            planted = true;
        }
    }
    planted
}

// seed-75, dropped call statement.

fn apply_dropped_call_statement(tree: &mut Tree) -> bool {
    let hits = match collect_with_parents(tree, &|tree, id| {
        matches!(&tree.nodes[id].data, Data::ExprStmt)
            && match tree.one(id, "value") {
                Some(value) => matches!(&tree.nodes[value].data, Data::Call),
                None => false,
            }
    }) {
        Ok(hits) => hits,
        Err(_) => return false,
    };
    let mut planted = false;
    let mut hits = hits;
    hits.sort_by(|a, b| {
        b.depth
            .cmp(&a.depth)
            .then(b.parent.cmp(&a.parent))
            .then(b.index.cmp(&a.index))
    });
    for hit in hits {
        let Some(index) = hit.index else {
            continue;
        };
        let siblings = tree.many(hit.parent, hit.field).to_vec();
        if siblings.len() <= 1 && !matches!(&tree.nodes[hit.parent].data, Data::Module) {
            continue;
        }
        if siblings.get(index) != Some(&hit.node) {
            continue;
        }
        let mut rest = siblings;
        rest.remove(index);
        tree.set_many(hit.parent, hit.field, rest);
        planted = true;
    }
    planted
}

// seed-76, swapped decorator order.

fn apply_swapped_decorator_order(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(
                &tree.nodes[*id].data,
                Data::FuncDef { .. } | Data::ClassDef { .. }
            ) && tree.many(*id, "decorator_list").len() >= 2
        })
        .collect();
    for id in &targets {
        let mut swapped = tree.many(*id, "decorator_list").to_vec();
        swapped.swap(0, 1);
        tree.set_many(*id, "decorator_list", swapped);
    }
    !targets.is_empty()
}

// seed-77, dropped parameter default.

fn apply_dropped_parameter_default(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| matches!(&tree.nodes[*id].data, Data::FuncDef { .. } | Data::Lambda))
        .filter_map(|id| tree.one(id, "args").map(|args| (id, args)))
        .filter(|(_, args)| !tree.many(*args, "defaults").is_empty())
        .map(|(_, args)| args)
        .collect();
    for args in &targets {
        let rest = tree.many(*args, "defaults")[1..].to_vec();
        tree.set_many(*args, "defaults", rest);
    }
    !targets.is_empty()
}

// seed-78, swapped base class order.

fn apply_swapped_base_class_order(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(&tree.nodes[*id].data, Data::ClassDef { .. })
                && tree.many(*id, "bases").len() >= 2
        })
        .collect();
    for id in &targets {
        let mut swapped = tree.many(*id, "bases").to_vec();
        swapped.swap(0, 1);
        tree.set_many(*id, "bases", swapped);
    }
    !targets.is_empty()
}

// seed-79, swapped match case order.

fn apply_swapped_match_case_order(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(&tree.nodes[*id].data, Data::Match) && tree.many(*id, "cases").len() >= 2
        })
        .collect();
    for id in &targets {
        let mut swapped = tree.many(*id, "cases").to_vec();
        swapped.swap(0, 1);
        tree.set_many(*id, "cases", swapped);
    }
    !targets.is_empty()
}

// seed-80, swapped with items.

fn apply_swapped_with_items(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(&tree.nodes[*id].data, Data::With { .. }) && tree.many(*id, "items").len() >= 2
        })
        .collect();
    for id in &targets {
        let mut swapped = tree.many(*id, "items").to_vec();
        swapped.swap(0, 1);
        tree.set_many(*id, "items", swapped);
    }
    !targets.is_empty()
}

// seed-81, dropped asynchronous modifier. The confinement
// reading of F10-3, a definition plants only where its own
// body carries no await and no asynchronous compound of its
// own, the headers of nested definitions standing in the
// reading and their bodies standing outside it.

fn async_body_stands_plain(tree: &Tree, def: usize) -> bool {
    let mut stack: Vec<usize> = tree.many(def, "body").to_vec();
    while let Some(id) = stack.pop() {
        match &tree.nodes[id].data {
            Data::Await => return false,
            Data::For { is_async: true }
            | Data::With { is_async: true }
            | Data::Comprehension { is_async: true } => return false,
            Data::FuncDef { .. } => {
                let body = tree.many(id, "body").to_vec();
                for child in tree.child_ids(id) {
                    if !body.contains(&child) {
                        stack.push(child);
                    }
                }
                continue;
            }
            _ => {}
        }
        for child in tree.child_ids(id) {
            stack.push(child);
        }
    }
    true
}

fn apply_dropped_asynchronous_modifier(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(&tree.nodes[*id].data, Data::FuncDef { is_async: true, .. })
                && async_body_stands_plain(tree, *id)
        })
        .collect();
    for id in &targets {
        if let Data::FuncDef { is_async, .. } = &mut tree.node_mut(*id).data {
            *is_async = false;
        }
    }
    !targets.is_empty()
}

// seed-82, swapped keyword argument order.

fn apply_swapped_keyword_argument_order(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| matches!(&tree.nodes[*id].data, Data::Call))
        .collect();
    let mut planted = false;
    for id in &targets {
        let keywords = tree.many(*id, "keywords").to_vec();
        let named: Vec<usize> = keywords
            .iter()
            .enumerate()
            .filter(|(_, kw)| matches!(&tree.nodes[**kw].data, Data::Keyword { arg: Some(_) }))
            .map(|(index, _)| index)
            .collect();
        if named.len() < 2 {
            continue;
        }
        let mut swapped = keywords;
        swapped.swap(named[0], named[1]);
        tree.set_many(*id, "keywords", swapped);
        planted = true;
    }
    planted
}

// seed-83, swapped import names.

fn apply_swapped_import_names(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(
                &tree.nodes[*id].data,
                Data::Import | Data::ImportFrom { .. }
            ) && tree.many(*id, "names").len() >= 2
        })
        .collect();
    for id in &targets {
        let mut swapped = tree.many(*id, "names").to_vec();
        swapped.swap(0, 1);
        tree.set_many(*id, "names", swapped);
    }
    !targets.is_empty()
}

// seed-84, dropped with item. The first item of a with
// carrying at least two leaves the statement whole, the
// removal the registry's first entry removals make, so the
// statement always keeps an item the language reads.

fn apply_dropped_with_item(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(&tree.nodes[*id].data, Data::With { .. }) && tree.many(*id, "items").len() >= 2
        })
        .collect();
    for id in &targets {
        let rest = tree.many(*id, "items")[1..].to_vec();
        tree.set_many(*id, "items", rest);
    }
    !targets.is_empty()
}

// seed-85, swapped try handler order. The confinement of
// F11-3, a try plants only where both of the first two
// handlers carry a type, so a bare handler never moves out
// of the last place it stands in every parseable document.

fn apply_swapped_try_handler_order(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            if !matches!(&tree.nodes[*id].data, Data::Try { .. }) {
                return false;
            }
            let handlers = tree.many(*id, "handlers");
            handlers.len() >= 2
                && tree.one(handlers[0], "type").is_some()
                && tree.one(handlers[1], "type").is_some()
        })
        .collect();
    for id in &targets {
        let mut swapped = tree.many(*id, "handlers").to_vec();
        swapped.swap(0, 1);
        tree.set_many(*id, "handlers", swapped);
    }
    !targets.is_empty()
}

// seed-86, dropped argument annotation. The annotation
// child of an argument clears the way seed-64 clears a
// with item's target, the argument and its definition
// standing exactly as written.

fn apply_dropped_argument_annotation(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(&tree.nodes[*id].data, Data::Arg { .. })
                && tree.one(*id, "annotation").is_some()
        })
        .collect();
    for id in &targets {
        tree.set_one(*id, "annotation", None);
    }
    !targets.is_empty()
}

// seed-87, swapped assignment target order. Only a target
// standing as a sequence plants, its first two elements
// exchanging whole, the chained name targets the tenth
// slice's weighing set aside standing untouched.

fn apply_swapped_assignment_target_order(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| matches!(&tree.nodes[*id].data, Data::Assign))
        .collect();
    let mut planted = false;
    for id in &targets {
        for target in tree.many(*id, "targets").to_vec() {
            if !matches!(
                &tree.nodes[target].data,
                Data::Tuple { .. } | Data::List { .. }
            ) {
                continue;
            }
            let elts = tree.many(target, "elts");
            if elts.len() < 2 {
                continue;
            }
            let mut swapped = elts.to_vec();
            swapped.swap(0, 1);
            tree.set_many(target, "elts", swapped);
            planted = true;
        }
    }
    planted
}

// seed-88, dropped return annotation. The returns child of
// a definition clears the way seed-86 clears an argument's
// annotation, the definition's name and arguments and body
// standing exactly as written.

fn apply_dropped_return_annotation(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(&tree.nodes[*id].data, Data::FuncDef { .. })
                && tree.one(*id, "returns").is_some()
        })
        .collect();
    for id in &targets {
        tree.set_one(*id, "returns", None);
    }
    !targets.is_empty()
}

// seed-89, swapped statement order. Every body the tree
// carries under the name body plants, its first two
// statements exchanging whole, and a nested body stands
// judged at its own turn and never travels with a
// statement of the body around it. The module's own body
// stands read at the root, which the sorted walk passes
// over because the module carries no position. At the
// module body the exchange does not stand where it would
// carry a future import past the leading run, the boundary
// the feature slice 12 correction states.

fn is_future_import(tree: &Tree, id: usize) -> bool {
    matches!(
        tree.data(id),
        Data::ImportFrom {
            module: Some(name),
            level: 0
        } if name == "__future__"
    )
}

fn is_docstring(tree: &Tree, id: usize) -> bool {
    if !matches!(tree.data(id), Data::ExprStmt) {
        return false;
    }
    match tree.one(id, "value") {
        Some(value) => matches!(
            tree.data(value),
            Data::Constant {
                value: ConstVal::Str(..)
            }
        ),
        None => false,
    }
}

fn apply_swapped_statement_order(tree: &mut Tree) -> bool {
    let mut planted = false;
    if tree.many(tree.root, "body").len() >= 2 {
        let body = tree.many(tree.root, "body");
        let blocked = is_future_import(tree, body[0])
            && !is_future_import(tree, body[1])
            && !is_docstring(tree, body[1]);
        if !blocked {
            let mut swapped = body.to_vec();
            swapped.swap(0, 1);
            tree.set_many(tree.root, "body", swapped);
            planted = true;
        }
    }
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| tree.many(*id, "body").len() >= 2)
        .collect();
    for id in &targets {
        let mut swapped = tree.many(*id, "body").to_vec();
        swapped.swap(0, 1);
        tree.set_many(*id, "body", swapped);
        planted = true;
    }
    planted
}

// seed-90, swapped comprehension condition order. The
// first two conditions standing inside one generator
// clause exchange, the clause's target and iterable and
// its remaining conditions standing exactly as written,
// where seed-58 removes a clause's first condition and
// seed-72 exchanges two clauses whole.

fn apply_swapped_comprehension_condition_order(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(
                &tree.nodes[*id].data,
                Data::ListComp | Data::SetComp | Data::GeneratorExp | Data::DictComp
            )
        })
        .collect();
    let mut planted = false;
    for id in &targets {
        for clause in tree.many(*id, "generators").to_vec() {
            if tree.many(clause, "ifs").len() >= 2 {
                let mut swapped = tree.many(clause, "ifs").to_vec();
                swapped.swap(0, 1);
                tree.set_many(clause, "ifs", swapped);
                planted = true;
            }
        }
    }
    planted
}

// seed-91, dropped starred argument. The first starred
// argument of a call stands removed whole, the call's
// remaining arguments keeping their places, where seed-73
// leaves the argument standing as the value it unpacks.

fn apply_dropped_starred_argument(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| matches!(&tree.nodes[*id].data, Data::Call))
        .collect();
    let mut planted = false;
    for id in &targets {
        let args = tree.many(*id, "args").to_vec();
        if let Some(at) = args
            .iter()
            .position(|arg| matches!(&tree.nodes[*arg].data, Data::Starred { .. }))
        {
            let mut kept = args;
            kept.remove(at);
            tree.set_many(*id, "args", kept);
            planted = true;
        }
    }
    planted
}

// seed-92, dropped raise statement. The statement stands
// removed whole from the body that holds it, a raise
// standing as the only statement of a body other than the
// module's own never planted, on the standing of F3-5,
// where seed-28 exchanges the error a raise carries and
// seed-51 clears its cause and the statement stands in
// both.

fn apply_dropped_raise_statement(tree: &mut Tree) -> bool {
    let hits = match collect_with_parents(tree, &|tree, id| {
        matches!(&tree.nodes[id].data, Data::Raise { .. })
    }) {
        Ok(hits) => hits,
        Err(_) => return false,
    };
    remove_statement_hits(tree, hits)
}

/// Remove the statement hits a parent carrying collector
/// gathered, deepest first and latest index first inside
/// one parent, each hit standing verified against its
/// parent's list before it leaves, a hit standing as the
/// only statement of a body other than the module's own
/// never removed, on the standing of F3-5.
fn remove_statement_hits(tree: &mut Tree, hits: Vec<Hit>) -> bool {
    let mut planted = false;
    let mut hits = hits;
    hits.sort_by(|a, b| {
        b.depth
            .cmp(&a.depth)
            .then(b.parent.cmp(&a.parent))
            .then(b.index.cmp(&a.index))
    });
    for hit in hits {
        let Some(index) = hit.index else {
            continue;
        };
        let siblings = tree.many(hit.parent, hit.field).to_vec();
        if siblings.len() <= 1 && !matches!(&tree.nodes[hit.parent].data, Data::Module) {
            continue;
        }
        if siblings.get(index) != Some(&hit.node) {
            continue;
        }
        let mut rest = siblings;
        rest.remove(index);
        tree.set_many(hit.parent, hit.field, rest);
        planted = true;
    }
    planted
}

// seed-93, dropped starred display element. The first
// starred element of a list or tuple or set display
// stands removed whole, the display's remaining elements
// keeping their places, where seed-09 removes the first
// element of a list or a dictionary reading no starred
// standing and seed-91 removes a starred element standing
// in a call's arguments.

fn apply_dropped_starred_display_element(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(
                &tree.nodes[*id].data,
                Data::List { .. } | Data::Tuple { .. } | Data::Set
            )
        })
        .collect();
    let mut planted = false;
    for id in &targets {
        let elts = tree.many(*id, "elts").to_vec();
        if let Some(at) = elts
            .iter()
            .position(|elt| matches!(&tree.nodes[*elt].data, Data::Starred { .. }))
        {
            let mut kept = elts;
            kept.remove(at);
            tree.set_many(*id, "elts", kept);
            planted = true;
        }
    }
    planted
}

// seed-94, dropped loop control statement. Every break
// and every continue standing in the body of a loop
// stands removed whole under the loop body reading the
// loop control collectors keep, a control standing as
// the only statement of a body never planted, on the
// standing of F3-5, where seed-23 turns a break into a
// continue and seed-41 turns a continue into a break and
// neither removes the statement it reads.

fn apply_dropped_loop_control_statement(tree: &mut Tree) -> Result<bool, Refusal> {
    let mut hits = collect_loop_control(tree, true)?;
    hits.extend(collect_loop_control(tree, false)?);
    Ok(remove_statement_hits(tree, hits))
}

// seed-95, swapped try else bodies. The body of a try
// statement and the body of its else clause exchange
// whole where both stand written and nonempty, the
// handlers and the finally clause standing exactly as
// written, where seed-69 exchanges the two bodies of an
// if statement and seed-48 removes a try statement's
// else clause whole.

fn apply_swapped_try_else_bodies(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(&tree.nodes[*id].data, Data::Try { .. })
                && !tree.many(*id, "body").is_empty()
                && !tree.many(*id, "orelse").is_empty()
        })
        .collect();
    for id in &targets {
        let body = tree.many(*id, "body").to_vec();
        let orelse = tree.many(*id, "orelse").to_vec();
        tree.set_many(*id, "body", orelse);
        tree.set_many(*id, "orelse", body);
    }
    !targets.is_empty()
}

// seed-96, dropped class keyword argument. The first
// keyword carrying a name in a class definition's header
// stands removed whole, where seed-45 removes the first
// named keyword argument of a call and seed-39 removes the
// first base of the same header, and a keyword carrying no
// name stands outside the family in both readings.

fn apply_dropped_class_keyword_argument(tree: &mut Tree) -> bool {
    let mut targets: Vec<(usize, usize)> = Vec::new();
    for id in tree.sorted_nodes() {
        if !matches!(&tree.nodes[id].data, Data::ClassDef { .. }) {
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

// seed-97, swapped loop else bodies. The body of a loop
// and the body of its else clause stand exchanged whole
// where both stand written, where seed-95 exchanges the
// two bodies of a try statement and seed-44 removes a
// loop's else clause whole. A loop stands outside the
// family where a break or a continue standing in its
// body binds to the loop itself, because the exchange
// would carry the control into the else clause where the
// language refuses it, and where a break or a continue
// standing in its else clause binds to a loop around it,
// because the exchange would bind it to this loop
// instead, the binding read the way the loop control
// reading of seed-94 reads it.

fn loop_control_binds_here(tree: &Tree, id: usize, depth: usize) -> bool {
    match &tree.nodes[id].data {
        Data::Break | Data::Continue => depth == 0,
        Data::For { .. } | Data::While => {
            tree.many(id, "body")
                .iter()
                .any(|stmt| loop_control_binds_here(tree, *stmt, depth + 1))
                || tree
                    .many(id, "orelse")
                    .iter()
                    .any(|stmt| loop_control_binds_here(tree, *stmt, depth))
        }
        Data::FuncDef { .. } | Data::ClassDef { .. } | Data::Lambda => false,
        _ => tree
            .child_ids(id)
            .iter()
            .any(|child| loop_control_binds_here(tree, *child, depth)),
    }
}

fn apply_swapped_loop_else_bodies(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(&tree.nodes[*id].data, Data::For { .. } | Data::While)
                && !tree.many(*id, "body").is_empty()
                && !tree.many(*id, "orelse").is_empty()
                && !tree
                    .many(*id, "body")
                    .iter()
                    .any(|stmt| loop_control_binds_here(tree, *stmt, 0))
                && !tree
                    .many(*id, "orelse")
                    .iter()
                    .any(|stmt| loop_control_binds_here(tree, *stmt, 0))
        })
        .collect();
    for id in &targets {
        let body = tree.many(*id, "body").to_vec();
        let orelse = tree.many(*id, "orelse").to_vec();
        tree.set_many(*id, "body", orelse);
        tree.set_many(*id, "orelse", body);
    }
    !targets.is_empty()
}

// seed-98, dropped starred parameter. The variable
// positional parameter of a definition stands removed
// where it stands and its variable keyword parameter
// stands removed where it stands, a definition carrying
// both losing both, where seed-77 drops the first
// positional default and seed-86 drops an argument's
// annotation and neither removes a parameter whole.

fn apply_dropped_starred_parameter(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| matches!(&tree.nodes[*id].data, Data::FuncDef { .. } | Data::Lambda))
        .filter_map(|id| tree.one(id, "args"))
        .filter(|args| tree.one(*args, "vararg").is_some() || tree.one(*args, "kwarg").is_some())
        .collect();
    for args in &targets {
        if tree.one(*args, "vararg").is_some() {
            tree.set_one(*args, "vararg", None);
        }
        if tree.one(*args, "kwarg").is_some() {
            tree.set_one(*args, "kwarg", None);
        }
    }
    !targets.is_empty()
}

// seed-99, dropped docstring. The string statement
// standing first in the body of a module or a definition
// or a class stands removed whole, the reading of that
// shape standing in the crate already where the swapped
// statement order keeps a docstring standing first, and a
// statement standing as the only statement of a body
// other than the module's own is never removed, on the
// standing of F3-5.

fn apply_dropped_docstring(tree: &mut Tree) -> bool {
    let hits = match collect_with_parents(tree, &|tree, id| is_docstring(tree, id)) {
        Ok(hits) => hits,
        Err(_) => return false,
    };
    let hits: Vec<Hit> = hits
        .into_iter()
        .filter(|hit| {
            hit.index == Some(0)
                && matches!(
                    &tree.nodes[hit.parent].data,
                    Data::Module | Data::FuncDef { .. } | Data::ClassDef { .. }
                )
        })
        .collect();
    remove_statement_hits(tree, hits)
}
// seed-112, dropped slice step.

fn apply_dropped_slice_step(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(&tree.nodes[*id].data, Data::Slice) && tree.one(*id, "step").is_some()
        })
        .collect();
    for id in &targets {
        tree.set_one(*id, "step", None);
    }
    !targets.is_empty()
}

// seed-113, dropped annotation value.

fn apply_dropped_annotation_value(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(&tree.nodes[*id].data, Data::AnnAssign { .. })
                && tree.one(*id, "value").is_some()
        })
        .collect();
    for id in &targets {
        tree.set_one(*id, "value", None);
    }
    !targets.is_empty()
}

// seed-114, dropped formatted value.

fn apply_dropped_formatted_value(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| matches!(&tree.nodes[*id].data, Data::JoinedStr))
        .collect();
    let mut planted = false;
    for id in &targets {
        let positions: Vec<usize> = tree
            .many(*id, "values")
            .iter()
            .enumerate()
            .filter(|(_, value)| matches!(&tree.nodes[**value].data, Data::FormattedValue { .. }))
            .map(|(index, _)| index)
            .collect();
        if positions.len() >= 2 {
            let mut values = tree.many(*id, "values").to_vec();
            values.remove(*positions.last().expect("a position stands"));
            tree.set_many(*id, "values", values);
            planted = true;
        }
    }
    planted
}

// seed-115, dropped raised error message.

fn apply_dropped_raised_error_message(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter_map(|id| {
            if !matches!(&tree.nodes[id].data, Data::Raise) {
                return None;
            }
            let exc = tree.one(id, "exc")?;
            if matches!(&tree.nodes[exc].data, Data::Call) && !tree.many(exc, "args").is_empty() {
                Some(exc)
            } else {
                None
            }
        })
        .collect();
    for exc in &targets {
        tree.set_many(*exc, "args", Vec::new());
    }
    !targets.is_empty()
}
// seed-116, dropped set element.

fn apply_dropped_set_element(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(&tree.nodes[*id].data, Data::Set) && tree.many(*id, "elts").len() >= 2
        })
        .collect();
    for id in &targets {
        let elts = tree.many(*id, "elts")[1..].to_vec();
        tree.set_many(*id, "elts", elts);
    }
    !targets.is_empty()
}

// seed-117, dropped decorator call.

fn apply_dropped_decorator_call(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(
                &tree.nodes[*id].data,
                Data::FuncDef { .. } | Data::ClassDef { .. }
            )
        })
        .collect();
    let mut planted = false;
    for id in &targets {
        let decorators = tree.many(*id, "decorator_list").to_vec();
        let mut kept = Vec::new();
        for decorator in decorators {
            if matches!(&tree.nodes[decorator].data, Data::Call) {
                let func = tree
                    .one(decorator, "func")
                    .expect("the called function stands");
                kept.push(func);
                planted = true;
            } else {
                kept.push(decorator);
            }
        }
        tree.set_many(*id, "decorator_list", kept);
    }
    planted
}

// seed-118, dropped attribute.

fn apply_dropped_attribute(tree: &mut Tree) -> Result<bool, Refusal> {
    let hits = collect_with_parents(tree, &|tree, id| {
        matches!(&tree.nodes[id].data, Data::Attribute { .. })
    })?;
    Ok(replace_hits_with_child(tree, hits, "value"))
}

// seed-119, dropped exception type.

fn apply_dropped_exception_type(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(&tree.nodes[*id].data, Data::ExceptHandler { .. })
                && tree.one(*id, "type").is_some()
        })
        .collect();
    for id in &targets {
        tree.set_one(*id, "type", None);
    }
    !targets.is_empty()
}
// seed-120, dropped subscript.

fn apply_dropped_subscript(tree: &mut Tree) -> Result<bool, Refusal> {
    let hits = collect_with_parents(tree, &|tree, id| {
        matches!(&tree.nodes[id].data, Data::Subscript { .. })
    })?;
    Ok(replace_hits_with_child(tree, hits, "value"))
}

// seed-121, dropped arithmetic operand.

fn apply_dropped_arithmetic_operand(tree: &mut Tree) -> Result<bool, Refusal> {
    let hits = collect_with_parents(tree, &|tree, id| {
        matches!(&tree.nodes[id].data, Data::BinOp { .. })
    })?;
    Ok(replace_hits_with_child(tree, hits, "left"))
}

// seed-122, dropped first call argument.

fn apply_dropped_first_call_argument(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(&tree.nodes[*id].data, Data::Call) && tree.many(*id, "args").len() >= 2
        })
        .collect();
    for id in &targets {
        let args = tree.many(*id, "args")[1..].to_vec();
        tree.set_many(*id, "args", args);
    }
    !targets.is_empty()
}

// seed-123, dropped first condition term.

fn apply_dropped_first_condition_term(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(&tree.nodes[*id].data, Data::BoolOp { .. })
                && tree.many(*id, "values").len() >= 2
        })
        .collect();
    for id in &targets {
        let values = tree.many(*id, "values")[1..].to_vec();
        tree.set_many(*id, "values", values);
    }
    !targets.is_empty()
}

// seed-124, dropped tuple element.

fn apply_dropped_tuple_element(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(&tree.nodes[*id].data, Data::Tuple { ctx: Ctx::Load, .. })
                && tree.many(*id, "elts").len() >= 2
        })
        .collect();
    for id in &targets {
        let elts = tree.many(*id, "elts")[1..].to_vec();
        tree.set_many(*id, "elts", elts);
    }
    !targets.is_empty()
}

// seed-125, dropped conditional test.

fn apply_dropped_conditional_test(tree: &mut Tree) -> Result<bool, Refusal> {
    let hits = collect_with_parents(tree, &|tree, id| {
        matches!(&tree.nodes[id].data, Data::IfExp)
    })?;
    Ok(replace_hits_with_child(tree, hits, "body"))
}

// seed-126, dropped comparison head.

fn apply_dropped_comparison_head(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| match &tree.nodes[*id].data {
            Data::Compare { ops } => ops.len() >= 2,
            _ => false,
        })
        .collect();
    for id in &targets {
        let comparators = tree.many(*id, "comparators").to_vec();
        if let Data::Compare { ops } = &mut tree.node_mut(*id).data {
            ops.remove(0);
        }
        tree.set_one(*id, "left", Some(comparators[0]));
        tree.set_many(*id, "comparators", comparators[1..].to_vec());
    }
    !targets.is_empty()
}

// seed-127, dropped assignment.

fn apply_dropped_assignment(tree: &mut Tree) -> bool {
    let hits = match collect_with_parents(tree, &|tree, id| {
        matches!(&tree.nodes[id].data, Data::Assign)
    }) {
        Ok(hits) => hits,
        Err(_) => return false,
    };
    let mut planted = false;
    // The removals run deepest first so a removal never moves
    // the standing of a hit still to come in the same body, and
    // hits in one list run from its end for the same reason.
    let mut hits = hits;
    hits.sort_by(|a, b| {
        b.depth
            .cmp(&a.depth)
            .then(b.parent.cmp(&a.parent))
            .then(b.index.cmp(&a.index))
    });
    for hit in hits {
        let Some(index) = hit.index else {
            continue;
        };
        let siblings = tree.many(hit.parent, hit.field).to_vec();
        if siblings.len() <= 1 && !matches!(&tree.nodes[hit.parent].data, Data::Module) {
            continue;
        }
        if siblings.get(index) != Some(&hit.node) {
            continue;
        }
        let mut rest = siblings;
        rest.remove(index);
        tree.set_many(hit.parent, hit.field, rest);
        planted = true;
    }
    planted
}

// seed-128, dropped annotated assignment.

fn apply_dropped_annotated_assignment(tree: &mut Tree) -> bool {
    let hits = match collect_with_parents(tree, &|tree, id| {
        matches!(&tree.nodes[id].data, Data::AnnAssign { .. })
    }) {
        Ok(hits) => hits,
        Err(_) => return false,
    };
    let mut planted = false;
    // The removals run deepest first so a removal never moves
    // the standing of a hit still to come in the same body, and
    // hits in one list run from its end for the same reason.
    let mut hits = hits;
    hits.sort_by(|a, b| {
        b.depth
            .cmp(&a.depth)
            .then(b.parent.cmp(&a.parent))
            .then(b.index.cmp(&a.index))
    });
    for hit in hits {
        let Some(index) = hit.index else {
            continue;
        };
        let siblings = tree.many(hit.parent, hit.field).to_vec();
        if siblings.len() <= 1 && !matches!(&tree.nodes[hit.parent].data, Data::Module) {
            continue;
        }
        if siblings.get(index) != Some(&hit.node) {
            continue;
        }
        let mut rest = siblings;
        rest.remove(index);
        tree.set_many(hit.parent, hit.field, rest);
        planted = true;
    }
    planted
}

// seed-129, dropped function definition.

fn apply_dropped_function_definition(tree: &mut Tree) -> bool {
    let hits = match collect_with_parents(tree, &|tree, id| {
        matches!(&tree.nodes[id].data, Data::FuncDef { .. })
    }) {
        Ok(hits) => hits,
        Err(_) => return false,
    };
    let mut planted = false;
    // The removals run deepest first so a removal never moves
    // the standing of a hit still to come in the same body, and
    // hits in one list run from its end for the same reason.
    let mut hits = hits;
    hits.sort_by(|a, b| {
        b.depth
            .cmp(&a.depth)
            .then(b.parent.cmp(&a.parent))
            .then(b.index.cmp(&a.index))
    });
    for hit in hits {
        let Some(index) = hit.index else {
            continue;
        };
        let siblings = tree.many(hit.parent, hit.field).to_vec();
        if siblings.len() <= 1 && !matches!(&tree.nodes[hit.parent].data, Data::Module) {
            continue;
        }
        if siblings.get(index) != Some(&hit.node) {
            continue;
        }
        let mut rest = siblings;
        rest.remove(index);
        tree.set_many(hit.parent, hit.field, rest);
        planted = true;
    }
    planted
}

// seed-130, dropped class definition.

fn apply_dropped_class_definition(tree: &mut Tree) -> bool {
    let hits = match collect_with_parents(tree, &|tree, id| {
        matches!(&tree.nodes[id].data, Data::ClassDef { .. })
    }) {
        Ok(hits) => hits,
        Err(_) => return false,
    };
    let mut planted = false;
    // The removals run deepest first so a removal never moves
    // the standing of a hit still to come in the same body, and
    // hits in one list run from its end for the same reason.
    let mut hits = hits;
    hits.sort_by(|a, b| {
        b.depth
            .cmp(&a.depth)
            .then(b.parent.cmp(&a.parent))
            .then(b.index.cmp(&a.index))
    });
    for hit in hits {
        let Some(index) = hit.index else {
            continue;
        };
        let siblings = tree.many(hit.parent, hit.field).to_vec();
        if siblings.len() <= 1 && !matches!(&tree.nodes[hit.parent].data, Data::Module) {
            continue;
        }
        if siblings.get(index) != Some(&hit.node) {
            continue;
        }
        let mut rest = siblings;
        rest.remove(index);
        tree.set_many(hit.parent, hit.field, rest);
        planted = true;
    }
    planted
}

// seed-131, dropped exception handler name.

fn apply_dropped_exception_handler_name(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| matches!(&tree.nodes[*id].data, Data::ExceptHandler { name: Some(_) }))
        .collect();
    for id in &targets {
        tree.nodes[*id].data = Data::ExceptHandler { name: None };
    }
    !targets.is_empty()
}

// seed-132, dropped assignment target.

fn apply_dropped_assignment_target(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(&tree.nodes[*id].data, Data::Assign) && tree.many(*id, "targets").len() >= 2
        })
        .collect();
    for id in &targets {
        let rest = tree.many(*id, "targets")[1..].to_vec();
        tree.set_many(*id, "targets", rest);
    }
    !targets.is_empty()
}

// seed-133, dropped last tuple element.

fn apply_dropped_last_tuple_element(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(&tree.nodes[*id].data, Data::Tuple { ctx: Ctx::Load, .. })
                && tree.many(*id, "elts").len() >= 2
        })
        .collect();
    for id in &targets {
        let mut elts = tree.many(*id, "elts").to_vec();
        elts.pop();
        tree.set_many(*id, "elts", elts);
    }
    !targets.is_empty()
}

// seed-134, dropped last base class.

fn apply_dropped_last_base_class(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(&tree.nodes[*id].data, Data::ClassDef { .. })
                && tree.many(*id, "bases").len() >= 2
        })
        .collect();
    for id in &targets {
        let mut bases = tree.many(*id, "bases").to_vec();
        bases.pop();
        tree.set_many(*id, "bases", bases);
    }
    !targets.is_empty()
}

// seed-135, dropped decorator argument.

fn apply_dropped_decorator_argument(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(
                &tree.nodes[*id].data,
                Data::FuncDef { .. } | Data::ClassDef { .. }
            )
        })
        .collect();
    let mut planted = false;
    for id in &targets {
        for decorator in tree.many(*id, "decorator_list").to_vec() {
            if matches!(&tree.nodes[decorator].data, Data::Call)
                && !tree.many(decorator, "args").is_empty()
            {
                let args = tree.many(decorator, "args")[1..].to_vec();
                tree.set_many(decorator, "args", args);
                planted = true;
            }
        }
    }
    planted
}

// seed-136, dropped unpacking target element. The first
// element of every tuple standing in Store context
// carrying at least two elements stands removed, the
// value the code unpacks first never bound, where
// seed-124 removes the first element of a tuple display
// in Load context and seed-133 removes its last.

fn apply_dropped_unpacking_target_element(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(
                &tree.nodes[*id].data,
                Data::Tuple {
                    ctx: Ctx::Store,
                    ..
                }
            ) && tree.many(*id, "elts").len() >= 2
        })
        .collect();
    for id in &targets {
        let elts = tree.many(*id, "elts")[1..].to_vec();
        tree.set_many(*id, "elts", elts);
    }
    !targets.is_empty()
}

// seed-137, dropped last unpacking target element. The
// last element of every tuple standing in Store context
// carrying at least two elements stands removed, the
// tail twin of seed-136 on the same ground, where
// seed-133 removes the last element of a tuple display
// in Load context.

fn apply_dropped_last_unpacking_target_element(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(
                &tree.nodes[*id].data,
                Data::Tuple {
                    ctx: Ctx::Store,
                    ..
                }
            ) && tree.many(*id, "elts").len() >= 2
        })
        .collect();
    for id in &targets {
        let mut elts = tree.many(*id, "elts").to_vec();
        elts.pop();
        tree.set_many(*id, "elts", elts);
    }
    !targets.is_empty()
}

// seed-138, dropped keyword unpacking. The keyword
// carrying no name, the double starred unpacking of a
// call, stands removed from every call carrying one,
// where seed-45 removes the first keyword carrying a
// name and seed-73 replaces a starred argument with the
// value it carries and removes nothing.

fn apply_dropped_keyword_unpacking(tree: &mut Tree) -> bool {
    let mut targets: Vec<(usize, usize)> = Vec::new();
    for id in tree.sorted_nodes() {
        if !matches!(&tree.nodes[id].data, Data::Call) {
            continue;
        }
        let unnamed = tree
            .many(id, "keywords")
            .iter()
            .copied()
            .find(|kw| matches!(&tree.nodes[*kw].data, Data::Keyword { arg: None }));
        if let Some(keyword) = unnamed {
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

// seed-139, dropped last dictionary entry. The last
// entry of every dictionary display carrying at least
// two entries stands removed, its key and value
// together, where seed-09 removes the first entry of
// every dictionary display and seed-17 exchanges two
// entries and removes none.

fn apply_dropped_last_dictionary_entry(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(&tree.nodes[*id].data, Data::Dict) && tree.many(*id, "values").len() >= 2
        })
        .collect();
    for id in &targets {
        let keys = tree.maybe(*id, "keys");
        let kept = keys.len() - 1;
        tree.set_maybe(*id, "keys", keys[..kept].to_vec());
        let values = tree.many(*id, "values");
        tree.set_many(*id, "values", values[..kept].to_vec());
    }
    !targets.is_empty()
}

// seed-140, dropped last keyword argument. The last
// keyword carrying a name stands removed from every
// call carrying at least two of them, where seed-45
// removes the first keyword carrying a name and
// seed-138 removes the keyword carrying no name whole.

fn apply_dropped_last_keyword_argument(tree: &mut Tree) -> bool {
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
            targets.push((id, *named.last().expect("two or more stand")));
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

// seed-141, dropped last list element. The last
// element of every list display standing where a
// value is read and carrying at least two stands
// removed, where seed-09 removes the first element
// of every list display and seed-133 removes the
// last element of a tuple display.

fn apply_dropped_last_list_element(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(&tree.nodes[*id].data, Data::List { ctx: Ctx::Load, .. })
                && tree.many(*id, "elts").len() >= 2
        })
        .collect();
    for id in &targets {
        let elts = tree.many(*id, "elts").to_vec();
        let kept = elts.len() - 1;
        tree.set_many(*id, "elts", elts[..kept].to_vec());
    }
    !targets.is_empty()
}

// seed-142, dropped first imported name. The first
// name of every from import carrying at least two
// stands removed, the head twin of seed-110, which
// removes the last name of the same shape.

fn apply_dropped_first_imported_name(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(&tree.nodes[*id].data, Data::ImportFrom { .. })
                && tree.many(*id, "names").len() >= 2
        })
        .collect();
    for id in &targets {
        let names = tree.many(*id, "names")[1..].to_vec();
        tree.set_many(*id, "names", names);
    }
    !targets.is_empty()
}

// seed-143, dropped first boolean operand. The
// first value of every boolean operation carrying
// at least three stands removed, where seed-38
// removes the last value of a boolean operation at
// any count of two or more.

fn apply_dropped_first_boolean_operand(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(&tree.nodes[*id].data, Data::BoolOp { .. })
                && tree.many(*id, "values").len() >= 3
        })
        .collect();
    for id in &targets {
        let values = tree.many(*id, "values")[1..].to_vec();
        tree.set_many(*id, "values", values);
    }
    !targets.is_empty()
}

// seed-144, dropped last call argument.

fn apply_dropped_last_call_argument(tree: &mut Tree) -> bool {
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

// seed-145, dropped last parameter.

fn apply_dropped_last_parameter(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            if !matches!(&tree.nodes[*id].data, Data::FuncDef { .. } | Data::Lambda) {
                return false;
            }
            let Some(args_node) = tree.one(*id, "args") else {
                return false;
            };
            tree.many(args_node, "posonlyargs").len() + tree.many(args_node, "args").len() >= 2
        })
        .collect();
    for id in &targets {
        let args_node = tree.one(*id, "args").expect("the arguments stand");
        let mut posonly = tree.many(args_node, "posonlyargs").to_vec();
        let mut ordinary = tree.many(args_node, "args").to_vec();
        let mut defaults = tree.many(args_node, "defaults").to_vec();
        if !defaults.is_empty() {
            defaults.pop();
            tree.set_many(args_node, "defaults", defaults);
        }
        if !ordinary.is_empty() {
            ordinary.pop();
            tree.set_many(args_node, "args", ordinary);
        } else {
            posonly.pop();
            tree.set_many(args_node, "posonlyargs", posonly);
        }
    }
    !targets.is_empty()
}

// seed-146, dropped subscript index element.

fn apply_dropped_subscript_index_element(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter_map(|id| {
            if matches!(&tree.nodes[id].data, Data::Subscript { .. }) {
                let slice = tree.one(id, "slice")?;
                if matches!(&tree.nodes[slice].data, Data::Tuple { .. })
                    && tree.many(slice, "elts").len() >= 2
                {
                    return Some(slice);
                }
            }
            None
        })
        .collect();
    for target in &targets {
        let mut elts = tree.many(*target, "elts").to_vec();
        elts.pop();
        tree.set_many(*target, "elts", elts);
    }
    !targets.is_empty()
}

// seed-147, dropped last exception handler.

fn apply_dropped_last_exception_handler(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(&tree.nodes[*id].data, Data::Try { .. })
                && tree.many(*id, "handlers").len() >= 2
        })
        .collect();
    for id in &targets {
        let mut handlers = tree.many(*id, "handlers").to_vec();
        handlers.pop();
        tree.set_many(*id, "handlers", handlers);
    }
    !targets.is_empty()
}
// seed-148, dropped last f-string element.

fn apply_dropped_last_fstring_element(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(&tree.nodes[*id].data, Data::JoinedStr) && tree.many(*id, "values").len() >= 2
        })
        .collect();
    for id in &targets {
        let mut values = tree.many(*id, "values").to_vec();
        values.pop();
        tree.set_many(*id, "values", values);
    }
    !targets.is_empty()
}

// seed-149, dropped last return element.

fn apply_dropped_last_return_element(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter_map(|id| match &tree.nodes[id].data {
            Data::Return => tree.one(id, "value").filter(|value| {
                matches!(&tree.nodes[*value].data, Data::Tuple { .. })
                    && tree.many(*value, "elts").len() >= 2
            }),
            _ => None,
        })
        .collect();
    for value in &targets {
        let mut elts = tree.many(*value, "elts").to_vec();
        elts.pop();
        tree.set_many(*value, "elts", elts);
    }
    !targets.is_empty()
}

// seed-150, dropped first subscript index element.

fn apply_dropped_first_subscript_index_element(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter_map(|id| match &tree.nodes[id].data {
            Data::Subscript { .. } => tree.one(id, "slice").filter(|slice| {
                matches!(&tree.nodes[*slice].data, Data::Tuple { .. })
                    && tree.many(*slice, "elts").len() >= 2
            }),
            _ => None,
        })
        .collect();
    for slice in &targets {
        let mut elts = tree.many(*slice, "elts").to_vec();
        elts.remove(0);
        tree.set_many(*slice, "elts", elts);
    }
    !targets.is_empty()
}

// seed-151, dropped last set element.

fn apply_dropped_last_set_element(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(&tree.nodes[*id].data, Data::Set) && tree.many(*id, "elts").len() >= 2
        })
        .collect();
    for id in &targets {
        let mut elts = tree.many(*id, "elts").to_vec();
        elts.pop();
        tree.set_many(*id, "elts", elts);
    }
    !targets.is_empty()
}

// seed-152, dropped first f-string element.

fn apply_dropped_first_fstring_element(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(&tree.nodes[*id].data, Data::JoinedStr) && tree.many(*id, "values").len() >= 2
        })
        .collect();
    for id in &targets {
        let mut values = tree.many(*id, "values").to_vec();
        values.remove(0);
        tree.set_many(*id, "values", values);
    }
    !targets.is_empty()
}

// seed-153, dropped first return element.

fn apply_dropped_first_return_element(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter_map(|id| match &tree.nodes[id].data {
            Data::Return => tree.one(id, "value").filter(|value| {
                matches!(&tree.nodes[*value].data, Data::Tuple { .. })
                    && tree.many(*value, "elts").len() >= 2
            }),
            _ => None,
        })
        .collect();
    for value in &targets {
        let mut elts = tree.many(*value, "elts").to_vec();
        elts.remove(0);
        tree.set_many(*value, "elts", elts);
    }
    !targets.is_empty()
}

// seed-154, dropped last match case.

fn apply_dropped_last_match_case(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(&tree.nodes[*id].data, Data::Match) && tree.many(*id, "cases").len() >= 2
        })
        .collect();
    for id in &targets {
        let mut cases = tree.many(*id, "cases").to_vec();
        cases.pop();
        tree.set_many(*id, "cases", cases);
    }
    !targets.is_empty()
}

// seed-155, dropped last decorator.

fn apply_dropped_last_decorator(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(
                &tree.nodes[*id].data,
                Data::FuncDef { .. } | Data::ClassDef { .. }
            ) && tree.many(*id, "decorator_list").len() >= 2
        })
        .collect();
    for id in &targets {
        let mut decorators = tree.many(*id, "decorator_list").to_vec();
        decorators.pop();
        tree.set_many(*id, "decorator_list", decorators);
    }
    !targets.is_empty()
}

// seed-156, dropped last assignment target.

fn apply_dropped_last_assignment_target(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter(|id| {
            matches!(&tree.nodes[*id].data, Data::Assign) && tree.many(*id, "targets").len() >= 2
        })
        .collect();
    for id in &targets {
        let mut kept = tree.many(*id, "targets").to_vec();
        kept.pop();
        tree.set_many(*id, "targets", kept);
    }
    !targets.is_empty()
}

// seed-157, dropped dictionary unpacking entry. The
// last unpacking entry standing after the first entry
// of every dictionary display stands removed, its
// value with it, where seed-139 removes the last entry
// of every dictionary display of any kind and seed-09
// removes the first entry of every dictionary display
// and reads no unpacking entry apart.

fn apply_dropped_dictionary_unpacking_entry(tree: &mut Tree) -> bool {
    let mut targets: Vec<(usize, usize)> = Vec::new();
    for id in tree.sorted_nodes() {
        if !matches!(&tree.nodes[id].data, Data::Dict) {
            continue;
        }
        let keys = tree.maybe(id, "keys");
        if let Some(pos) = keys.iter().rposition(|key| key.is_none()) {
            if pos >= 1 {
                targets.push((id, pos));
            }
        }
    }
    for (id, pos) in &targets {
        let mut keys = tree.maybe(*id, "keys").to_vec();
        keys.remove(*pos);
        tree.set_maybe(*id, "keys", keys);
        let mut values = tree.many(*id, "values").to_vec();
        values.remove(*pos);
        tree.set_many(*id, "values", values);
    }
    !targets.is_empty()
}

// seed-158, dropped first assignment value element.

fn apply_dropped_first_assignment_value_element(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter_map(|id| match &tree.nodes[id].data {
            Data::Assign => tree.one(id, "value").filter(|value| {
                matches!(&tree.nodes[*value].data, Data::Tuple { .. })
                    && tree.many(*value, "elts").len() >= 2
            }),
            _ => None,
        })
        .collect();
    for value in &targets {
        let mut elts = tree.many(*value, "elts").to_vec();
        elts.remove(0);
        tree.set_many(*value, "elts", elts);
    }
    !targets.is_empty()
}

// seed-159, dropped last assignment value element.

fn apply_dropped_last_assignment_value_element(tree: &mut Tree) -> bool {
    let targets: Vec<usize> = tree
        .sorted_nodes()
        .into_iter()
        .filter_map(|id| match &tree.nodes[id].data {
            Data::Assign => tree.one(id, "value").filter(|value| {
                matches!(&tree.nodes[*value].data, Data::Tuple { .. })
                    && tree.many(*value, "elts").len() >= 2
            }),
            _ => None,
        })
        .collect();
    for value in &targets {
        let mut elts = tree.many(*value, "elts").to_vec();
        elts.pop();
        tree.set_many(*value, "elts", elts);
    }
    !targets.is_empty()
}

// seed-160, dropped last tuple argument element.

fn apply_dropped_last_tuple_argument_element(tree: &mut Tree) -> bool {
    let mut targets: Vec<usize> = Vec::new();
    for id in tree.sorted_nodes() {
        if !matches!(&tree.nodes[id].data, Data::Call) {
            continue;
        }
        for arg in tree.many(id, "args").to_vec() {
            if matches!(&tree.nodes[arg].data, Data::Tuple { .. })
                && tree.many(arg, "elts").len() >= 2
            {
                targets.push(arg);
            }
        }
    }
    let planted = !targets.is_empty();
    for id in targets {
        let mut elts = tree.many(id, "elts").to_vec();
        elts.pop();
        tree.set_many(id, "elts", elts);
    }
    planted
}

// seed-161, dropped first tuple argument element.

fn apply_dropped_first_tuple_argument_element(tree: &mut Tree) -> bool {
    let mut targets: Vec<usize> = Vec::new();
    for id in tree.sorted_nodes() {
        if !matches!(&tree.nodes[id].data, Data::Call) {
            continue;
        }
        for arg in tree.many(id, "args").to_vec() {
            if matches!(&tree.nodes[arg].data, Data::Tuple { .. })
                && tree.many(arg, "elts").len() >= 2
            {
                targets.push(arg);
            }
        }
    }
    let planted = !targets.is_empty();
    for id in targets {
        let elts = tree.many(id, "elts")[1..].to_vec();
        tree.set_many(id, "elts", elts);
    }
    planted
}

// seed-162, dropped last for loop tuple element.

fn apply_dropped_last_for_loop_tuple_element(tree: &mut Tree) -> bool {
    let mut targets: Vec<usize> = Vec::new();
    for id in tree.sorted_nodes() {
        if !matches!(&tree.nodes[id].data, Data::For { .. }) {
            continue;
        }
        if let Some(iter) = tree.one(id, "iter") {
            if matches!(&tree.nodes[iter].data, Data::Tuple { .. })
                && tree.many(iter, "elts").len() >= 2
            {
                targets.push(iter);
            }
        }
    }
    let planted = !targets.is_empty();
    for id in targets {
        let mut elts = tree.many(id, "elts").to_vec();
        elts.pop();
        tree.set_many(id, "elts", elts);
    }
    planted
}

// seed-163, dropped first for loop tuple element.

fn apply_dropped_first_for_loop_tuple_element(tree: &mut Tree) -> bool {
    let mut targets: Vec<usize> = Vec::new();
    for id in tree.sorted_nodes() {
        if !matches!(&tree.nodes[id].data, Data::For { .. }) {
            continue;
        }
        if let Some(iter) = tree.one(id, "iter") {
            if matches!(&tree.nodes[iter].data, Data::Tuple { .. })
                && tree.many(iter, "elts").len() >= 2
            {
                targets.push(iter);
            }
        }
    }
    let planted = !targets.is_empty();
    for id in targets {
        let elts = tree.many(id, "elts")[1..].to_vec();
        tree.set_many(id, "elts", elts);
    }
    planted
}

// seed-164, dropped last annotated assignment value element.

fn apply_dropped_last_annotated_assignment_value_element(tree: &mut Tree) -> bool {
    let mut targets: Vec<usize> = Vec::new();
    for id in tree.sorted_nodes() {
        if !matches!(&tree.nodes[id].data, Data::AnnAssign { .. }) {
            continue;
        }
        if let Some(value) = tree.one(id, "value") {
            if matches!(&tree.nodes[value].data, Data::Tuple { .. })
                && tree.many(value, "elts").len() >= 2
            {
                targets.push(value);
            }
        }
    }
    let planted = !targets.is_empty();
    for id in targets {
        let mut elts = tree.many(id, "elts").to_vec();
        elts.pop();
        tree.set_many(id, "elts", elts);
    }
    planted
}

// seed-165, dropped first annotated assignment value element.

fn apply_dropped_first_annotated_assignment_value_element(tree: &mut Tree) -> bool {
    let mut targets: Vec<usize> = Vec::new();
    for id in tree.sorted_nodes() {
        if !matches!(&tree.nodes[id].data, Data::AnnAssign { .. }) {
            continue;
        }
        if let Some(value) = tree.one(id, "value") {
            if matches!(&tree.nodes[value].data, Data::Tuple { .. })
                && tree.many(value, "elts").len() >= 2
            {
                targets.push(value);
            }
        }
    }
    let planted = !targets.is_empty();
    for id in targets {
        let elts = tree.many(id, "elts")[1..].to_vec();
        tree.set_many(id, "elts", elts);
    }
    planted
}

// seed-166, dropped last comprehension tuple element.

fn apply_dropped_last_comprehension_tuple_element(tree: &mut Tree) -> bool {
    let mut targets: Vec<usize> = Vec::new();
    for id in tree.sorted_nodes() {
        if !matches!(
            &tree.nodes[id].data,
            Data::ListComp | Data::SetComp | Data::GeneratorExp
        ) {
            continue;
        }
        if let Some(elt) = tree.one(id, "elt") {
            if matches!(&tree.nodes[elt].data, Data::Tuple { .. })
                && tree.many(elt, "elts").len() >= 2
            {
                targets.push(elt);
            }
        }
    }
    let planted = !targets.is_empty();
    for id in targets {
        let mut elts = tree.many(id, "elts").to_vec();
        elts.pop();
        tree.set_many(id, "elts", elts);
    }
    planted
}

// seed-167, dropped first comprehension tuple element.

fn apply_dropped_first_comprehension_tuple_element(tree: &mut Tree) -> bool {
    let mut targets: Vec<usize> = Vec::new();
    for id in tree.sorted_nodes() {
        if !matches!(
            &tree.nodes[id].data,
            Data::ListComp | Data::SetComp | Data::GeneratorExp
        ) {
            continue;
        }
        if let Some(elt) = tree.one(id, "elt") {
            if matches!(&tree.nodes[elt].data, Data::Tuple { .. })
                && tree.many(elt, "elts").len() >= 2
            {
                targets.push(elt);
            }
        }
    }
    let planted = !targets.is_empty();
    for id in targets {
        let elts = tree.many(id, "elts")[1..].to_vec();
        tree.set_many(id, "elts", elts);
    }
    planted
}

// seed-168, dropped last lambda body tuple element.

fn apply_dropped_last_lambda_body_tuple_element(tree: &mut Tree) -> bool {
    let mut targets: Vec<usize> = Vec::new();
    for id in tree.sorted_nodes() {
        if !matches!(&tree.nodes[id].data, Data::Lambda) {
            continue;
        }
        if let Some(body) = tree.one(id, "body") {
            if matches!(&tree.nodes[body].data, Data::Tuple { .. })
                && tree.many(body, "elts").len() >= 2
            {
                targets.push(body);
            }
        }
    }
    let planted = !targets.is_empty();
    for id in targets {
        let mut elts = tree.many(id, "elts").to_vec();
        elts.pop();
        tree.set_many(id, "elts", elts);
    }
    planted
}

// seed-169, dropped first lambda body tuple element.

fn apply_dropped_first_lambda_body_tuple_element(tree: &mut Tree) -> bool {
    let mut targets: Vec<usize> = Vec::new();
    for id in tree.sorted_nodes() {
        if !matches!(&tree.nodes[id].data, Data::Lambda) {
            continue;
        }
        if let Some(body) = tree.one(id, "body") {
            if matches!(&tree.nodes[body].data, Data::Tuple { .. })
                && tree.many(body, "elts").len() >= 2
            {
                targets.push(body);
            }
        }
    }
    let planted = !targets.is_empty();
    for id in targets {
        let elts = tree.many(id, "elts")[1..].to_vec();
        tree.set_many(id, "elts", elts);
    }
    planted
}

// seed-170, dropped last comparison operand tuple element.

fn apply_dropped_last_comparison_operand_tuple_element(tree: &mut Tree) -> bool {
    let mut targets: Vec<usize> = Vec::new();
    for id in tree.sorted_nodes() {
        if !matches!(&tree.nodes[id].data, Data::Compare { .. }) {
            continue;
        }
        if let Some(left) = tree.one(id, "left") {
            if matches!(&tree.nodes[left].data, Data::Tuple { .. })
                && tree.many(left, "elts").len() >= 2
            {
                targets.push(left);
            }
        }
        for comparator in tree.many(id, "comparators").to_vec() {
            if matches!(&tree.nodes[comparator].data, Data::Tuple { .. })
                && tree.many(comparator, "elts").len() >= 2
            {
                targets.push(comparator);
            }
        }
    }
    let planted = !targets.is_empty();
    for id in targets {
        let mut elts = tree.many(id, "elts").to_vec();
        elts.pop();
        tree.set_many(id, "elts", elts);
    }
    planted
}

// seed-171, dropped first comparison operand tuple element.

fn apply_dropped_first_comparison_operand_tuple_element(tree: &mut Tree) -> bool {
    let mut targets: Vec<usize> = Vec::new();
    for id in tree.sorted_nodes() {
        if !matches!(&tree.nodes[id].data, Data::Compare { .. }) {
            continue;
        }
        if let Some(left) = tree.one(id, "left") {
            if matches!(&tree.nodes[left].data, Data::Tuple { .. })
                && tree.many(left, "elts").len() >= 2
            {
                targets.push(left);
            }
        }
        for comparator in tree.many(id, "comparators").to_vec() {
            if matches!(&tree.nodes[comparator].data, Data::Tuple { .. })
                && tree.many(comparator, "elts").len() >= 2
            {
                targets.push(comparator);
            }
        }
    }
    let planted = !targets.is_empty();
    for id in targets {
        let elts = tree.many(id, "elts")[1..].to_vec();
        tree.set_many(id, "elts", elts);
    }
    planted
}

// seed-172, dropped last parameter default tuple element.

fn apply_dropped_last_parameter_default_tuple_element(tree: &mut Tree) -> bool {
    let mut targets: Vec<usize> = Vec::new();
    for id in tree.sorted_nodes() {
        if !matches!(&tree.nodes[id].data, Data::FuncDef { .. } | Data::Lambda) {
            continue;
        }
        if let Some(args) = tree.one(id, "args") {
            for default in tree.many(args, "defaults").to_vec() {
                if matches!(&tree.nodes[default].data, Data::Tuple { .. })
                    && tree.many(default, "elts").len() >= 2
                {
                    targets.push(default);
                }
            }
        }
    }
    let planted = !targets.is_empty();
    for id in targets {
        let mut elts = tree.many(id, "elts").to_vec();
        elts.pop();
        tree.set_many(id, "elts", elts);
    }
    planted
}

// seed-173, dropped first parameter default tuple element.

fn apply_dropped_first_parameter_default_tuple_element(tree: &mut Tree) -> bool {
    let mut targets: Vec<usize> = Vec::new();
    for id in tree.sorted_nodes() {
        if !matches!(&tree.nodes[id].data, Data::FuncDef { .. } | Data::Lambda) {
            continue;
        }
        if let Some(args) = tree.one(id, "args") {
            for default in tree.many(args, "defaults").to_vec() {
                if matches!(&tree.nodes[default].data, Data::Tuple { .. })
                    && tree.many(default, "elts").len() >= 2
                {
                    targets.push(default);
                }
            }
        }
    }
    let planted = !targets.is_empty();
    for id in targets {
        let elts = tree.many(id, "elts")[1..].to_vec();
        tree.set_many(id, "elts", elts);
    }
    planted
}

// seed-174, dropped last conditional expression tuple element.

fn apply_dropped_last_conditional_expression_tuple_element(tree: &mut Tree) -> bool {
    let mut targets: Vec<usize> = Vec::new();
    for id in tree.sorted_nodes() {
        if !matches!(&tree.nodes[id].data, Data::IfExp) {
            continue;
        }
        for field in ["body", "orelse"] {
            if let Some(arm) = tree.one(id, field) {
                if matches!(&tree.nodes[arm].data, Data::Tuple { .. })
                    && tree.many(arm, "elts").len() >= 2
                {
                    targets.push(arm);
                }
            }
        }
    }
    let planted = !targets.is_empty();
    for id in targets {
        let mut elts = tree.many(id, "elts").to_vec();
        elts.pop();
        tree.set_many(id, "elts", elts);
    }
    planted
}

// seed-175, dropped first conditional expression tuple element.

fn apply_dropped_first_conditional_expression_tuple_element(tree: &mut Tree) -> bool {
    let mut targets: Vec<usize> = Vec::new();
    for id in tree.sorted_nodes() {
        if !matches!(&tree.nodes[id].data, Data::IfExp) {
            continue;
        }
        for field in ["body", "orelse"] {
            if let Some(arm) = tree.one(id, field) {
                if matches!(&tree.nodes[arm].data, Data::Tuple { .. })
                    && tree.many(arm, "elts").len() >= 2
                {
                    targets.push(arm);
                }
            }
        }
    }
    let planted = !targets.is_empty();
    for id in targets {
        let elts = tree.many(id, "elts")[1..].to_vec();
        tree.set_many(id, "elts", elts);
    }
    planted
}

// seed-176, dropped last exception handler type tuple element.

fn apply_dropped_last_exception_handler_type_tuple_element(tree: &mut Tree) -> bool {
    let mut targets: Vec<usize> = Vec::new();
    for id in tree.sorted_nodes() {
        if !matches!(&tree.nodes[id].data, Data::ExceptHandler { .. }) {
            continue;
        }
        if let Some(ty) = tree.one(id, "type") {
            if matches!(&tree.nodes[ty].data, Data::Tuple { .. })
                && tree.many(ty, "elts").len() >= 2
            {
                targets.push(ty);
            }
        }
    }
    let planted = !targets.is_empty();
    for id in targets {
        let mut elts = tree.many(id, "elts").to_vec();
        elts.pop();
        tree.set_many(id, "elts", elts);
    }
    planted
}

// seed-177, dropped first exception handler type tuple element.

fn apply_dropped_first_exception_handler_type_tuple_element(tree: &mut Tree) -> bool {
    let mut targets: Vec<usize> = Vec::new();
    for id in tree.sorted_nodes() {
        if !matches!(&tree.nodes[id].data, Data::ExceptHandler { .. }) {
            continue;
        }
        if let Some(ty) = tree.one(id, "type") {
            if matches!(&tree.nodes[ty].data, Data::Tuple { .. })
                && tree.many(ty, "elts").len() >= 2
            {
                targets.push(ty);
            }
        }
    }
    let planted = !targets.is_empty();
    for id in targets {
        let elts = tree.many(id, "elts")[1..].to_vec();
        tree.set_many(id, "elts", elts);
    }
    planted
}

// seed-178, dropped last keyword value tuple element.

fn apply_dropped_last_keyword_value_tuple_element(tree: &mut Tree) -> bool {
    let mut targets: Vec<usize> = Vec::new();
    for id in tree.sorted_nodes() {
        if !matches!(&tree.nodes[id].data, Data::Call) {
            continue;
        }
        for kw in tree.many(id, "keywords").to_vec() {
            if let Some(value) = tree.one(kw, "value") {
                if matches!(&tree.nodes[value].data, Data::Tuple { .. })
                    && tree.many(value, "elts").len() >= 2
                {
                    targets.push(value);
                }
            }
        }
    }
    let planted = !targets.is_empty();
    for id in targets {
        let mut elts = tree.many(id, "elts").to_vec();
        elts.pop();
        tree.set_many(id, "elts", elts);
    }
    planted
}

// seed-179, dropped first keyword value tuple element.

fn apply_dropped_first_keyword_value_tuple_element(tree: &mut Tree) -> bool {
    let mut targets: Vec<usize> = Vec::new();
    for id in tree.sorted_nodes() {
        if !matches!(&tree.nodes[id].data, Data::Call) {
            continue;
        }
        for kw in tree.many(id, "keywords").to_vec() {
            if let Some(value) = tree.one(kw, "value") {
                if matches!(&tree.nodes[value].data, Data::Tuple { .. })
                    && tree.many(value, "elts").len() >= 2
                {
                    targets.push(value);
                }
            }
        }
    }
    let planted = !targets.is_empty();
    for id in targets {
        let elts = tree.many(id, "elts")[1..].to_vec();
        tree.set_many(id, "elts", elts);
    }
    planted
}
