/// # Status Color Variants
///
/// Style enum for daisyUI status color classes that control the semantic color scheme
/// of status indicators. Colors follow daisyUI's semantic system for context and meaning.
#[derive(Clone, Debug, Default)]
pub enum StatusColor {
    /// Default status color (no color class applied)
    #[default]
    Default,
    /// Neutral color for subdued status indicators
    Neutral,
    /// Primary brand color for main status indicators
    Primary,
    /// Secondary brand color for secondary status indicators
    Secondary,
    /// Accent brand color for highlighted status indicators
    Accent,
    /// Info color for informational status indicators
    Info,
    /// Success color for positive status indicators
    Success,
    /// Warning color for cautionary status indicators
    Warning,
    /// Error color for error state status indicators
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

/// # Status Size Variants
///
/// Style enum for daisyUI status size classes that control the physical dimensions
/// of status indicators. Sizes scale proportionally for various contexts.
#[derive(Clone, Debug, Default)]
pub enum StatusSize {
    /// Default size (no size class applied)
    #[default]
    Default,
    /// Extra small size for compact layouts
    Xs,
    /// Small size for minimal space usage
    Sm,
    /// Medium size for standard usage
    Md,
    /// Large size for emphasis and visibility
    Lg,
    /// Extra large size for prominent display
    Xl,
}

impl StatusSize {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            StatusSize::Default => "",
            StatusSize::Xs => "status-xs",
            StatusSize::Sm => "status-sm",
            StatusSize::Md => "status-md",
            StatusSize::Lg => "status-lg",
            StatusSize::Xl => "status-xl",
        }
    }
}
