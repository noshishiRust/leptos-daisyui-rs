//! Theme data structures for runtime theming
//!
//! This module contains all the data structures needed for the advanced theming system,
//! including theme configuration, overrides, and metadata.

use serde::{Deserialize, Serialize};

/// Main theme configuration structure
///
/// This is the primary structure that holds all theme data including the base theme,
/// any overrides, and metadata about the theme.
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ThemeConfiguration {
    /// Base daisyUI theme name (e.g., "light", "dark", "cupcake", "cyberpunk")
    pub base_theme: String,

    /// Optional overrides to customize the base theme
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides: Option<ThemeOverrides>,

    /// Optional metadata about the theme
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ThemeMetadata>,
}

impl ThemeConfiguration {
    /// Create a new theme configuration with just a base theme
    pub fn new(base_theme: impl Into<String>) -> Self {
        Self {
            base_theme: base_theme.into(),
            overrides: None,
            metadata: None,
        }
    }

    /// Create a theme configuration with overrides
    pub fn with_overrides(base_theme: impl Into<String>, overrides: ThemeOverrides) -> Self {
        Self {
            base_theme: base_theme.into(),
            overrides: Some(overrides),
            metadata: None,
        }
    }

    /// Add metadata to the theme configuration
    pub fn with_metadata(mut self, metadata: ThemeMetadata) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

/// Theme overrides for customizing any aspect of the theme
///
/// All fields are optional to allow partial customization. Each field represents
/// a different category of theme customization.
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ThemeOverrides {
    /// Color overrides (primary, secondary, accent, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colors: Option<ColorOverrides>,

    /// Typography overrides (font families, scales, weights)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typography: Option<TypographyOverrides>,

    /// Border and spacing overrides
    #[serde(skip_serializing_if = "Option::is_none")]
    pub borders: Option<BorderOverrides>,

    /// Component-specific overrides
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<ComponentOverrides>,
}

/// Color overrides for theme colors
///
/// All colors should be in Oklahoma LCH format (e.g., "oklch(0.5 0.2 180)")
/// or hex format (e.g., "#3b82f6") which will be converted to Oklahoma LCH.
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ColorOverrides {
    /// Primary color
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<String>,

    /// Secondary color
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary: Option<String>,

    /// Accent color
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent: Option<String>,

    /// Neutral color
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neutral: Option<String>,

    /// Base background color
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_100: Option<String>,

    /// Secondary background color
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_200: Option<String>,

    /// Tertiary background color
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_300: Option<String>,

    /// Base content (text) color
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_content: Option<String>,

    /// Info color
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<String>,

    /// Success color
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<String>,

    /// Warning color
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warning: Option<String>,

    /// Error color
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Typography overrides for font customization
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TypographyOverrides {
    /// Font family overrides
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_family: Option<FontFamily>,

    /// Font scale/sizing overrides
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_scale: Option<FontScale>,

    /// Base font size in rem
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_font_size: Option<f32>,

    /// Line height multiplier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_height: Option<f32>,

    /// Letter spacing in em
    #[serde(skip_serializing_if = "Option::is_none")]
    pub letter_spacing: Option<f32>,
}

/// Font family configuration
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FontFamily {
    /// Primary font family (e.g., "Inter, sans-serif")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<String>,

    /// Secondary/heading font family
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<String>,

    /// Monospace font family for code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monospace: Option<String>,
}

/// Font scale configuration (type scale)
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FontScale {
    /// Extra small text size (rem)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xs: Option<f32>,

    /// Small text size (rem)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sm: Option<f32>,

    /// Base text size (rem)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<f32>,

    /// Large text size (rem)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lg: Option<f32>,

    /// Extra large text size (rem)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xl: Option<f32>,

    /// 2xl text size (rem)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xl2: Option<f32>,

    /// 3xl text size (rem)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xl3: Option<f32>,
}

/// Border and spacing overrides
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BorderOverrides {
    /// Border width in pixels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_width: Option<f32>,

    /// Border radius for small elements (rem)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius_sm: Option<f32>,

    /// Border radius for medium elements (rem)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius_md: Option<f32>,

    /// Border radius for large elements (rem)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius_lg: Option<f32>,

    /// Spacing scale multiplier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spacing_scale: Option<f32>,
}

/// Component-specific style overrides
///
/// Allows customization of individual components beyond theme-wide changes.
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ComponentOverrides {
    /// Button component overrides
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button: Option<ButtonOverrides>,

    /// Card component overrides
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CardOverrides>,

    /// Input component overrides
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<InputOverrides>,
}

/// Button-specific overrides
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ButtonOverrides {
    /// Border radius override (rem)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_radius: Option<f32>,

    /// Padding override (rem)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<String>,

    /// Font weight
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_weight: Option<u16>,

    /// Text transform (e.g., "uppercase", "capitalize", "none")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_transform: Option<String>,
}

/// Card-specific overrides
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CardOverrides {
    /// Border radius override (rem)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_radius: Option<f32>,

    /// Shadow override (CSS box-shadow value)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow: Option<String>,

    /// Background opacity (0.0-1.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_opacity: Option<f32>,
}

/// Input-specific overrides
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InputOverrides {
    /// Border radius override (rem)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_radius: Option<f32>,

    /// Border width override (px)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_width: Option<f32>,

    /// Focus ring width (px)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_ring_width: Option<f32>,

    /// Focus ring color
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_ring_color: Option<String>,
}

/// Theme metadata
///
/// Stores information about the theme for organizational and sharing purposes.
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ThemeMetadata {
    /// Theme name/title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Theme description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Theme author
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,

    /// Theme version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// Theme tags for categorization
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,

    /// Creation timestamp (ISO 8601)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,

    /// Last modified timestamp (ISO 8601)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl ThemeMetadata {
    /// Create new theme metadata with a name
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            ..Default::default()
        }
    }

