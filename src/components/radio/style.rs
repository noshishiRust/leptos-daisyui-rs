/// Color variants for the Radio component.
#[derive(Clone, Debug, Default)]
pub enum RadioColor {
    /// Default radio styling with no additional color classes
    #[default]
    Default,

    /// Primary theme color
    Primary,

    /// Secondary theme color
    Secondary,

    /// Accent theme color
    Accent,

    /// Success/positive color (green)
    Success,

    /// Warning color (yellow/orange)
    Warning,

    /// Info color (blue)
    Info,

    /// Error/danger color (red)
    Error,
}

impl RadioColor {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            RadioColor::Default => "",
            RadioColor::Primary => "radio-primary",
            RadioColor::Secondary => "radio-secondary",
            RadioColor::Accent => "radio-accent",
            RadioColor::Success => "radio-success",
            RadioColor::Warning => "radio-warning",
            RadioColor::Info => "radio-info",
            RadioColor::Error => "radio-error",
        }
    }
}

/// Size variants for the Radio component.
#[derive(Clone, Debug, Default)]
pub enum RadioSize {
    /// Extra small range
    Xs,

    /// Small range
    Sm,

    /// Medium range
    #[default]
    Md,

    /// Large range
    Lg,

    /// Extra large range
    Xl,
}

impl RadioSize {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            RadioSize::Xs => "radio-xs",
            RadioSize::Sm => "radio-sm",
            RadioSize::Md => "radio-md",
            RadioSize::Lg => "radio-lg",
            RadioSize::Xl => "radio-xl",
        }
    }
}
