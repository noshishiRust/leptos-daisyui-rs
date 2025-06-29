#[derive(Clone, Debug, Default)]
pub enum ProgressColor {
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

impl ProgressColor {
    pub fn as_str(&self) -> &'static str {
        match self {
            ProgressColor::Default => "",
            ProgressColor::Primary => "progress-primary",
            ProgressColor::Secondary => "progress-secondary",
            ProgressColor::Accent => "progress-accent",
            ProgressColor::Success => "progress-success",
            ProgressColor::Info => "progress-info",
            ProgressColor::Warning => "progress-warning",
            ProgressColor::Error => "progress-error",
        }
    }
}