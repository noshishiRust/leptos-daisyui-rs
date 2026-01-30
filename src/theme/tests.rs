//! Integration tests for the theming foundation
//!
//! These tests verify that all theme components work together correctly
//! and handle edge cases appropriately.

#[cfg(test)]
mod integration_tests {
    use crate::theme::*;

    #[test]
    fn test_theme_configuration_roundtrip() {
        // Create a complex theme configuration
        let mut theme = ThemeConfiguration::new("dark");

        let overrides = ThemeOverrides {
            colors: Some(ColorOverrides {
                primary: Some("#3b82f6".to_string()),
                secondary: Some("#8b5cf6".to_string()),
                accent: Some("#f59e0b".to_string()),
                ..Default::default()
            }),
            typography: Some(TypographyOverrides {
                font_family: Some(FontFamily {
                    primary: Some("Inter, sans-serif".to_string()),
                    ..Default::default()
                }),
                base_font_size: Some(1.0),
                ..Default::default()
            }),
            ..Default::default()
        };

        theme.overrides = Some(overrides);
        theme.metadata = Some(ThemeMetadata::new("Test Theme"));

        // Serialize to JSON
        let json = serde_json::to_string(&theme).unwrap();

        // Deserialize back
        let restored: ThemeConfiguration = serde_json::from_str(&json).unwrap();

        // Verify equality
        assert_eq!(theme.base_theme, restored.base_theme);
        assert!(restored.overrides.is_some());
        assert!(restored.metadata.is_some());
    }

    #[test]
    fn test_theme_context_lifecycle() {
        let config = ThemeConfiguration::new("light");
        let context = ThemeContext::new(config);

        // Test initial state
        assert_eq!(context.base_theme(), "light");

        // Test apply
        context.apply_theme(ThemeConfiguration::new("dark"));
        assert_eq!(context.base_theme(), "dark");

        // Test reset
        context.reset_theme();
        assert_eq!(context.base_theme(), "light");

        // Test set_base_theme
        context.set_base_theme("cupcake");
        assert_eq!(context.base_theme(), "cupcake");
    }

    #[test]
    fn test_theme_export_import_cycle() {
        let config = ThemeConfiguration::new("cyberpunk");
        let context = ThemeContext::new(config);

        // Export
        let exported = context.export_theme().unwrap();
        assert!(exported.contains("cyberpunk"));

        // Change theme
        context.set_base_theme("forest");
        assert_eq!(context.base_theme(), "forest");

        // Import back
        context.import_theme(&exported).unwrap();
        assert_eq!(context.base_theme(), "cyberpunk");
    }

    #[test]
    fn test_empty_overrides() {
        let theme = ThemeConfiguration {
            base_theme: "light".to_string(),
            overrides: None,
            metadata: None,
        };

        // Should serialize without overrides field
        let json = serde_json::to_string(&theme).unwrap();
        assert!(!json.contains("overrides"));
    }

