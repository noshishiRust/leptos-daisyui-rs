/// # Link Color Variants
///
/// Style enum for daisyUI link color classes that control the semantic color scheme
/// of link text. Colors follow daisyUI's semantic system for context and meaning.
#[derive(Clone, Debug, Default)]
pub enum LinkColor {
    /// Default link color (no color class applied)
    #[default]
    Default,

    /// Neutral color for subdued links
    Neutral,

    /// Primary brand color for main action links
    Primary,

    /// Secondary brand color for secondary links
    Secondary,

    /// Accent brand color for highlighted links
    Accent,

    /// Success color for positive action links
    Success,

    /// Info color for informational links
    Info,

    /// Warning color for cautionary links
    Warning,

    /// Error color for destructive action links
    Error,
}

impl LinkColor {
    /// CSS class string
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
