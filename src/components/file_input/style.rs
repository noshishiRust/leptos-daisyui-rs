/// # File Input Style Variants
///
/// Style enum for daisyUI file input style classes that control the visual appearance
/// of file input components.
#[derive(Clone, Debug, Default)]
pub enum FileInputStyle {
    /// Default file input style (no style class applied)
    #[default]
    Default,

    /// Ghost style with transparent background
    Ghost,
}

impl FileInputStyle {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            FileInputStyle::Default => "",
            FileInputStyle::Ghost => "file-input-ghost",
        }
    }
}

/// # File Input Color Variants
///
/// Style enum for daisyUI file input color classes that control the semantic color scheme
/// of file input components. Colors follow daisyUI's semantic system for context and meaning.
#[derive(Clone, Debug, Default)]
pub enum FileInputColor {
    /// Default file input color (no color class applied)
    #[default]
    Default,

    /// Neutral color for subdued file inputs
    Neutral,

    /// Primary brand color for main action file inputs
    Primary,

    /// Secondary brand color for secondary file inputs
    Secondary,

    /// Accent brand color for highlighted file inputs
    Accent,

    /// Info color for informational file inputs
    Info,

    /// Success color for positive action file inputs
    Success,

    /// Warning color for cautionary file inputs
    Warning,

    /// Error color for error state file inputs
    Error,
}

impl FileInputColor {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            FileInputColor::Default => "",
            FileInputColor::Neutral => "file-input-neutral",
            FileInputColor::Primary => "file-input-primary",
            FileInputColor::Secondary => "file-input-secondary",
            FileInputColor::Accent => "file-input-accent",
            FileInputColor::Info => "file-input-info",
            FileInputColor::Success => "file-input-success",
            FileInputColor::Warning => "file-input-warning",
            FileInputColor::Error => "file-input-error",
        }
    }
}

/// # File Input Size Variants
///
/// Style enum for daisyUI file input size classes that control the physical dimensions
/// of file input components. Sizes scale proportionally for various contexts.
#[derive(Clone, Debug, Default)]
pub enum FileInputSize {
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

impl FileInputSize {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            FileInputSize::Xs => "file-input-xs",
            FileInputSize::Sm => "file-input-sm",
            FileInputSize::Md => "file-input-md",
            FileInputSize::Lg => "file-input-lg",
            FileInputSize::Xl => "file-input-xl",
        }
    }
}
