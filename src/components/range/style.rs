#[derive(Clone, Debug, Default)]
pub enum RangeColor {
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

impl RangeColor {
    pub fn as_str(&self) -> &'static str {
        match self {
            RangeColor::Default => "",
            RangeColor::Primary => "range-primary",
            RangeColor::Secondary => "range-secondary",
            RangeColor::Accent => "range-accent",
            RangeColor::Success => "range-success",
            RangeColor::Warning => "range-warning",
            RangeColor::Info => "range-info",
            RangeColor::Error => "range-error",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum RangeSize {
    #[default]
    Default,
    Xs,
    Sm,
    Md,
    Lg,
}

impl RangeSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            RangeSize::Default => "",
            RangeSize::Xs => "range-xs",
            RangeSize::Sm => "range-sm",
            RangeSize::Md => "range-md",
            RangeSize::Lg => "range-lg",
        }
    }
}