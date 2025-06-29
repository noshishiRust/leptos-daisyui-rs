#[derive(Clone, Debug, Default)]
pub enum RadioColor {
    #[default]
    Default,
    Primary,
    Secondary,
    Accent,
    Success,
    Warning,
    Info,
    Error,
}

impl RadioColor {
    pub fn as_str(&self) -> &'static str {
        match self {
            RadioColor::Default => "",
            RadioColor::Primary => "radio-primary",
            RadioColor::Secondary => "radio-secondary",
            RadioColor::Accent => "radio-accent",
            RadioColor::Success => "radio-success",
            RadioColor::Warning => "radio-warning",
            RadioColor::Info => "radio-info",
            RadioColor::Error => "radio-error",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum RadioSize {
    #[default]
    Default,
    Xs,
    Sm,
    Md,
    Lg,
}

impl RadioSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            RadioSize::Default => "",
            RadioSize::Xs => "radio-xs",
            RadioSize::Sm => "radio-sm",
            RadioSize::Md => "radio-md",
            RadioSize::Lg => "radio-lg",
        }
    }
}