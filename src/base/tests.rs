use super::*;

#[test]
fn parse_simple_base() {
    let yaml = r#"
views:
  - type: table
    name: Table
    filters:
      and:
        - contains(file.path, "Inventory")
        - contains(file.ext, "md")
    order:
      - file.name
      - root
    sort:
      - property: file.name
        direction: ASC
"#;

    let spec = parse_str(yaml).unwrap();
    assert!(spec.filters.is_none());
    assert_eq!(spec.views.len(), 1);
    let view = &spec.views[0];
    assert_eq!(view.view_type, "table");
    assert_eq!(view.name, "Table");
    assert_eq!(view.order, vec!["file.name", "root"]);
    assert_eq!(view.sort.len(), 1);
    assert_eq!(view.sort[0].direction, SortDirection::Asc);

    match &view.filters {
        Some(FilterNode::And(entries)) => {
            assert_eq!(entries.len(), 2);
            match &entries[0] {
                FilterEntry::Expr(s) => assert_eq!(s, r#"contains(file.path, "Inventory")"#),
                other => panic!("expected Expr, got {other:?}"),
            }
        }
        other => panic!("expected And, got {other:?}"),
    }
}

#[test]
fn parse_nested_filters() {
    let yaml = r#"
views:
  - type: table
    name: Mixed
    filters:
      or:
        - and:
            - contains(property.tags, "type/item")
        - and:
            - contains(file.path, "Inventory")
"#;

    let spec = parse_str(yaml).unwrap();
    let view = &spec.views[0];
    match &view.filters {
        Some(FilterNode::Or(entries)) => {
            assert_eq!(entries.len(), 2);
            assert!(matches!(&entries[0], FilterEntry::Nested(FilterNode::And(_))));
        }
        other => panic!("expected Or, got {other:?}"),
    }
}

#[test]
fn parse_top_level_filters() {
    let yaml = r#"
filters:
  and:
    - file.fullname != this.file.fullname
views:
  - type: table
    name: Notes
"#;

    let spec = parse_str(yaml).unwrap();
    assert!(spec.filters.is_some());
    match &spec.filters {
        Some(FilterNode::And(entries)) => {
            assert_eq!(entries.len(), 1);
        }
        other => panic!("expected And, got {other:?}"),
    }
}

#[test]
fn parse_column_sort_key() {
    let yaml = r#"
views:
  - type: table
    name: T
    sort:
      - column: property.tags
        direction: DESC
"#;

    let spec = parse_str(yaml).unwrap();
    assert_eq!(spec.views[0].sort[0].property, "property.tags");
    assert_eq!(spec.views[0].sort[0].direction, SortDirection::Desc);
}
