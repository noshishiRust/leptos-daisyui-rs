#[derive(Clone, Debug, Default)]
pub enum LinkColor {
    #[default]
    Default,
    Neutral,
    Primary,
    Secondary,
    Accent,
    Success,
    Info,
    Warning,
    Error,
}

impl LinkColor {
    pub fn as_str(&self) -> &'static str {
        match self {
            LinkColor::Default => "",
            LinkColor::Neutral => "link-neutral",
            LinkColor::Primary => "link-primary",
            LinkColor::Secondary => "link-secondary",
            LinkColor::Accent => "link-accent",
            LinkColor::Success => "link-success",
            LinkColor::Info => "link-info",
            LinkColor::Warning => "link-warning",
            LinkColor::Error => "link-error",
        }
    }
}