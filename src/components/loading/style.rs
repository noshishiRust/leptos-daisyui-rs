/// Style definitions for the Loading component.
#[derive(Clone, Debug, Default)]
pub enum LoadingColor {
    /// Default theme color
    #[default]
    Default,

    /// Neutral gray color scheme
    Neutral,

    /// Primary theme color
    Primary,

    /// Secondary theme color
    Secondary,

    /// Accent theme color
    Accent,

    /// Information/info theme color (typically blue)
    Info,

    /// Success theme color (typically green)
    Success,

    /// Warning theme color (typically yellow/orange)
    Warning,

    /// Error theme color (typically red)
    Error,
}

impl LoadingColor {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            LoadingColor::Default => "",
            LoadingColor::Neutral => "loading-neutral",
            LoadingColor::Primary => "loading-primary",
            LoadingColor::Secondary => "loading-secondary",
            LoadingColor::Accent => "loading-accent",
            LoadingColor::Info => "loading-info",
            LoadingColor::Success => "loading-success",
            LoadingColor::Warning => "loading-warning",
            LoadingColor::Error => "loading-error",
        }
    }
}

/// Animation type variations for loading indicators.
#[derive(Clone, Debug, Default)]
pub enum LoadingType {
    /// Classic spinning circle animation (default)
    #[default]
    Spinner,

    /// Three bouncing dots animation
    Dots,

    /// Rotating ring with gap animation
    Ring,

    /// Bouncing ball animation
    Ball,

    /// Animated vertical bars
    Bars,

    /// Infinity symbol animation
    Infinity,
}

impl LoadingType {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            LoadingType::Spinner => "loading-spinner",
            LoadingType::Dots => "loading-dots",
            LoadingType::Ring => "loading-ring",
            LoadingType::Ball => "loading-ball",
            LoadingType::Bars => "loading-bars",
            LoadingType::Infinity => "loading-infinity",
        }
    }
}

/// Size variations for loading indicators.
#[derive(Clone, Debug, Default)]
pub enum LoadingSize {
    /// Standard/medium size
    #[default]
    Default,
    /// Extra small size
    Xs,
    /// Small size
    Sm,
    /// Medium size (explicit)
    Md,
    /// Large size
    Lg,
}

impl LoadingSize {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            LoadingSize::Default => "",
            LoadingSize::Xs => "loading-xs",
            LoadingSize::Sm => "loading-sm",
            LoadingSize::Md => "loading-md",
            LoadingSize::Lg => "loading-lg",
        }
    }
}
