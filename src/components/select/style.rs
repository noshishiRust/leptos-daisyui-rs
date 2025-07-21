/// Style variants for the Select component.
#[derive(Clone, Debug, Default)]
pub enum SelectStyle {
    /// Standard select styling with default appearance
    #[default]
    Default,

    /// Transparent background with border appearing on focus
    Ghost,
}

impl SelectStyle {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            SelectStyle::Default => "",
            SelectStyle::Ghost => "select-ghost",
        }
    }
}

/// Color variants for the Select component.
#[derive(Clone, Debug, Default)]
pub enum SelectColor {
    /// Default select styling with no additional color classes
    #[default]
    Default,

    /// Primary theme color
    Primary,

    /// Secondary theme color
    Secondary,

    /// Accent theme color
    Accent,

    /// Info color (blue)
    Info,

    /// Success/positive color (green)
    Success,

    /// Warning color (yellow/orange)
    Warning,

    /// Error/danger color (red)
    Error,
}

impl SelectColor {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            SelectColor::Default => "",
            SelectColor::Primary => "select-primary",
            SelectColor::Secondary => "select-secondary",
            SelectColor::Accent => "select-accent",
            SelectColor::Info => "select-info",
            SelectColor::Success => "select-success",
            SelectColor::Warning => "select-warning",
            SelectColor::Error => "select-error",
        }
    }
}

/// Size variants for the Select component.
#[derive(Clone, Debug, Default)]
pub enum SelectSize {
    /// Extra small select
    Xs,

    /// Small select
    Sm,

    /// Medium select (default size)
    #[default]
    Md,

    /// Large select
    Lg,

    /// Extra large select
    Xl,
}

impl SelectSize {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            SelectSize::Xs => "select-xs",
            SelectSize::Sm => "select-sm",
            SelectSize::Md => "select-md",
            SelectSize::Lg => "select-lg",
            SelectSize::Xl => "select-xl",
        }
    }
}
