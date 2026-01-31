use super::*;

// BadgeStyle tests
#[test]
fn test_badge_style_default() {
    let style = BadgeStyle::default();
    assert_eq!(style.as_str(), "");
}

#[test]
fn test_badge_style_outline() {
    let style = BadgeStyle::Outline;
    assert_eq!(style.as_str(), "badge-outline");
}

#[test]
fn test_badge_style_dash() {
    let style = BadgeStyle::Dash;
    assert_eq!(style.as_str(), "badge-dash");
}

#[test]
fn test_badge_style_soft() {
    let style = BadgeStyle::Soft;
    assert_eq!(style.as_str(), "badge-soft");
}

#[test]
fn test_badge_style_ghost() {
    let style = BadgeStyle::Ghost;
    assert_eq!(style.as_str(), "badge-ghost");
}

#[test]
fn test_badge_style_clone() {
    let style1 = BadgeStyle::Outline;
    let style2 = style1.clone();
    assert_eq!(style1.as_str(), style2.as_str());
}

#[test]
fn test_badge_style_debug() {
    let style = BadgeStyle::Dash;
    assert!(format!("{:?}", style).contains("Dash"));
}

// BadgeColor tests
#[test]
fn test_badge_color_default() {
    let color = BadgeColor::default();
    assert_eq!(color.as_str(), "");
}

#[test]
fn test_badge_color_neutral() {
    let color = BadgeColor::Neutral;
    assert_eq!(color.as_str(), "badge-neutral");
}

#[test]
fn test_badge_color_primary() {
    let color = BadgeColor::Primary;
    assert_eq!(color.as_str(), "badge-primary");
}

#[test]
fn test_badge_color_secondary() {
    let color = BadgeColor::Secondary;
    assert_eq!(color.as_str(), "badge-secondary");
}

#[test]
fn test_badge_color_accent() {
    let color = BadgeColor::Accent;
    assert_eq!(color.as_str(), "badge-accent");
}

#[test]
fn test_badge_color_info() {
    let color = BadgeColor::Info;
    assert_eq!(color.as_str(), "badge-info");
}

#[test]
fn test_badge_color_success() {
    let color = BadgeColor::Success;
    assert_eq!(color.as_str(), "badge-success");
}

#[test]
fn test_badge_color_warning() {
    let color = BadgeColor::Warning;
    assert_eq!(color.as_str(), "badge-warning");
}

#[test]
fn test_badge_color_error() {
    let color = BadgeColor::Error;
    assert_eq!(color.as_str(), "badge-error");
}

#[test]
fn test_badge_color_clone() {
    let color1 = BadgeColor::Success;
    let color2 = color1.clone();
    assert_eq!(color1.as_str(), color2.as_str());
}

#[test]
fn test_badge_color_debug() {
    let color = BadgeColor::Warning;
    assert!(format!("{:?}", color).contains("Warning"));
}

// BadgeSize tests
#[test]
fn test_badge_size_default() {
    let size = BadgeSize::default();
    assert_eq!(size.as_str(), "badge-md");
}

#[test]
fn test_badge_size_xs() {
    let size = BadgeSize::Xs;
    assert_eq!(size.as_str(), "badge-xs");
}

#[test]
fn test_badge_size_sm() {
    let size = BadgeSize::Sm;
    assert_eq!(size.as_str(), "badge-sm");
}

#[test]
fn test_badge_size_md() {
    let size = BadgeSize::Md;
    assert_eq!(size.as_str(), "badge-md");
}

#[test]
fn test_badge_size_lg() {
    let size = BadgeSize::Lg;
    assert_eq!(size.as_str(), "badge-lg");
}

#[test]
fn test_badge_size_xl() {
    let size = BadgeSize::Xl;
    assert_eq!(size.as_str(), "badge-xl");
}

#[test]
fn test_badge_size_clone() {
    let size1 = BadgeSize::Lg;
    let size2 = size1.clone();
    assert_eq!(size1.as_str(), size2.as_str());
}

#[test]
fn test_badge_size_debug() {
    let size = BadgeSize::Xl;
    assert!(format!("{:?}", size).contains("Xl"));
}

// Comprehensive enum variant coverage tests
#[test]
fn test_all_badge_styles_return_valid_classes() {
    let styles = vec![
        (BadgeStyle::Default, ""),
        (BadgeStyle::Outline, "badge-outline"),
        (BadgeStyle::Dash, "badge-dash"),
        (BadgeStyle::Soft, "badge-soft"),
        (BadgeStyle::Ghost, "badge-ghost"),
    ];

    for (style, expected) in styles {
        assert_eq!(style.as_str(), expected);
    }
}

#[test]
fn test_all_badge_colors_return_valid_classes() {
    let colors = vec![
        (BadgeColor::Default, ""),
        (BadgeColor::Neutral, "badge-neutral"),
        (BadgeColor::Primary, "badge-primary"),
        (BadgeColor::Secondary, "badge-secondary"),
        (BadgeColor::Accent, "badge-accent"),
        (BadgeColor::Info, "badge-info"),
        (BadgeColor::Success, "badge-success"),
        (BadgeColor::Warning, "badge-warning"),
        (BadgeColor::Error, "badge-error"),
    ];

    for (color, expected) in colors {
        assert_eq!(color.as_str(), expected);
    }
}

#[test]
fn test_all_badge_sizes_return_valid_classes() {
    let sizes = vec![
        (BadgeSize::Xs, "badge-xs"),
        (BadgeSize::Sm, "badge-sm"),
        (BadgeSize::Md, "badge-md"),
        (BadgeSize::Lg, "badge-lg"),
        (BadgeSize::Xl, "badge-xl"),
    ];

    for (size, expected) in sizes {
        assert_eq!(size.as_str(), expected);
    }
}
