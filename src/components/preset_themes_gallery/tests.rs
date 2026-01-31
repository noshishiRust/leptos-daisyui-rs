//! Unit tests for PresetThemesGallery component

use super::presets::get_preset_themes;

#[test]
fn test_preset_themes_count() {
    let themes = get_preset_themes();
    assert_eq!(themes.len(), 6, "Should have 6 preset themes");
}

#[test]
fn test_preset_themes_have_names() {
    let themes = get_preset_themes();

    for (id, config) in themes {
        assert!(!id.is_empty(), "Theme ID should not be empty");

        if let Some(metadata) = config.metadata
            && let Some(name) = metadata.name
        {
            assert!(!name.is_empty(), "Theme name should not be empty");
        }
    }
}

#[test]
fn test_preset_themes_include_popular_themes() {
    let themes = get_preset_themes();
    let theme_names: Vec<String> = themes.iter().map(|(id, _)| id.clone()).collect();

    assert!(
        theme_names.contains(&"Ocean Breeze".to_string()),
        "Should include Ocean Breeze"
    );
    assert!(
        theme_names.contains(&"Forest Harmony".to_string()),
        "Should include Forest Harmony"
    );
    assert!(
        theme_names.contains(&"Sunset Glow".to_string()),
        "Should include Sunset Glow"
    );
}

#[test]
fn test_preset_themes_have_base_themes() {
    let themes = get_preset_themes();

    for (_, config) in themes {
        assert!(
            !config.base_theme.is_empty(),
            "Each preset should have a base theme"
        );
    }
}

#[test]
fn test_preset_themes_no_duplicates() {
    let themes = get_preset_themes();
    let theme_ids: Vec<String> = themes.iter().map(|(id, _)| id.clone()).collect();
    let unique_count = theme_ids
        .iter()
        .collect::<std::collections::HashSet<_>>()
        .len();

    assert_eq!(
        theme_ids.len(),
        unique_count,
        "Preset theme IDs should be unique"
    );
}

#[test]
fn test_preset_themes_have_metadata() {
    let themes = get_preset_themes();

    for (id, config) in themes {
        assert!(
            config.metadata.is_some(),
            "Theme '{}' should have metadata",
            id
        );
    }
}

#[test]
fn test_preset_themes_have_descriptions() {
    let themes = get_preset_themes();

    for (id, config) in themes {
        if let Some(metadata) = config.metadata {
            assert!(
                metadata.description.is_some(),
                "Theme '{}' should have a description",
                id
            );
        }
    }
}

#[test]
fn test_preset_themes_have_color_overrides() {
    let themes = get_preset_themes();

    for (id, config) in themes {
        if let Some(overrides) = config.overrides {
            assert!(
                overrides.colors.is_some(),
                "Theme '{}' should have color overrides",
                id
            );
        }
    }
}

#[test]
fn test_ocean_breeze_theme_exists() {
    let themes = get_preset_themes();
    let ocean_breeze = themes.iter().find(|(id, _)| id == "Ocean Breeze");

    assert!(ocean_breeze.is_some(), "Ocean Breeze preset should exist");

    if let Some((_, config)) = ocean_breeze {
        assert_eq!(
            config.base_theme, "aqua",
            "Ocean Breeze should use aqua base theme"
        );
    }
}

#[test]
fn test_forest_harmony_theme_exists() {
    let themes = get_preset_themes();
    let forest = themes.iter().find(|(id, _)| id == "Forest Harmony");

    assert!(forest.is_some(), "Forest Harmony preset should exist");

    if let Some((_, config)) = forest {
        assert_eq!(
            config.base_theme, "forest",
            "Forest Harmony should use forest base theme"
        );
    }
}

#[test]
fn test_midnight_sky_theme_exists() {
    let themes = get_preset_themes();
    let midnight = themes.iter().find(|(id, _)| id == "Midnight Sky");

    assert!(midnight.is_some(), "Midnight Sky preset should exist");

    if let Some((_, config)) = midnight {
        assert_eq!(
            config.base_theme, "night",
            "Midnight Sky should use night base theme"
        );
    }
}
