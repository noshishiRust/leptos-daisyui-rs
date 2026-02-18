/// Style variants for the Textarea component.
#[derive(Clone, Debug, Default)]
pub enum TextareaStyle {
    /// Standard textarea styling with default appearance
    #[default]
    Default,

    /// Transparent background with border appearing on focus
    Ghost,
}

impl TextareaStyle {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            TextareaStyle::Default => "",
            TextareaStyle::Ghost => "textarea-ghost",
        }
    }
}

/// Color variants for the Textarea component.
#[derive(Clone, Debug, Default)]
pub enum TextareaColor {
    /// Default textarea styling with no additional color classes
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

impl TextareaColor {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            TextareaColor::Default => "",
            TextareaColor::Primary => "textarea-primary",
            TextareaColor::Secondary => "textarea-secondary",
            TextareaColor::Accent => "textarea-accent",
            TextareaColor::Info => "textarea-info",
            TextareaColor::Success => "textarea-success",
            TextareaColor::Warning => "textarea-warning",
            TextareaColor::Error => "textarea-error",
        }
    }
}

/// Size variants for the Textarea component.
#[derive(Clone, Debug, Default)]
pub enum TextareaSize {
    /// Extra small textarea
    Xs,

    /// Small textarea
    Sm,

    /// Medium textarea (default size)
    #[default]
    Md,

    /// Large textarea
    Lg,

    /// Extra large textarea
    Xl,
}

impl TextareaSize {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            TextareaSize::Xs => "textarea-xs",
            TextareaSize::Sm => "textarea-sm",
            TextareaSize::Md => "textarea-md",
            TextareaSize::Lg => "textarea-lg",
            TextareaSize::Xl => "textarea-xl",
        }
    }
}
