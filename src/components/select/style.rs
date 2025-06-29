#[derive(Clone, Debug, Default)]
pub enum SelectStyle {
    #[default]
    Default,
    Ghost,
}

impl SelectStyle {
    pub fn as_str(&self) -> &'static str {
        match self {
            SelectStyle::Default => "",
            SelectStyle::Ghost => "select-ghost",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum SelectColor {
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

impl SelectColor {
    pub fn as_str(&self) -> &'static str {
        match self {
            SelectColor::Default => "",
            SelectColor::Primary => "select-primary",
            SelectColor::Secondary => "select-secondary",
            SelectColor::Accent => "select-accent",
            SelectColor::Info => "select-info",
            SelectColor::Success => "select-success",
            SelectColor::Warning => "select-warning",
            SelectColor::Error => "select-error",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum SelectSize {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

impl SelectSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            SelectSize::Xs => "select-xs",
            SelectSize::Sm => "select-sm",
            SelectSize::Md => "select-md",
            SelectSize::Lg => "select-lg",
            SelectSize::Xl => "select-xl",
        }
    }
}
