/// # Tooltip Position Variants
///
/// Position variants for daisyUI tooltip component that control where the tooltip appears
/// relative to its trigger element.
#[derive(Clone, Debug, Default)]
pub enum TooltipPosition {
    /// Tooltip appears above the element (default)
    #[default]
    Top,

    /// Tooltip appears below the element
    Bottom,

    /// Tooltip appears to the left of the element
    Left,

    /// Tooltip appears to the right of the element
    Right,
}

impl TooltipPosition {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            TooltipPosition::Top => "tooltip-top",
            TooltipPosition::Bottom => "tooltip-bottom",
            TooltipPosition::Left => "tooltip-left",
            TooltipPosition::Right => "tooltip-right",
        }
    }
}

/// # Tooltip Color Variants
///
/// Color variants for daisyUI tooltip component that control the background color
/// of the tooltip bubble.
#[derive(Clone, Debug, Default)]
pub enum TooltipColor {
    /// Default tooltip color (theme-dependent)
    #[default]
    Default,

    /// Neutral color
    Neutral,

    /// Primary brand color
    Primary,

    /// Secondary brand color
    Secondary,

    /// Accent brand color
    Accent,

    /// Info color for informational tooltips
    Info,

    /// Success color for positive tooltips
    Success,

    /// Warning color for caution tooltips
    Warning,

    /// Error color for error tooltips
    Error,
}

impl TooltipColor {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            TooltipColor::Default => "",
            TooltipColor::Neutral => "tooltip-neutral",
            TooltipColor::Primary => "tooltip-primary",
            TooltipColor::Secondary => "tooltip-secondary",
            TooltipColor::Accent => "tooltip-accent",
            TooltipColor::Info => "tooltip-info",
            TooltipColor::Success => "tooltip-success",
            TooltipColor::Warning => "tooltip-warning",
            TooltipColor::Error => "tooltip-error",
        }
    }
}
