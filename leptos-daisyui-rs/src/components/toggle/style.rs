/// Color variants for the Toggle component.
#[derive(Clone, Debug, Default)]
pub enum ToggleColor {
    /// Default toggle styling with no additional color classes
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

impl ToggleColor {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            ToggleColor::Default => "",
            ToggleColor::Primary => "toggle-primary",
            ToggleColor::Secondary => "toggle-secondary",
            ToggleColor::Accent => "toggle-accent",
            ToggleColor::Success => "toggle-success",
            ToggleColor::Warning => "toggle-warning",
            ToggleColor::Info => "toggle-info",
            ToggleColor::Error => "toggle-error",
        }
    }
}

/// Size variants for the Toggle component.
#[derive(Clone, Debug, Default)]
pub enum ToggleSize {
    /// Extra small toggle
    Xs,

    /// Small toggle
    Sm,

    /// Medium toggle (default size)
    #[default]
    Md,

    /// Large toggle
    Lg,

    /// Extra large toggle
    Xl,
}

impl ToggleSize {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            ToggleSize::Xs => "toggle-xs",
            ToggleSize::Sm => "toggle-sm",
            ToggleSize::Md => "toggle-md",
            ToggleSize::Lg => "toggle-lg",
            ToggleSize::Xl => "toggle-xl",
        }
    }
}
