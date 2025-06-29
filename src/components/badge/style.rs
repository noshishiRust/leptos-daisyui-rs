#[derive(Clone, Debug, Default)]
pub enum BadgeStyle {
    #[default]
    Default,
    Outline,
    Dash,
    Soft,
    Ghost,
}

impl BadgeStyle {
    pub fn as_str(&self) -> &'static str {
        match self {
            BadgeStyle::Default => "",
            BadgeStyle::Outline => "badge-outline",
            BadgeStyle::Dash => "badge-dash",
            BadgeStyle::Soft => "badge-soft",
            BadgeStyle::Ghost => "badge-ghost",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum BadgeColor {
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

impl BadgeColor {
    pub fn as_str(&self) -> &'static str {
        match self {
            BadgeColor::Default => "",
            BadgeColor::Neutral => "badge-neutral",
            BadgeColor::Primary => "badge-primary",
            BadgeColor::Secondary => "badge-secondary",
            BadgeColor::Accent => "badge-accent",
            BadgeColor::Info => "badge-info",
            BadgeColor::Success => "badge-success",
            BadgeColor::Warning => "badge-warning",
            BadgeColor::Error => "badge-error",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum BadgeSize {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

impl BadgeSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            BadgeSize::Xs => "badge-xs",
            BadgeSize::Sm => "badge-sm",
            BadgeSize::Md => "badge-md",
            BadgeSize::Lg => "badge-lg",
            BadgeSize::Xl => "badge-xl",
        }
    }
}
