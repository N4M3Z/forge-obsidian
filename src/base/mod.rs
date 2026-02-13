#[cfg(test)]
mod tests;

use serde_yaml::Value;
use std::fs;
use std::path::Path;

/// Top-level .base file structure.
#[derive(Debug, Clone)]
pub struct BaseSpec {
    pub filters: Option<FilterNode>,
    pub views: Vec<ViewSpec>,
}

/// A single view within the base (table, cards, list).
#[derive(Debug, Clone)]
pub struct ViewSpec {
    pub view_type: String,
    pub name: String,
    pub filters: Option<FilterNode>,
    pub order: Vec<String>,
    pub sort: Vec<SortSpec>,
}

/// Sort specification for a view.
#[derive(Debug, Clone)]
pub struct SortSpec {
    pub property: String,
    pub direction: SortDirection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortDirection {
    Asc,
    Desc,
}

/// Boolean combinator node: and/or over filter expressions.
#[derive(Debug, Clone)]
pub enum FilterNode {
    And(Vec<FilterEntry>),
    Or(Vec<FilterEntry>),
}

/// A single entry in a filter list — either a nested combinator or an expression string.
#[derive(Debug, Clone)]
pub enum FilterEntry {
    Expr(String),
    Nested(FilterNode),
}

/// Parse a `.base` file from disk.
pub fn parse_file(path: &Path) -> Result<BaseSpec, String> {
    let content = fs::read_to_string(path).map_err(|e| format!("Cannot read {}: {e}", path.display()))?;
    parse_str(&content)
}

/// Parse `.base` YAML from a string.
pub fn parse_str(content: &str) -> Result<BaseSpec, String> {
    let value: Value =
        serde_yaml::from_str(content).map_err(|e| format!("Invalid YAML: {e}"))?;

    let top_filters = value.get("filters").and_then(parse_filter_node);

    let views = match value.get("views") {
        Some(Value::Sequence(seq)) => seq.iter().filter_map(parse_view).collect(),
        _ => Vec::new(),
    };

    Ok(BaseSpec {
        filters: top_filters,
        views,
    })
}

fn parse_view(val: &Value) -> Option<ViewSpec> {
    let view_type = val.get("type")?.as_str()?.to_owned();
    let name = val
        .get("name")
        .and_then(Value::as_str)
        .unwrap_or("Unnamed")
        .to_owned();

    let filters = val.get("filters").and_then(parse_filter_node);

    let order = match val.get("order") {
        Some(Value::Sequence(seq)) => seq.iter().filter_map(|v| v.as_str().map(String::from)).collect(),
        _ => Vec::new(),
    };

    let sort = match val.get("sort") {
        Some(Value::Sequence(seq)) => seq.iter().filter_map(parse_sort).collect(),
        _ => Vec::new(),
    };

    Some(ViewSpec {
        view_type,
        name,
        filters,
        order,
        sort,
    })
}

fn parse_filter_node(val: &Value) -> Option<FilterNode> {
    if let Some(and_list) = val.get("and").and_then(Value::as_sequence) {
        let entries = and_list.iter().filter_map(parse_filter_entry).collect();
        return Some(FilterNode::And(entries));
    }
    if let Some(or_list) = val.get("or").and_then(Value::as_sequence) {
        let entries = or_list.iter().filter_map(parse_filter_entry).collect();
        return Some(FilterNode::Or(entries));
    }
    None
}

fn parse_filter_entry(val: &Value) -> Option<FilterEntry> {
    // Nested combinator (an object with `and` or `or` key)
    if val.is_mapping() {
        if let Some(node) = parse_filter_node(val) {
            return Some(FilterEntry::Nested(node));
        }
    }
    // String expression
    if let Some(s) = val.as_str() {
        return Some(FilterEntry::Expr(s.to_owned()));
    }
    None
}

fn parse_sort(val: &Value) -> Option<SortSpec> {
    // Handles both `property:` and `column:` keys (Obsidian uses both)
    let property = val
        .get("property")
        .or_else(|| val.get("column"))
        .and_then(Value::as_str)?
        .to_owned();

    let direction = match val.get("direction").and_then(Value::as_str) {
        Some(d) if d.eq_ignore_ascii_case("desc") => SortDirection::Desc,
        _ => SortDirection::Asc,
    };

    Some(SortSpec {
        property,
        direction,
    })
}
