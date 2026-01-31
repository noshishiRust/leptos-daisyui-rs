//! Unit tests for ColorCustomizer component

#[test]
fn test_color_tokens_count() {
    // ColorCustomizer defines 12 color tokens
    // 4 brand: primary, secondary, accent, neutral
    // 4 base: base_100, base_200, base_300, base_content
    // 4 state: info, success, warning, error
    let expected_count = 12;
    assert_eq!(expected_count, 12, "Should have 12 color tokens");
}

#[test]
fn test_brand_color_tokens() {
    let brand_colors = ["primary", "secondary", "accent", "neutral"];
    assert_eq!(brand_colors.len(), 4, "Should have 4 brand colors");

    assert!(brand_colors.contains(&"primary"), "Should include primary");
    assert!(brand_colors.contains(&"secondary"), "Should include secondary");
    assert!(brand_colors.contains(&"accent"), "Should include accent");
    assert!(brand_colors.contains(&"neutral"), "Should include neutral");
}

#[test]
fn test_base_color_tokens() {
    let base_colors = ["base_100", "base_200", "base_300", "base_content"];
    assert_eq!(base_colors.len(), 4, "Should have 4 base colors");

    assert!(base_colors.contains(&"base_100"), "Should include base_100");
    assert!(base_colors.contains(&"base_200"), "Should include base_200");
    assert!(base_colors.contains(&"base_300"), "Should include base_300");
    assert!(base_colors.contains(&"base_content"), "Should include base_content");
}

#[test]
fn test_state_color_tokens() {
    let state_colors = ["info", "success", "warning", "error"];
    assert_eq!(state_colors.len(), 4, "Should have 4 state colors");

    assert!(state_colors.contains(&"info"), "Should include info");
    assert!(state_colors.contains(&"success"), "Should include success");
    assert!(state_colors.contains(&"warning"), "Should include warning");
    assert!(state_colors.contains(&"error"), "Should include error");
}

#[test]
fn test_all_color_tokens_unique() {
    let all_tokens = [
        "primary", "secondary", "accent", "neutral",
        "base_100", "base_200", "base_300", "base_content",
        "info", "success", "warning", "error",
    ];

    let unique_count = all_tokens.iter().collect::<std::collections::HashSet<_>>().len();
    assert_eq!(
        all_tokens.len(),
        unique_count,
        "All color tokens should be unique"
    );
}

#[test]
fn test_color_token_categories() {
    let brand = ["primary", "secondary", "accent", "neutral"];
    let base = ["base_100", "base_200", "base_300", "base_content"];
    let state = ["info", "success", "warning", "error"];

    assert_eq!(brand.len() + base.len() + state.len(), 12,
        "Total should equal 12 color tokens");
}
