/// Color variants for the Slider component
#[derive(Clone, Debug, Default, PartialEq)]
pub enum SliderColor {
    /// Default color (uses theme default)
    #[default]
    Default,
    /// Primary color
    Primary,
    /// Secondary color
    Secondary,
    /// Accent color
    Accent,
    /// Success color (green)
    Success,
    /// Warning color (yellow/orange)
    Warning,
    /// Info color (blue)
    Info,
    /// Error color (red)
    Error,
}

impl SliderColor {
    /// Returns the CSS class for the slider color
    pub fn as_str(&self) -> &'static str {
        match self {
            SliderColor::Default => "",
            SliderColor::Primary => "range-primary",
            SliderColor::Secondary => "range-secondary",
            SliderColor::Accent => "range-accent",
            SliderColor::Success => "range-success",
            SliderColor::Warning => "range-warning",
            SliderColor::Info => "range-info",
            SliderColor::Error => "range-error",
        }
    }
}

/// Size variants for the Slider component
#[derive(Clone, Debug, Default, PartialEq)]
pub enum SliderSize {
    /// Extra small size
    ExtraSmall,
    /// Small size
    Small,
    /// Medium size (default)
    #[default]
    Medium,
    /// Large size
    Large,
}

impl SliderSize {
    /// Returns the CSS class for the slider size
    pub fn as_str(&self) -> &'static str {
        match self {
            SliderSize::ExtraSmall => "range-xs",
            SliderSize::Small => "range-sm",
            SliderSize::Medium => "range-md",
            SliderSize::Large => "range-lg",
        }
    }
}
