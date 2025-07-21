/// Color variants for divider styling and semantic meaning.
#[derive(Clone, Debug, Default)]
pub enum DividerColor {
    /// Default divider color using theme defaults
    #[default]
    Default,

    /// Neutral color variant for subtle separation
    Neutral,

    /// Primary brand color for important sections
    Primary,

    /// Secondary color variant
    Secondary,

    /// Accent color for special emphasis
    Accent,

    /// Success color for completed or positive sections
    Success,

    /// Warning color for sections requiring attention
    Warning,

    /// Information color for informational sections
    Info,

    /// Error color for problem or error sections
    Error,
}

impl DividerColor {
    /// CSS class string
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

/// Direction variants for divider orientation.
#[derive(Clone, Debug, Default, PartialEq)]
pub enum DividerDirection {
    /// Horizontal divider for separating content vertically (default)
    #[default]
    Horizontal,

    /// Vertical divider for separating content horizontally
    Vertical,
}

impl DividerDirection {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            DividerDirection::Horizontal => "divider-horizontal",
            DividerDirection::Vertical => "divider-vertical",
        }
    }
}

/// Placement variants for divider text positioning.
#[derive(Clone, Debug, Default)]
pub enum DividerPlacement {
    /// Default center placement for divider text
    #[default]
    Default,

    /// Start-aligned placement (left in LTR, right in RTL)
    Start,

    /// End-aligned placement (right in LTR, left in RTL)
    End,
}

impl DividerPlacement {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            DividerPlacement::Default => "",
            DividerPlacement::Start => "divider-start",
            DividerPlacement::End => "divider-end",
        }
    }
}
