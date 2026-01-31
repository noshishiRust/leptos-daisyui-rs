use crate::theme::{ColorOverrides, ThemeConfiguration, ThemeMetadata, ThemeOverrides};

/// Predefined preset themes with custom configurations
pub fn get_preset_themes() -> Vec<(String, ThemeConfiguration)> {
    vec![
        (
            "Ocean Breeze".to_string(),
            ThemeConfiguration {
                base_theme: "aqua".to_string(),
                overrides: Some(ThemeOverrides {
                    colors: Some(ColorOverrides {
                        primary: Some("oklch(0.65 0.15 230.0)".to_string()), // Ocean blue
                        secondary: Some("oklch(0.55 0.12 210.0)".to_string()), // Deep blue
                        accent: Some("oklch(0.70 0.18 180.0)".to_string()),  // Turquoise
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                metadata: Some(ThemeMetadata {
                    name: Some("Ocean Breeze".to_string()),
                    description: Some("Calming ocean-inspired color palette".to_string()),
                    author: Some("leptos-daisyui-rs".to_string()),
                    version: Some("1.0.0".to_string()),
                    tags: Some(vec![
                        "blue".to_string(),
                        "ocean".to_string(),
                        "calm".to_string(),
                    ]),
                    ..Default::default()
                }),
            },
        ),
        (
            "Forest Harmony".to_string(),
            ThemeConfiguration {
                base_theme: "forest".to_string(),
                overrides: Some(ThemeOverrides {
                    colors: Some(ColorOverrides {
                        primary: Some("oklch(0.55 0.15 150.0)".to_string()), // Forest green
                        secondary: Some("oklch(0.50 0.12 130.0)".to_string()), // Dark green
                        accent: Some("oklch(0.65 0.18 100.0)".to_string()),  // Lime
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                metadata: Some(ThemeMetadata {
                    name: Some("Forest Harmony".to_string()),
                    description: Some("Natural forest-inspired green tones".to_string()),
                    author: Some("leptos-daisyui-rs".to_string()),
                    version: Some("1.0.0".to_string()),
                    tags: Some(vec![
                        "green".to_string(),
                        "nature".to_string(),
                        "forest".to_string(),
                    ]),
                    ..Default::default()
                }),
            },
        ),
        (
            "Sunset Glow".to_string(),
            ThemeConfiguration {
                base_theme: "sunset".to_string(),
                overrides: Some(ThemeOverrides {
                    colors: Some(ColorOverrides {
                        primary: Some("oklch(0.65 0.20 40.0)".to_string()), // Warm orange
                        secondary: Some("oklch(0.55 0.18 20.0)".to_string()), // Deep orange
                        accent: Some("oklch(0.70 0.22 60.0)".to_string()),  // Golden yellow
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                metadata: Some(ThemeMetadata {
                    name: Some("Sunset Glow".to_string()),
                    description: Some("Warm sunset-inspired orange and gold palette".to_string()),
                    author: Some("leptos-daisyui-rs".to_string()),
                    version: Some("1.0.0".to_string()),
                    tags: Some(vec![
                        "orange".to_string(),
                        "warm".to_string(),
                        "sunset".to_string(),
                    ]),
                    ..Default::default()
                }),
            },
        ),
        (
            "Midnight Sky".to_string(),
            ThemeConfiguration {
                base_theme: "night".to_string(),
                overrides: Some(ThemeOverrides {
                    colors: Some(ColorOverrides {
                        primary: Some("oklch(0.50 0.15 270.0)".to_string()), // Deep purple
                        secondary: Some("oklch(0.45 0.12 250.0)".to_string()), // Dark violet
                        accent: Some("oklch(0.60 0.18 290.0)".to_string()),  // Bright purple
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                metadata: Some(ThemeMetadata {
                    name: Some("Midnight Sky".to_string()),
                    description: Some("Deep night sky with purple accents".to_string()),
                    author: Some("leptos-daisyui-rs".to_string()),
                    version: Some("1.0.0".to_string()),
                    tags: Some(vec![
                        "purple".to_string(),
                        "dark".to_string(),
                        "night".to_string(),
                    ]),
                    ..Default::default()
                }),
            },
        ),
        (
            "Cherry Blossom".to_string(),
            ThemeConfiguration {
                base_theme: "valentine".to_string(),
                overrides: Some(ThemeOverrides {
                    colors: Some(ColorOverrides {
                        primary: Some("oklch(0.70 0.15 350.0)".to_string()), // Soft pink
                        secondary: Some("oklch(0.65 0.12 340.0)".to_string()), // Rose
                        accent: Some("oklch(0.75 0.10 330.0)".to_string()),  // Light pink
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                metadata: Some(ThemeMetadata {
                    name: Some("Cherry Blossom".to_string()),
                    description: Some("Delicate cherry blossom pink theme".to_string()),
                    author: Some("leptos-daisyui-rs".to_string()),
                    version: Some("1.0.0".to_string()),
                    tags: Some(vec![
                        "pink".to_string(),
                        "soft".to_string(),
                        "spring".to_string(),
                    ]),
                    ..Default::default()
                }),
            },
        ),
        (
            "Corporate Blue".to_string(),
            ThemeConfiguration {
                base_theme: "corporate".to_string(),
                overrides: Some(ThemeOverrides {
                    colors: Some(ColorOverrides {
                        primary: Some("oklch(0.55 0.12 240.0)".to_string()), // Professional blue
                        secondary: Some("oklch(0.45 0.08 220.0)".to_string()), // Navy
                        neutral: Some("oklch(0.30 0.02 250.0)".to_string()), // Dark gray-blue
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                metadata: Some(ThemeMetadata {
                    name: Some("Corporate Blue".to_string()),
                    description: Some("Professional corporate blue theme".to_string()),
                    author: Some("leptos-daisyui-rs".to_string()),
                    version: Some("1.0.0".to_string()),
                    tags: Some(vec![
                        "blue".to_string(),
                        "professional".to_string(),
                        "corporate".to_string(),
                    ]),
                    ..Default::default()
                }),
            },
        ),
    ]
}
