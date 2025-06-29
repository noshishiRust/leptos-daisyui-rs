#[derive(Clone, Debug, Default)]
pub enum AlertStyle {
    #[default]
    Default,
    Outline,
    Dash,
    Soft,
}

impl AlertStyle {
    pub fn as_str(&self) -> &'static str {
        match self {
            AlertStyle::Default => "",
            AlertStyle::Outline => "alert-outline",
            AlertStyle::Dash => "alert-dash",
            AlertStyle::Soft => "alert-soft",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum AlertColor {
    #[default]
    Default,
    Info,
    Success,
    Warning,
    Error,
}

impl AlertColor {
    pub fn as_str(&self) -> &'static str {
        match self {
            AlertColor::Default => "",
            AlertColor::Info => "alert-info",
            AlertColor::Success => "alert-success",
            AlertColor::Warning => "alert-warning",
            AlertColor::Error => "alert-error",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum AlertDirection {
    #[default]
    Default,
    Vertical,
    Horizontal,
}

impl AlertDirection {
    pub fn as_str(&self) -> &'static str {
        match self {
            AlertDirection::Default => "",
            AlertDirection::Vertical => "alert-vertical",
            AlertDirection::Horizontal => "alert-horizontal",
        }
    }
}
