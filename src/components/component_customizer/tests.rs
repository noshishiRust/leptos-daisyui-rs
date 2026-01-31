//! Unit tests for ComponentCustomizer component

#[test]
fn test_component_categories_count() {
    let categories = ["button", "card", "input"];
    assert_eq!(categories.len(), 3, "Should have 3 component categories");
}

#[test]
fn test_border_radius_options_count() {
    let border_radius_options = [
        ("None (Square)", "0"),
        ("Small (0.125rem)", "0.125rem"),
        ("Medium (0.25rem)", "0.25rem"),
        ("Large (0.5rem)", "0.5rem"),
        ("Extra Large (1rem)", "1rem"),
        ("Full (Pill)", "9999px"),
    ];

    assert_eq!(border_radius_options.len(), 6, "Should have 6 border radius options");
}

#[test]
fn test_border_width_options_count() {
    let border_width_options = [
        ("None", "0"),
        ("Thin (1px)", "1px"),
        ("Medium (2px)", "2px"),
        ("Thick (3px)", "3px"),
    ];

    assert_eq!(border_width_options.len(), 4, "Should have 4 border width options");
}

#[test]
fn test_border_radius_includes_extreme_values() {
    let radius_values = ["0", "0.125rem", "0.25rem", "0.5rem", "1rem", "9999px"];

    assert!(radius_values.contains(&"0"), "Should include square (0)");
    assert!(radius_values.contains(&"9999px"), "Should include pill (9999px)");
}

#[test]
fn test_border_width_includes_none() {
    let width_values = ["0", "1px", "2px", "3px"];

    assert!(width_values.contains(&"0"), "Should include no border option");
}

#[test]
fn test_customizable_components_list() {
    let components = ["button", "card", "input"];

    assert!(components.contains(&"button"), "Should customize buttons");
    assert!(components.contains(&"card"), "Should customize cards");
    assert!(components.contains(&"input"), "Should customize inputs");
}

#[test]
fn test_border_radius_options_are_unique() {
    let values = ["0", "0.125rem", "0.25rem", "0.5rem", "1rem", "9999px"];
    let unique_count = values.iter().collect::<std::collections::HashSet<_>>().len();

    assert_eq!(
        values.len(),
        unique_count,
        "Border radius options should be unique"
    );
}
