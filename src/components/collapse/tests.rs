use super::*;

// CollapseModifier tests
#[test]
fn test_collapse_modifier_default() {
    let modifier = CollapseModifier::default();
    assert_eq!(modifier.as_str(), "");
}

#[test]
fn test_collapse_modifier_arrow() {
    let modifier = CollapseModifier::Arrow;
    assert_eq!(modifier.as_str(), "collapse-arrow");
}

#[test]
fn test_collapse_modifier_plus() {
    let modifier = CollapseModifier::Plus;
    assert_eq!(modifier.as_str(), "collapse-plus");
}

#[test]
fn test_collapse_modifier_clone() {
    let modifier1 = CollapseModifier::Arrow;
    let modifier2 = modifier1.clone();
    assert_eq!(modifier1.as_str(), modifier2.as_str());
}

#[test]
fn test_collapse_modifier_debug() {
    let modifier = CollapseModifier::Plus;
    assert!(format!("{:?}", modifier).contains("Plus"));
}

// CollapseForceModifier tests
#[test]
fn test_collapse_force_modifier_default() {
    let force = CollapseForceModifier::default();
    assert_eq!(force.as_str(), "");
}

#[test]
fn test_collapse_force_modifier_open() {
    let force = CollapseForceModifier::Open;
    assert_eq!(force.as_str(), "collapse-open");
}

#[test]
fn test_collapse_force_modifier_close() {
    let force = CollapseForceModifier::Close;
    assert_eq!(force.as_str(), "collapse-close");
}

#[test]
fn test_collapse_force_modifier_clone() {
    let force1 = CollapseForceModifier::Open;
    let force2 = force1.clone();
    assert_eq!(force1.as_str(), force2.as_str());
}

#[test]
fn test_collapse_force_modifier_debug() {
    let force = CollapseForceModifier::Close;
    assert!(format!("{:?}", force).contains("Close"));
}

// Comprehensive enum variant coverage tests
#[test]
fn test_all_collapse_modifiers_return_valid_classes() {
    let modifiers = vec![
        (CollapseModifier::Default, ""),
        (CollapseModifier::Arrow, "collapse-arrow"),
        (CollapseModifier::Plus, "collapse-plus"),
    ];

    for (modifier, expected) in modifiers {
        assert_eq!(modifier.as_str(), expected);
    }
}

#[test]
fn test_all_collapse_force_modifiers_return_valid_classes() {
    let forces = vec![
        (CollapseForceModifier::Default, ""),
        (CollapseForceModifier::Open, "collapse-open"),
        (CollapseForceModifier::Close, "collapse-close"),
    ];

    for (force, expected) in forces {
        assert_eq!(force.as_str(), expected);
    }
}
