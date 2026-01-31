//! Unit tests for Tag component

use super::style::{TagColor, TagSize};

#[test]
fn test_tag_color_count() {
    let colors = [
        TagColor::Neutral,
        TagColor::Primary,
        TagColor::Secondary,
        TagColor::Accent,
        TagColor::Success,
        TagColor::Warning,
        TagColor::Error,
        TagColor::Info,
    ];

    assert_eq!(colors.len(), 8, "Should have 8 tag colors");
}

#[test]
fn test_tag_size_count() {
    let sizes = [TagSize::Small, TagSize::Medium, TagSize::Large];

    assert_eq!(sizes.len(), 3, "Should have 3 tag sizes");
}

#[test]
fn test_tag_color_css_classes() {
    assert_eq!(TagColor::Neutral.as_str(), "badge-neutral");
    assert_eq!(TagColor::Primary.as_str(), "badge-primary");
    assert_eq!(TagColor::Secondary.as_str(), "badge-secondary");
    assert_eq!(TagColor::Accent.as_str(), "badge-accent");
    assert_eq!(TagColor::Success.as_str(), "badge-success");
    assert_eq!(TagColor::Warning.as_str(), "badge-warning");
    assert_eq!(TagColor::Error.as_str(), "badge-error");
    assert_eq!(TagColor::Info.as_str(), "badge-info");
}

#[test]
fn test_tag_size_css_classes() {
    assert_eq!(TagSize::Small.as_str(), "badge-sm");
    assert_eq!(TagSize::Medium.as_str(), "badge-md");
    assert_eq!(TagSize::Large.as_str(), "badge-lg");
}

#[test]
fn test_tag_color_default() {
    assert_eq!(
        TagColor::default(),
        TagColor::Neutral,
        "Default color should be Neutral"
    );
}

#[test]
fn test_tag_size_default() {
    assert_eq!(
        TagSize::default(),
        TagSize::Medium,
        "Default size should be Medium"
    );
}

#[test]
fn test_tag_color_clone() {
    let color = TagColor::Primary;
    let cloned = color.clone();
    assert_eq!(color, cloned, "Cloned color should equal original");
}

#[test]
fn test_tag_size_clone() {
    let size = TagSize::Large;
    let cloned = size.clone();
    assert_eq!(size, cloned, "Cloned size should equal original");
}

#[test]
fn test_tag_color_debug() {
    let color = TagColor::Success;
    let debug_str = format!("{:?}", color);
    assert!(
        debug_str.contains("Success"),
        "Debug output should contain color name"
    );
}

#[test]
fn test_tag_size_debug() {
    let size = TagSize::Small;
    let debug_str = format!("{:?}", size);
    assert!(
        debug_str.contains("Small"),
        "Debug output should contain size name"
    );
}

#[test]
fn test_tag_colors_use_badge_prefix() {
    let colors = [
        TagColor::Neutral,
        TagColor::Primary,
        TagColor::Secondary,
        TagColor::Accent,
        TagColor::Success,
        TagColor::Warning,
        TagColor::Error,
        TagColor::Info,
    ];

    for color in colors {
        assert!(
            color.as_str().starts_with("badge-"),
            "Tag color '{}' should start with 'badge-'",
            color.as_str()
        );
    }
}

#[test]
fn test_tag_sizes_use_badge_prefix() {
    let sizes = [TagSize::Small, TagSize::Medium, TagSize::Large];

    for size in sizes {
        assert!(
            size.as_str().starts_with("badge-"),
            "Tag size '{}' should start with 'badge-'",
            size.as_str()
        );
    }
}