    #[test]
    fn test_partial_overrides() {
        let overrides = ThemeOverrides {
            colors: Some(ColorOverrides {
                primary: Some("#ff0000".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };

        let theme = ThemeConfiguration::with_overrides("dark", overrides);
        let json = serde_json::to_string(&theme).unwrap();

        // Should have colors but not typography/borders/components
        assert!(json.contains("colors"));
        assert!(json.contains("primary"));
        assert!(!json.contains("typography"));
    }

    #[test]
    fn test_storage_key_constant() {
        assert_eq!(DEFAULT_THEME_STORAGE_KEY, "leptos-daisyui-theme-config");
    }

    #[test]
    fn test_save_load_roundtrip() {
        let theme = ThemeConfiguration::new("retro");

        // Save should succeed (or be no-op on non-WASM)
        let save_result = save_theme_config(&theme);
        #[cfg(target_arch = "wasm32")]
        assert!(save_result.is_ok());
        #[cfg(not(target_arch = "wasm32"))]
        assert!(save_result.is_ok()); // No-op on non-WASM

        // Load behavior depends on platform
        let load_result = load_theme_config();
        #[cfg(not(target_arch = "wasm32"))]
        assert!(load_result.is_err()); // Should error on non-WASM
    }

    #[test]
    fn test_hex_validation_edge_cases() {
        // Valid formats
        assert!(validate_hex_color("#000000"));
        assert!(validate_hex_color("#FFFFFF"));
        assert!(validate_hex_color("#abcdef"));
        assert!(validate_hex_color("#ABC"));
        assert!(validate_hex_color("123456")); // Without #

        // Invalid formats
        assert!(!validate_hex_color("#12"));
        assert!(!validate_hex_color("#1234"));
        assert!(!validate_hex_color("#12345"));
        assert!(!validate_hex_color("#1234567"));
        assert!(!validate_hex_color("#GGGGGG"));
        assert!(!validate_hex_color(""));
        assert!(!validate_hex_color("#"));
    }

    #[test]
    fn test_hex_to_oklch_edge_cases() {
        // Pure black
        let black = hex_to_oklch("#000000");
        assert!(black.contains("oklch("));
        assert!(black.contains("0.000"));

        // Pure white
        let white = hex_to_oklch("#FFFFFF");
        assert!(white.contains("oklch("));
        assert!(white.contains("1.000"));

        // Short format
        let short = hex_to_oklch("#F00"); // Red
        assert!(short.contains("oklch("));

        // Without # prefix
        let no_hash = hex_to_oklch("00FF00"); // Green
        assert!(no_hash.contains("oklch("));

        // Invalid input - should return fallback
        let invalid = hex_to_oklch("invalid");
        assert_eq!(invalid, "oklch(0.5 0 0)");

        let too_short = hex_to_oklch("#12");
        assert_eq!(too_short, "oklch(0.5 0 0)");
    }

    #[test]
    fn test_theme_list_completeness() {
        let themes = get_available_themes();

        // Should have all major daisyUI themes
        assert!(themes.contains(&"light"));
        assert!(themes.contains(&"dark"));
        assert!(themes.contains(&"cupcake"));
        assert!(themes.contains(&"cyberpunk"));
        assert!(themes.contains(&"retro"));
        assert!(themes.contains(&"dracula"));
        assert!(themes.contains(&"nord"));

        // Should have reasonable count (daisyUI v5 has 32 themes)
        assert!(themes.len() >= 30);
    }

    #[test]
    fn test_is_valid_theme_comprehensive() {
        // All themes from the list should be valid
        for theme in get_available_themes() {
            assert!(is_valid_theme_name(theme), "Theme '{}' should be valid", theme);
        }

        // Invalid themes
        assert!(!is_valid_theme_name(""));
        assert!(!is_valid_theme_name("invalid-theme"));
        assert!(!is_valid_theme_name("DARK")); // Case sensitive
        assert!(!is_valid_theme_name("light ")); // Trailing space
    }

    #[test]
    fn test_metadata_builder_chain() {
        let metadata = ThemeMetadata::new("My Theme")
            .with_description("A custom theme")
            .with_author("Test User")
            .with_tags(vec!["dark".to_string(), "modern".to_string()]);

        assert_eq!(metadata.name.unwrap(), "My Theme");
        assert_eq!(metadata.description.unwrap(), "A custom theme");
        assert_eq!(metadata.author.unwrap(), "Test User");
        assert_eq!(metadata.tags.unwrap().len(), 2);
    }

    #[test]
    fn test_color_overrides_defaults() {
        let colors = ColorOverrides::default();

        // All fields should be None
        assert!(colors.primary.is_none());
        assert!(colors.secondary.is_none());
        assert!(colors.accent.is_none());
        assert!(colors.neutral.is_none());
        assert!(colors.base_100.is_none());
        assert!(colors.info.is_none());
        assert!(colors.success.is_none());
        assert!(colors.warning.is_none());
        assert!(colors.error.is_none());
    }

    #[test]
    fn test_typography_overrides_defaults() {
        let typography = TypographyOverrides::default();

        assert!(typography.font_family.is_none());
        assert!(typography.font_scale.is_none());
        assert!(typography.base_font_size.is_none());
        assert!(typography.line_height.is_none());
        assert!(typography.letter_spacing.is_none());
    }

    #[test]
    fn test_border_overrides_defaults() {
        let borders = BorderOverrides::default();

        assert!(borders.border_width.is_none());
        assert!(borders.radius_sm.is_none());
        assert!(borders.radius_md.is_none());
        assert!(borders.radius_lg.is_none());
        assert!(borders.spacing_scale.is_none());
    }

    #[test]
    fn test_component_overrides_defaults() {
        let components = ComponentOverrides::default();

        assert!(components.button.is_none());
        assert!(components.card.is_none());
        assert!(components.input.is_none());
    }

    #[test]
    fn test_theme_configuration_equality() {
        let theme1 = ThemeConfiguration::new("dark");
        let theme2 = ThemeConfiguration::new("dark");
        let theme3 = ThemeConfiguration::new("light");

        assert_eq!(theme1, theme2);
        assert_ne!(theme1, theme3);
    }

    #[test]
    fn test_css_injection_non_wasm() {
        let theme = ThemeConfiguration::new("dark");

        // Should not error on non-WASM
        assert!(inject_css_variables(&theme).is_ok());
        assert!(clear_css_variables().is_ok());
    }
}
