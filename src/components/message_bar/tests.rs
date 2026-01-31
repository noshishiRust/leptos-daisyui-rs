//! Unit tests for MessageBar component

use super::style::MessageBarType;

#[test]
fn test_message_bar_type_count() {
    let types = [
        MessageBarType::Info,
        MessageBarType::Success,
        MessageBarType::Warning,
        MessageBarType::Error,
    ];

    assert_eq!(types.len(), 4, "Should have 4 message bar types");
}

#[test]
fn test_message_bar_bg_classes() {
    assert_eq!(MessageBarType::Info.bg_class(), "bg-info");
    assert_eq!(MessageBarType::Success.bg_class(), "bg-success");
    assert_eq!(MessageBarType::Warning.bg_class(), "bg-warning");
    assert_eq!(MessageBarType::Error.bg_class(), "bg-error");
}

#[test]
fn test_message_bar_text_classes() {
    assert_eq!(MessageBarType::Info.text_class(), "text-info-content");
    assert_eq!(MessageBarType::Success.text_class(), "text-success-content");
    assert_eq!(MessageBarType::Warning.text_class(), "text-warning-content");
    assert_eq!(MessageBarType::Error.text_class(), "text-error-content");
}

#[test]
fn test_message_bar_icons() {
    assert_eq!(MessageBarType::Info.icon(), "ℹ️");
    assert_eq!(MessageBarType::Success.icon(), "✓");
    assert_eq!(MessageBarType::Warning.icon(), "⚠");
    assert_eq!(MessageBarType::Error.icon(), "✕");
}

#[test]
fn test_message_bar_type_default() {
    assert_eq!(
        MessageBarType::default(),
        MessageBarType::Info,
        "Default type should be Info"
    );
}

#[test]
fn test_message_bar_type_clone() {
    let msg_type = MessageBarType::Warning;
    let cloned = msg_type.clone();
    assert_eq!(msg_type, cloned, "Cloned type should equal original");
}

#[test]
fn test_message_bar_type_debug() {
    let msg_type = MessageBarType::Error;
    let debug_str = format!("{:?}", msg_type);
    assert!(
        debug_str.contains("Error"),
        "Debug output should contain type name"
    );
}

#[test]
fn test_message_bar_bg_classes_use_bg_prefix() {
    let types = [
        MessageBarType::Info,
        MessageBarType::Success,
        MessageBarType::Warning,
        MessageBarType::Error,
    ];

    for msg_type in types {
        assert!(
            msg_type.bg_class().starts_with("bg-"),
            "Background class '{}' should start with 'bg-'",
            msg_type.bg_class()
        );
    }
}

#[test]
fn test_message_bar_text_classes_use_text_prefix() {
    let types = [
        MessageBarType::Info,
        MessageBarType::Success,
        MessageBarType::Warning,
        MessageBarType::Error,
    ];

    for msg_type in types {
        assert!(
            msg_type.text_class().starts_with("text-"),
            "Text class '{}' should start with 'text-'",
            msg_type.text_class()
        );
    }
}

#[test]
fn test_message_bar_text_classes_use_content_suffix() {
    let types = [
        MessageBarType::Info,
        MessageBarType::Success,
        MessageBarType::Warning,
        MessageBarType::Error,
    ];

    for msg_type in types {
        assert!(
            msg_type.text_class().ends_with("-content"),
            "Text class '{}' should end with '-content'",
            msg_type.text_class()
        );
    }
}

#[test]
fn test_message_bar_icons_are_unique() {
    let icons = [
        MessageBarType::Info.icon(),
        MessageBarType::Success.icon(),
        MessageBarType::Warning.icon(),
        MessageBarType::Error.icon(),
    ];

    let unique_count = icons.iter().collect::<std::collections::HashSet<_>>().len();
    assert_eq!(
        icons.len(),
        unique_count,
        "All message bar icons should be unique"
    );
}

#[test]
fn test_message_bar_icons_not_empty() {
    let types = [
        MessageBarType::Info,
        MessageBarType::Success,
        MessageBarType::Warning,
        MessageBarType::Error,
    ];

    for msg_type in types {
        assert!(
            !msg_type.icon().is_empty(),
            "Icon for {:?} should not be empty",
            msg_type
        );
    }
}
