#[derive(Clone, Debug, Default)]
pub enum FileInputStyle {
    #[default]
    Default,
    Ghost,
}

impl FileInputStyle {
    pub fn as_str(&self) -> &'static str {
        match self {
            FileInputStyle::Default => "",
            FileInputStyle::Ghost => "file-input-ghost",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum FileInputColor {
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

impl FileInputColor {
    pub fn as_str(&self) -> &'static str {
        match self {
            FileInputColor::Default => "",
            FileInputColor::Neutral => "file-input-neutral",
            FileInputColor::Primary => "file-input-primary",
            FileInputColor::Secondary => "file-input-secondary",
            FileInputColor::Accent => "file-input-accent",
            FileInputColor::Info => "file-input-info",
            FileInputColor::Success => "file-input-success",
            FileInputColor::Warning => "file-input-warning",
            FileInputColor::Error => "file-input-error",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum FileInputSize {
    #[default]
    Default,
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
}

impl FileInputSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            FileInputSize::Default => "",
            FileInputSize::Xs => "file-input-xs",
            FileInputSize::Sm => "file-input-sm",
            FileInputSize::Md => "file-input-md",
            FileInputSize::Lg => "file-input-lg",
            FileInputSize::Xl => "file-input-xl",
        }
    }
}