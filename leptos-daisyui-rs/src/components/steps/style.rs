/// Direction variants for steps layout orientation.
#[derive(Clone, Debug, Default)]
pub enum StepsDirection {
    /// Horizontal step flow from left to right (default)
    #[default]
    Horizontal,
    /// Vertical step flow from top to bottom
    Vertical,
}

impl StepsDirection {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            StepsDirection::Horizontal => "",
            StepsDirection::Vertical => "steps-vertical",
        }
    }
}

/// Color variants for individual step indicators.
#[derive(Clone, Debug, Default)]
pub enum StepColor {
    /// Default step styling - neutral/inactive appearance
    #[default]
    Default,

    /// Primary color - typically used for current/active step
    Primary,

    /// Secondary color variant
    Secondary,

    /// Accent color for special emphasis
    Accent,

    /// Information state - typically blue, for informational steps
    Info,

    /// Success state - typically green, for completed steps
    Success,

    /// Warning state - typically yellow, for steps requiring attention
    Warning,

    /// Error state - typically red, for failed or blocked steps
    Error,
}

impl StepColor {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            StepColor::Default => "",
            StepColor::Primary => "step-primary",
            StepColor::Secondary => "step-secondary",
            StepColor::Accent => "step-accent",
            StepColor::Info => "step-info",
            StepColor::Success => "step-success",
            StepColor::Warning => "step-warning",
            StepColor::Error => "step-error",
        }
    }
}
