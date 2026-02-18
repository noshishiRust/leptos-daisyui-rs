/// Color variants for checkbox components based on daisyUI's color system.
#[derive(Clone, Debug, Default)]
pub enum CheckboxColor {
    /// Default checkbox color (no additional styling)
    #[default]
    Default,

    /// Primary theme color checkbox
    Primary,

    /// Secondary theme color checkbox
    Secondary,

    /// Accent theme color checkbox
    Accent,

    /// Neutral theme color checkbox
    Neutral,

    /// Success color
    Success,

    /// Warning color
    Warning,

    /// Info color
    Info,

    /// Error color
    Error,
}

impl CheckboxColor {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            CheckboxColor::Default => "",
            CheckboxColor::Primary => "checkbox-primary",
            CheckboxColor::Secondary => "checkbox-secondary",
            CheckboxColor::Accent => "checkbox-accent",
            CheckboxColor::Neutral => "checkbox-neutral",
            CheckboxColor::Success => "checkbox-success",
            CheckboxColor::Warning => "checkbox-warning",
            CheckboxColor::Info => "checkbox-info",
            CheckboxColor::Error => "checkbox-error",
        }
    }
}

/// Size variants for checkbox components based on daisyUI's sizing system.

#[derive(Clone, Debug, Default)]
pub enum CheckboxSize {
    /// Extra small checkbox
    Xs,

    /// Small checkbox
    Sm,

    /// Medium checkbox (explicit size)
    #[default]
    Md,

    /// Large checkbox
    Lg,

    /// Extra large checkbox
    Xl,
}

impl CheckboxSize {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            CheckboxSize::Xs => "checkbox-xs",
            CheckboxSize::Sm => "checkbox-sm",
            CheckboxSize::Md => "checkbox-md",
            CheckboxSize::Lg => "checkbox-lg",
            CheckboxSize::Xl => "checkbox-xl",
        }
    }
}
