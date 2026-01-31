//! Unit tests for Icon component

use super::style::IconSize;

#[test]
fn test_icon_size_count() {
    let sizes = [
        IconSize::XSmall,
        IconSize::Small,
        IconSize::Medium,
        IconSize::Large,
        IconSize::XLarge,
    ];

    assert_eq!(sizes.len(), 5, "Should have 5 icon sizes");
}

#[test]
fn test_icon_size_px_values() {
    assert_eq!(IconSize::XSmall.as_px(), 16, "XSmall should be 16px");
    assert_eq!(IconSize::Small.as_px(), 20, "Small should be 20px");
    assert_eq!(IconSize::Medium.as_px(), 24, "Medium should be 24px");
    assert_eq!(IconSize::Large.as_px(), 32, "Large should be 32px");
    assert_eq!(IconSize::XLarge.as_px(), 48, "XLarge should be 48px");
}

#[test]
fn test_icon_size_css_classes() {
    assert_eq!(
        IconSize::XSmall.as_str(),
        "w-4 h-4",
        "XSmall should use w-4 h-4"
    );
    assert_eq!(
        IconSize::Small.as_str(),
        "w-5 h-5",
        "Small should use w-5 h-5"
    );
    assert_eq!(
        IconSize::Medium.as_str(),
        "w-6 h-6",
        "Medium should use w-6 h-6"
    );
    assert_eq!(
        IconSize::Large.as_str(),
        "w-8 h-8",
        "Large should use w-8 h-8"
    );
    assert_eq!(
        IconSize::XLarge.as_str(),
        "w-12 h-12",
        "XLarge should use w-12 h-12"
    );
}

#[test]
fn test_icon_size_default() {
    assert_eq!(
        IconSize::default(),
        IconSize::Medium,
        "Default size should be Medium"
    );
}

#[test]
fn test_icon_size_clone() {
    let size = IconSize::Large;
    let cloned = size.clone();
    assert_eq!(size, cloned, "Cloned size should equal original");
}

#[test]
fn test_icon_size_debug() {
    let size = IconSize::Small;
    let debug_str = format!("{:?}", size);
    assert!(
        debug_str.contains("Small"),
        "Debug output should contain size name"
    );
}

#[test]
fn test_icon_sizes_are_ascending() {
    assert!(IconSize::XSmall.as_px() < IconSize::Small.as_px());
    assert!(IconSize::Small.as_px() < IconSize::Medium.as_px());
    assert!(IconSize::Medium.as_px() < IconSize::Large.as_px());
    assert!(IconSize::Large.as_px() < IconSize::XLarge.as_px());
}

#[test]
fn test_icon_size_css_classes_match_px() {
    // Tailwind's w-4 = 1rem = 16px (at default 16px base)
    assert_eq!(IconSize::XSmall.as_px(), 16);
    assert!(IconSize::XSmall.as_str().contains("w-4"));

    // w-12 = 3rem = 48px
    assert_eq!(IconSize::XLarge.as_px(), 48);
    assert!(IconSize::XLarge.as_str().contains("w-12"));
}
