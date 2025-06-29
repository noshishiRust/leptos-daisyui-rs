#[derive(Clone, Debug, Default)]
pub enum LoadingColor {
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

impl LoadingColor {
    pub fn as_str(&self) -> &'static str {
        match self {
            LoadingColor::Default => "",
            LoadingColor::Neutral => "loading-neutral",
            LoadingColor::Primary => "loading-primary",
            LoadingColor::Secondary => "loading-secondary",
            LoadingColor::Accent => "loading-accent",
            LoadingColor::Info => "loading-info",
            LoadingColor::Success => "loading-success",
            LoadingColor::Warning => "loading-warning",
            LoadingColor::Error => "loading-error",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum LoadingType {
    #[default]
    Spinner,
    Dots,
    Ring,
    Ball,
    Bars,
    Infinity,
}

impl LoadingType {
    pub fn as_str(&self) -> &'static str {
        match self {
            LoadingType::Spinner => "loading-spinner",
            LoadingType::Dots => "loading-dots",
            LoadingType::Ring => "loading-ring",
            LoadingType::Ball => "loading-ball",
            LoadingType::Bars => "loading-bars",
            LoadingType::Infinity => "loading-infinity",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum LoadingSize {
    #[default]
    Default,
    Xs,
    Sm,
    Md,
    Lg,
}

impl LoadingSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            LoadingSize::Default => "",
            LoadingSize::Xs => "loading-xs",
            LoadingSize::Sm => "loading-sm",
            LoadingSize::Md => "loading-md",
            LoadingSize::Lg => "loading-lg",
        }
    }
}
