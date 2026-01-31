//! Unit tests for LoadingBar component

use super::style::{LoadingBarColor, LoadingBarSize};

#[test]
fn test_loading_bar_color_count() {
    let colors = [
        LoadingBarColor::Primary,
        LoadingBarColor::Secondary,
        LoadingBarColor::Accent,
        LoadingBarColor::Success,
        LoadingBarColor::Warning,
        LoadingBarColor::Error,
        LoadingBarColor::Info,
    ];

    assert_eq!(colors.len(), 7, "Should have 7 loading bar colors");
}

#[test]
fn test_loading_bar_size_count() {
    let sizes = [
        LoadingBarSize::XSmall,
        LoadingBarSize::Small,
        LoadingBarSize::Medium,
        LoadingBarSize::Large,
    ];

    assert_eq!(sizes.len(), 4, "Should have 4 loading bar sizes");
}

#[test]
fn test_loading_bar_color_css_classes() {
    assert_eq!(LoadingBarColor::Primary.as_str(), "bg-primary");
    assert_eq!(LoadingBarColor::Secondary.as_str(), "bg-secondary");
    assert_eq!(LoadingBarColor::Accent.as_str(), "bg-accent");
    assert_eq!(LoadingBarColor::Success.as_str(), "bg-success");
    assert_eq!(LoadingBarColor::Warning.as_str(), "bg-warning");
    assert_eq!(LoadingBarColor::Error.as_str(), "bg-error");
    assert_eq!(LoadingBarColor::Info.as_str(), "bg-info");
}

#[test]
fn test_loading_bar_size_css_classes() {
    assert_eq!(LoadingBarSize::XSmall.as_str(), "h-1");
    assert_eq!(LoadingBarSize::Small.as_str(), "h-2");
    assert_eq!(LoadingBarSize::Medium.as_str(), "h-3");
    assert_eq!(LoadingBarSize::Large.as_str(), "h-4");
}

#[test]
fn test_loading_bar_color_default() {
    assert_eq!(
        LoadingBarColor::default(),
        LoadingBarColor::Primary,
        "Default color should be Primary"
    );
}

#[test]
fn test_loading_bar_size_default() {
    assert_eq!(
        LoadingBarSize::default(),
        LoadingBarSize::Medium,
        "Default size should be Medium"
    );
}

#[test]
fn test_loading_bar_color_clone() {
    let color = LoadingBarColor::Success;
    let cloned = color.clone();
    assert_eq!(color, cloned, "Cloned color should equal original");
}

#[test]
fn test_loading_bar_size_clone() {
    let size = LoadingBarSize::Large;
    let cloned = size.clone();
    assert_eq!(size, cloned, "Cloned size should equal original");
}

#[test]
fn test_loading_bar_color_debug() {
    let color = LoadingBarColor::Warning;
    let debug_str = format!("{:?}", color);
    assert!(debug_str.contains("Warning"), "Debug output should contain color name");
}

#[test]
fn test_loading_bar_size_debug() {
    let size = LoadingBarSize::Small;
    let debug_str = format!("{:?}", size);
    assert!(debug_str.contains("Small"), "Debug output should contain size name");
}

#[test]
fn test_loading_bar_colors_use_bg_prefix() {
    let colors = [
        LoadingBarColor::Primary,
        LoadingBarColor::Secondary,
        LoadingBarColor::Accent,
        LoadingBarColor::Success,
        LoadingBarColor::Warning,
        LoadingBarColor::Error,
        LoadingBarColor::Info,
    ];

    for color in colors {
        assert!(
            color.as_str().starts_with("bg-"),
            "LoadingBar color '{}' should start with 'bg-'",
            color.as_str()
        );
    }
}

#[test]
fn test_loading_bar_sizes_are_ascending() {
    let extract_height = |class: &str| -> u32 {
        class.trim_start_matches("h-").parse().unwrap()
    };

    assert!(extract_height(LoadingBarSize::XSmall.as_str()) < extract_height(LoadingBarSize::Small.as_str()));
    assert!(extract_height(LoadingBarSize::Small.as_str()) < extract_height(LoadingBarSize::Medium.as_str()));
    assert!(extract_height(LoadingBarSize::Medium.as_str()) < extract_height(LoadingBarSize::Large.as_str()));
}