    /// Add a description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Add an author
    pub fn with_author(mut self, author: impl Into<String>) -> Self {
        self.author = Some(author.into());
        self
    }

    /// Add tags
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = Some(tags);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_configuration_new() {
        let theme = ThemeConfiguration::new("dark");
        assert_eq!(theme.base_theme, "dark");
        assert!(theme.overrides.is_none());
        assert!(theme.metadata.is_none());
    }

    #[test]
    fn test_theme_configuration_with_overrides() {
        let overrides = ThemeOverrides::default();
        let theme = ThemeConfiguration::with_overrides("light", overrides);
        assert_eq!(theme.base_theme, "light");
        assert!(theme.overrides.is_some());
    }

    #[test]
    fn test_theme_configuration_serialization() {
        let theme = ThemeConfiguration::new("cupcake");
        let json = serde_json::to_string(&theme).unwrap();
        assert!(json.contains("cupcake"));

        let deserialized: ThemeConfiguration = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.base_theme, "cupcake");
    }

    #[test]
    fn test_theme_metadata_builder() {
        let metadata = ThemeMetadata::new("My Theme")
            .with_description("A beautiful theme")
            .with_author("John Doe")
            .with_tags(vec!["dark".to_string(), "modern".to_string()]);

        assert_eq!(metadata.name, Some("My Theme".to_string()));
        assert_eq!(metadata.description, Some("A beautiful theme".to_string()));
        assert_eq!(metadata.author, Some("John Doe".to_string()));
        assert_eq!(metadata.tags.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_color_overrides_partial() {
        let colors = ColorOverrides {
            primary: Some("oklch(0.5 0.2 180)".to_string()),
            secondary: Some("#3b82f6".to_string()),
            ..Default::default()
        };

        assert!(colors.primary.is_some());
        assert!(colors.secondary.is_some());
        assert!(colors.accent.is_none());
    }
}
