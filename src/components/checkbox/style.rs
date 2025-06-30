#[derive(Clone, Debug, Default)]
pub enum CheckboxColor {
    #[default]
    Default,
    Primary,
    Secondary,
    Accent,
    Neutral,
    Success,
    Warning,
    Info,
    Error,
}

impl CheckboxColor {
    pub fn as_str(&self) -> &'static str {
        match self {
            CheckboxColor::Default => "",
            CheckboxColor::Primary => "checkbox-primary",
            CheckboxColor::Secondary => "checkbox-secondary",
            CheckboxColor::Accent => "checkbox-accent",
            CheckboxColor::Neutral => "checkbox-neutral",
            CheckboxColor::Success => "checkbox-success",
            CheckboxColor::Warning => "checkbox-warning",
            CheckboxColor::Info => "checkbox-info",
            CheckboxColor::Error => "checkbox-error",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum CheckboxSize {
    #[default]
    Default,
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
}

impl CheckboxSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            CheckboxSize::Default => "",
            CheckboxSize::Xs => "checkbox-xs",
            CheckboxSize::Sm => "checkbox-sm",
            CheckboxSize::Md => "checkbox-md",
            CheckboxSize::Lg => "checkbox-lg",
            CheckboxSize::Xl => "checkbox-xl",
        }
    }
}
