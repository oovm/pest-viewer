use std::cmp::max;

use pest::{iterators::Pair, RuleType};

pub fn width_hint<R>(node: Pair<R>) -> f64
where
    R: RuleType,
{
    let text = if has_child(&node) {
        // take node rule
        format!("{:?}", node.as_rule())
    }
    else {
        // take leaf text
        node.as_str().to_string()
    };
    // not too narrow
    max(text.len(), 3) as f64
}

/// white space is not child
fn has_child<R>(node: &Pair<R>) -> bool
where
    R: RuleType,
{
    for i in node.clone().into_inner() {
        if ignored_rule(i.as_rule()) {
            continue;
        }
        return true;
    }
    false
}
/// white space is not child
fn get_children<'i, R>(node: &Pair<'i, R>) -> Vec<Pair<'i, R>>
where
    R: RuleType,
{
    let mut out = vec![];
    for i in node.clone().into_inner() {
        if ignored_rule(i.as_rule()) {
            continue;
        }
        out.push(i)
    }
    out
}

fn ignored_rule(rule: impl RuleType) -> bool {
    let name = format!("{:?}", rule);
    if name.eq("WHITESPACE") || name.eq("COMMENT") {
        return true;
    }
    false
}
