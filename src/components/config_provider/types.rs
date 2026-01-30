//! Configuration types for the ConfigProvider component

use serde::{Deserialize, Serialize};

/// Text direction for internationalization
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum TextDirection {
    /// Left-to-right (default for most languages)
    #[default]
    Ltr,
    /// Right-to-left (for Arabic, Hebrew, etc.)
    Rtl,
}

impl TextDirection {
    /// Returns the HTML dir attribute value
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Ltr => "ltr",
            Self::Rtl => "rtl",
        }
    }
}

/// Application configuration
///
/// Holds global configuration settings for the entire application including
/// locale, text direction, and other app-wide preferences.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppConfig {
    /// Current locale (e.g., "en-US", "ar-SA", "fr-FR")
    pub locale: String,

    /// Text direction (LTR or RTL)
    pub direction: TextDirection,

    /// Enable reduced motion for accessibility
    pub reduce_motion: bool,

    /// Enable high contrast mode for accessibility
    pub high_contrast: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            locale: "en-US".to_string(),
            direction: TextDirection::Ltr,
            reduce_motion: false,
            high_contrast: false,
        }
    }
}

impl AppConfig {
    /// Create a new AppConfig with a specific locale
    pub fn new(locale: impl Into<String>) -> Self {
        let locale_str = locale.into();
        let direction = if locale_str.starts_with("ar") || locale_str.starts_with("he") {
            TextDirection::Rtl
        } else {
            TextDirection::Ltr
        };

        Self {
            locale: locale_str,
            direction,
            reduce_motion: false,
            high_contrast: false,
        }
    }

    /// Set the text direction
    pub fn with_direction(mut self, direction: TextDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Enable or disable reduced motion
    pub fn with_reduce_motion(mut self, reduce_motion: bool) -> Self {
        self.reduce_motion = reduce_motion;
        self
    }

    /// Enable or disable high contrast mode
    pub fn with_high_contrast(mut self, high_contrast: bool) -> Self {
        self.high_contrast = high_contrast;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_direction_default() {
        assert_eq!(TextDirection::default(), TextDirection::Ltr);
        assert_eq!(TextDirection::Ltr.as_str(), "ltr");
        assert_eq!(TextDirection::Rtl.as_str(), "rtl");
    }

    #[test]
    fn test_app_config_default() {
        let config = AppConfig::default();
        assert_eq!(config.locale, "en-US");
        assert_eq!(config.direction, TextDirection::Ltr);
        assert!(!config.reduce_motion);
        assert!(!config.high_contrast);
    }

    #[test]
    fn test_app_config_new() {
        let config = AppConfig::new("en-US");
        assert_eq!(config.direction, TextDirection::Ltr);

        let config_ar = AppConfig::new("ar-SA");
        assert_eq!(config_ar.direction, TextDirection::Rtl);

        let config_he = AppConfig::new("he-IL");
        assert_eq!(config_he.direction, TextDirection::Rtl);
    }

    #[test]
    fn test_app_config_builder() {
        let config = AppConfig::new("fr-FR")
            .with_direction(TextDirection::Ltr)
            .with_reduce_motion(true)
            .with_high_contrast(true);

        assert_eq!(config.locale, "fr-FR");
        assert_eq!(config.direction, TextDirection::Ltr);
        assert!(config.reduce_motion);
        assert!(config.high_contrast);
    }
}
