#[derive(Clone, Debug, Default)]
pub enum InputStyle {
    #[default]
    Default,
    Ghost,
}

impl InputStyle {
    pub fn as_str(&self) -> &'static str {
        match self {
            InputStyle::Default => "",
            InputStyle::Ghost => "input-ghost",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum InputColor {
    #[default]
    Default,
    Neutral,
    Primary,
    Secondary,
    Accent,
    Info,
    Success,
    Warning,
    Error,
}

impl InputColor {
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

#[derive(Clone, Debug, Default)]
pub enum InputSize {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

impl InputSize {
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
