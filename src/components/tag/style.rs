/// Tag color variants
#[derive(Clone, Debug, Default, PartialEq)]
pub enum TagColor {
    /// Default/neutral color
    #[default]
    Neutral,
    /// Primary color
    Primary,
    /// Secondary color
    Secondary,
    /// Accent color
    Accent,
    /// Success color
    Success,
    /// Warning color
    Warning,
    /// Error color
    Error,
    /// Info color
    Info,
}

impl TagColor {
    /// Returns the CSS class string for this color variant
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Neutral => "badge-neutral",
            Self::Primary => "badge-primary",
            Self::Secondary => "badge-secondary",
            Self::Accent => "badge-accent",
            Self::Success => "badge-success",
            Self::Warning => "badge-warning",
            Self::Error => "badge-error",
            Self::Info => "badge-info",
        }
    }
}

/// Tag size variants
#[derive(Clone, Debug, Default, PartialEq)]
pub enum TagSize {
    /// Small size
    Small,
    /// Medium size (default)
    #[default]
    Medium,
    /// Large size
    Large,
}

impl TagSize {
    /// Returns the CSS class string for this size variant
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Small => "badge-sm",
            Self::Medium => "badge-md",
            Self::Large => "badge-lg",
        }
    }
}
