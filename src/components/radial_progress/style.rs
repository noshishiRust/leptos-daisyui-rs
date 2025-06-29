#[derive(Clone, Debug, Default)]
pub enum RadialProgressColor {
    #[default]
    Default,
    Primary,
    Secondary,
    Accent,
    Success,
    Info,
    Warning,
    Error,
}

impl RadialProgressColor {
    pub fn as_str(&self) -> &'static str {
        match self {
            RadialProgressColor::Default => "",
            RadialProgressColor::Primary => "text-primary",
            RadialProgressColor::Secondary => "text-secondary",
            RadialProgressColor::Accent => "text-accent",
            RadialProgressColor::Success => "text-success",
            RadialProgressColor::Info => "text-info",
            RadialProgressColor::Warning => "text-warning",
            RadialProgressColor::Error => "text-error",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum RadialProgressSize {
    #[default]
    Default,
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
}

impl RadialProgressSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            RadialProgressSize::Default => "",
            RadialProgressSize::Xs => "w-8 h-8",
            RadialProgressSize::Sm => "w-12 h-12",
            RadialProgressSize::Md => "w-16 h-16",
            RadialProgressSize::Lg => "w-20 h-20",
            RadialProgressSize::Xl => "w-24 h-24",
        }
    }
}