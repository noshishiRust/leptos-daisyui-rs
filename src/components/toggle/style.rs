#[derive(Clone, Debug, Default)]
pub enum ToggleColor {
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

impl ToggleColor {
    pub fn as_str(&self) -> &'static str {
        match self {
            ToggleColor::Default => "",
            ToggleColor::Primary => "toggle-primary",
            ToggleColor::Secondary => "toggle-secondary",
            ToggleColor::Accent => "toggle-accent",
            ToggleColor::Success => "toggle-success",
            ToggleColor::Warning => "toggle-warning",
            ToggleColor::Info => "toggle-info",
            ToggleColor::Error => "toggle-error",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum ToggleSize {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

impl ToggleSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            ToggleSize::Xs => "toggle-xs",
            ToggleSize::Sm => "toggle-sm",
            ToggleSize::Md => "toggle-md",
            ToggleSize::Lg => "toggle-lg",
            ToggleSize::Xl => "toggle-xl",
        }
    }
}
