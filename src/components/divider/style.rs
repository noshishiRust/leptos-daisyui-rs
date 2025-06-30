#[derive(Clone, Debug, Default)]
pub enum DividerColor {
    #[default]
    Default,
    Neutral,
    Primary,
    Secondary,
    Accent,
    Success,
    Warning,
    Info,
    Error,
}

impl DividerColor {
    pub fn as_str(&self) -> &'static str {
        match self {
            DividerColor::Default => "",
            DividerColor::Neutral => "divider-neutral",
            DividerColor::Primary => "divider-primary",
            DividerColor::Secondary => "divider-secondary",
            DividerColor::Accent => "divider-accent",
            DividerColor::Success => "divider-success",
            DividerColor::Warning => "divider-warning",
            DividerColor::Info => "divider-info",
            DividerColor::Error => "divider-error",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum DividerDirection {
    #[default]
    Horizontal,
    Vertical,
}

impl DividerDirection {
    pub fn as_str(&self) -> &'static str {
        match self {
            DividerDirection::Horizontal => "divider-horizontal",
            DividerDirection::Vertical => "divider-vertical",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum DividerPlacement {
    #[default]
    Default,
    Start,
    End,
}

impl DividerPlacement {
    pub fn as_str(&self) -> &'static str {
        match self {
            DividerPlacement::Default => "",
            DividerPlacement::Start => "divider-start",
            DividerPlacement::End => "divider-end",
        }
    }
}
