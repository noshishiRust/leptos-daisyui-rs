#[derive(Clone, Debug, Default)]
pub enum StatusColor {
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

impl StatusColor {
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

#[derive(Clone, Debug, Default)]
pub enum StatusSize {
    #[default]
    Default,
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
}

impl StatusSize {
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
