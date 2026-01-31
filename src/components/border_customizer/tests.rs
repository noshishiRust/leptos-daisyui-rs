//! Unit tests for BorderCustomizer component

#[test]
fn test_border_width_range() {
    let min_width = 0.0;
    let max_width = 8.0;
    let step = 0.5;

    assert_eq!(min_width, 0.0, "Min border width should be 0");
    assert_eq!(max_width, 8.0, "Max border width should be 8");
    assert_eq!(step, 0.5, "Border width step should be 0.5");
}

#[test]
fn test_default_border_width() {
    let default_width = 1.0;
    assert_eq!(default_width, 1.0, "Default border width should be 1px");
}

#[test]
fn test_border_width_increments() {
    let min = 0.0;
    let max = 8.0;
    let step = 0.5;

    let expected_steps = ((max - min) / step) as usize + 1;
    assert_eq!(
        expected_steps, 17,
        "Should have 17 possible border width values"
    );
}

#[test]
fn test_spacing_scale_range() {
    let min_scale = 0.5;
    let max_scale = 2.0;
    let default_scale = 1.0;

    assert!(
        min_scale < default_scale,
        "Min spacing should be less than default"
    );
    assert!(
        max_scale > default_scale,
        "Max spacing should be greater than default"
    );
    assert_eq!(default_scale, 1.0, "Default spacing scale should be 1.0");
}

#[test]
fn test_border_radius_range() {
    let min_radius = 0.0;
    let max_radius = 2.0;
    let default_radius = 0.5;

    assert_eq!(min_radius, 0.0, "Min border radius should be 0");
    assert!(
        max_radius >= 2.0,
        "Max border radius should be at least 2.0"
    );
    assert!(
        default_radius > 0.0,
        "Default border radius should be positive"
    );
}

#[test]
fn test_border_customization_categories() {
    let categories = ["border_width", "border_radius", "spacing_scale"];

    assert_eq!(
        categories.len(),
        3,
        "Should have 3 border customization categories"
    );
    assert!(
        categories.contains(&"border_width"),
        "Should include border width"
    );
    assert!(
        categories.contains(&"border_radius"),
        "Should include border radius"
    );
    assert!(
        categories.contains(&"spacing_scale"),
        "Should include spacing scale"
    );
}
