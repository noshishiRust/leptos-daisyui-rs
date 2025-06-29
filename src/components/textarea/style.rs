#[derive(Clone, Debug, Default)]
pub enum TextareaStyle {
    #[default]
    Default,
    Ghost,
}

impl TextareaStyle {
    pub fn as_str(&self) -> &'static str {
        match self {
            TextareaStyle::Default => "",
            TextareaStyle::Ghost => "textarea-ghost",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum TextareaColor {
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

impl TextareaColor {
    pub fn as_str(&self) -> &'static str {
        match self {
            TextareaColor::Default => "",
            TextareaColor::Primary => "textarea-primary",
            TextareaColor::Secondary => "textarea-secondary",
            TextareaColor::Accent => "textarea-accent",
            TextareaColor::Info => "textarea-info",
            TextareaColor::Success => "textarea-success",
            TextareaColor::Warning => "textarea-warning",
            TextareaColor::Error => "textarea-error",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum TextareaSize {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

impl TextareaSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            TextareaSize::Xs => "textarea-xs",
            TextareaSize::Sm => "textarea-sm",
            TextareaSize::Md => "textarea-md",
            TextareaSize::Lg => "textarea-lg",
            TextareaSize::Xl => "textarea-xl",
        }
    }
}
