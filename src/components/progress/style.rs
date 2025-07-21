/// # Progress Color Variants
#[derive(Clone, Debug, Default)]
pub enum ProgressColor {
    /// Default progress color (no color class applied)
    #[default]
    Default,

    /// Primary brand color for main progress indicators
    Primary,

    /// Secondary brand color for secondary progress
    Secondary,

    /// Accent brand color for highlighted progress
    Accent,

    /// Success color for completed or positive progress
    Success,

    /// Info color for informational progress
    Info,

    /// Warning color for caution or intermediate progress
    Warning,

    /// Error color for failed or problematic progress
    Error,
}

impl ProgressColor {
    /// CSS class string
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
