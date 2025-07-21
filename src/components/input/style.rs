/// # Input Style Variants
///
/// Style enum for daisyUI input style classes that control the visual appearance
/// of input components.
#[derive(Clone, Debug, Default)]
pub enum InputStyle {
    /// Default input style (no style class applied)
    #[default]
    Default,

    /// Ghost style with transparent background
    Ghost,
}

impl InputStyle {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            InputStyle::Default => "",
            InputStyle::Ghost => "input-ghost",
        }
    }
}

/// # Input Color Variants
///
/// Style enum for daisyUI input color classes that control the semantic color scheme
/// of input components. Colors follow daisyUI's semantic system for context and meaning.
#[derive(Clone, Debug, Default)]
pub enum InputColor {
    /// Default input color (no color class applied)
    #[default]
    Default,

    /// Neutral color for subdued inputs
    Neutral,

    /// Primary brand color for main action inputs
    Primary,

    /// Secondary brand color for secondary inputs
    Secondary,

    /// Accent brand color for highlighted inputs
    Accent,

    /// Info color for informational inputs
    Info,

    /// Success color for positive action inputs
    Success,

    /// Warning color for cautionary inputs
    Warning,

    /// Error color for error state inputs
    Error,
}

impl InputColor {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            InputColor::Default => "",
            InputColor::Neutral => "input-neutral",
            InputColor::Primary => "input-primary",
            InputColor::Secondary => "input-secondary",
            InputColor::Accent => "input-accent",
            InputColor::Info => "input-info",
            InputColor::Success => "input-success",
            InputColor::Warning => "input-warning",
            InputColor::Error => "input-error",
        }
    }
}

/// # Input Size Variants
///
/// Style enum for daisyUI input size classes that control the physical dimensions
/// of input components. Sizes scale proportionally for various contexts.
#[derive(Clone, Debug, Default)]
pub enum InputSize {
    /// Extra small size for compact layouts
    Xs,

    /// Small size for minimal space usage
    Sm,

    /// Medium size for standard usage
    #[default]
    Md,

    /// Large size for emphasis and visibility
    Lg,

    /// Extra large size for prominent display
    Xl,
}

impl InputSize {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            InputSize::Xs => "input-xs",
            InputSize::Sm => "input-sm",
            InputSize::Md => "input-md",
            InputSize::Lg => "input-lg",
            InputSize::Xl => "input-xl",
        }
    }
}
