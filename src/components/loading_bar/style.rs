/// LoadingBar color variants
#[derive(Clone, Debug, Default, PartialEq)]
pub enum LoadingBarColor {
    /// Primary color
    #[default]
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

impl LoadingBarColor {
    /// Returns the CSS class string for this color variant
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Primary => "bg-primary",
            Self::Secondary => "bg-secondary",
            Self::Accent => "bg-accent",
            Self::Success => "bg-success",
            Self::Warning => "bg-warning",
            Self::Error => "bg-error",
            Self::Info => "bg-info",
        }
    }
}

/// LoadingBar size variants
#[derive(Clone, Debug, Default, PartialEq)]
pub enum LoadingBarSize {
    /// Extra small (h-1)
    XSmall,
    /// Small (h-2)
    Small,
    /// Medium (h-3)
    #[default]
    Medium,
    /// Large (h-4)
    Large,
}

impl LoadingBarSize {
    /// Returns the CSS class string for this size variant
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::XSmall => "h-1",
            Self::Small => "h-2",
            Self::Medium => "h-3",
            Self::Large => "h-4",
        }
    }
}
