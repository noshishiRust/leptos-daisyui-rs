#[derive(Clone, Debug, Default)]
pub enum StepsDirection {
    #[default]
    Horizontal,
    Vertical,
}

impl StepsDirection {
    pub fn as_str(&self) -> &'static str {
        match self {
            StepsDirection::Horizontal => "",
            StepsDirection::Vertical => "steps-vertical",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum StepColor {
    #[default]
    Default,
    Primary,
    Secondary,
    Accent,
    Info,
    Success,
    Warning,
    Error,
}

impl StepColor {
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
