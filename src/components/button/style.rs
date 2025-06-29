#[derive(Clone, Debug, Default)]
pub enum ButtonColor {
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

impl ButtonColor {
    pub fn as_str(&self) -> &'static str {
        match self {
            ButtonColor::Default => "",
            ButtonColor::Neutral => "btn-neutral",
            ButtonColor::Primary => "btn-primary",
            ButtonColor::Secondary => "btn-secondary",
            ButtonColor::Accent => "btn-accent",
            ButtonColor::Info => "btn-info",
            ButtonColor::Success => "btn-success",
            ButtonColor::Warning => "btn-warning",
            ButtonColor::Error => "btn-error",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum ButtonStyle {
    #[default]
    Default,
    Outline,
    Dash,
    Soft,
    Ghost,
    Link,
}

impl ButtonStyle {
    pub fn as_str(&self) -> &'static str {
        match self {
            ButtonStyle::Default => "",
            ButtonStyle::Outline => "btn-outline",
            ButtonStyle::Dash => "btn-dash",
            ButtonStyle::Soft => "btn-soft",
            ButtonStyle::Ghost => "btn-ghost",
            ButtonStyle::Link => "btn-link",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum ButtonSize {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

impl ButtonSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            ButtonSize::Xs => "btn-xs",
            ButtonSize::Sm => "btn-sm",
            ButtonSize::Md => "btn-md",
            ButtonSize::Lg => "btn-lg",
            ButtonSize::Xl => "btn-xl",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum ButtonShape {
    #[default]
    Default,
    Wide,
    Block,
    Square,
    Circle,
}

impl ButtonShape {
    pub fn as_str(&self) -> &'static str {
        match self {
            ButtonShape::Default => "",
            ButtonShape::Wide => "btn-wide",
            ButtonShape::Block => "btn-block",
            ButtonShape::Square => "btn-square",
            ButtonShape::Circle => "btn-circle",
        }
    }
}
