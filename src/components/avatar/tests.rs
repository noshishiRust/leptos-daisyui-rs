use super::*;

// AvatarModifier tests
#[test]
fn test_avatar_modifier_default() {
    let modifier = AvatarModifier::default();
    assert_eq!(modifier.as_str(), "");
}

#[test]
fn test_avatar_modifier_online() {
    let modifier = AvatarModifier::Online;
    assert_eq!(modifier.as_str(), "avatar-online");
}

#[test]
fn test_avatar_modifier_offline() {
    let modifier = AvatarModifier::Offline;
    assert_eq!(modifier.as_str(), "avatar-offline");
}

#[test]
fn test_avatar_modifier_placeholder() {
    let modifier = AvatarModifier::Placeholder;
    assert_eq!(modifier.as_str(), "avatar-placeholder");
}

#[test]
fn test_avatar_modifier_clone() {
    let modifier1 = AvatarModifier::Online;
    let modifier2 = modifier1.clone();
    assert_eq!(modifier1.as_str(), modifier2.as_str());
}

#[test]
fn test_avatar_modifier_debug() {
    let modifier = AvatarModifier::Offline;
    assert!(format!("{:?}", modifier).contains("Offline"));
}

// Comprehensive enum variant coverage tests
#[test]
fn test_all_avatar_modifiers_return_valid_classes() {
    let modifiers = vec![
        (AvatarModifier::Default, ""),
        (AvatarModifier::Online, "avatar-online"),
        (AvatarModifier::Offline, "avatar-offline"),
        (AvatarModifier::Placeholder, "avatar-placeholder"),
    ];

    for (modifier, expected) in modifiers {
        assert_eq!(modifier.as_str(), expected);
    }
}
