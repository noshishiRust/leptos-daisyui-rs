/// Color variants for the Status component.
#[derive(Clone, Debug, Default)]
pub enum StatusColor {
    /// Default status styling
    #[default]
    Default,

    /// Neutral color
    Neutral,

    /// Primary theme color
    Primary,

    /// Secondary theme color
    Secondary,

    /// Accent theme color
    Accent,

    /// Info color
    Info,

    /// Success color
    Success,

    /// Warning color
    Warning,

    /// Error color
    Error,
}

impl StatusColor {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            StatusColor::Default => "",
            StatusColor::Neutral => "status-neutral",
            StatusColor::Primary => "status-primary",
            StatusColor::Secondary => "status-secondary",
            StatusColor::Accent => "status-accent",
            StatusColor::Info => "status-info",
            StatusColor::Success => "status-success",
            StatusColor::Warning => "status-warning",
            StatusColor::Error => "status-error",
        }
    }
}

/// Size variants for the Status component.
#[derive(Clone, Debug, Default)]
pub enum StatusSize {
    /// Extra small status
    Xs,

    /// Small status
    Sm,

    /// Medium status
    #[default]
    Md,

    /// Large status
    Lg,

    /// Extra large status
    Xl,
}

impl StatusSize {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            StatusSize::Xs => "status-xs",
            StatusSize::Sm => "status-sm",
            StatusSize::Md => "status-md",
            StatusSize::Lg => "status-lg",
            StatusSize::Xl => "status-xl",
        }
    }
}
