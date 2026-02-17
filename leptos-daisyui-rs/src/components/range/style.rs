/// Color variants for the Range component.
#[derive(Clone, Debug, Default)]
pub enum RangeColor {
    /// Default range styling
    #[default]
    Default,
    /// Primary theme color
    Primary,
    /// Secondary theme color
    Secondary,
    /// Accent theme color
    Accent,
    /// Success color
    Success,
    /// Warning color
    Warning,
    /// Info color
    Info,
    /// Error color
    Error,
}

impl RangeColor {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            RangeColor::Default => "",
            RangeColor::Primary => "range-primary",
            RangeColor::Secondary => "range-secondary",
            RangeColor::Accent => "range-accent",
            RangeColor::Success => "range-success",
            RangeColor::Warning => "range-warning",
            RangeColor::Info => "range-info",
            RangeColor::Error => "range-error",
        }
    }
}

/// Size variants for the Range component.
#[derive(Clone, Debug, Default)]
pub enum RangeSize {
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

impl RangeSize {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            RangeSize::Xs => "range-xs",
            RangeSize::Sm => "range-sm",
            RangeSize::Md => "range-md",
            RangeSize::Lg => "range-lg",
            RangeSize::Xl => "range-xl",
        }
    }
}
