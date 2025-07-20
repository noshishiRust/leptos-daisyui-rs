/// # Alert Style Variants
///
/// Style enum for daisyUI alert style classes that control the visual appearance
/// and treatment of alert components.
#[derive(Clone, Debug, Default)]
pub enum AlertStyle {
    /// Default filled alert style
    #[default]
    Default,

    /// Transparent background with colored border
    Outline,

    /// Dashed border style
    Dash,

    /// Subtle background with soft appearance
    Soft,
}

impl AlertStyle {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            AlertStyle::Default => "",
            AlertStyle::Outline => "alert-outline",
            AlertStyle::Dash => "alert-dash",
            AlertStyle::Soft => "alert-soft",
        }
    }
}

/// # Alert Color Variants
///
/// Style enum for daisyUI alert color classes that control the semantic color scheme
/// of alert components. Colors convey the meaning and urgency of the alert message.
#[derive(Clone, Debug, Default)]
pub enum AlertColor {
    /// Default alert color (no color class applied)
    #[default]
    Default,

    /// Info color for informational messages
    Info,

    /// Success color for positive feedback
    Success,

    /// Warning color for caution messages
    Warning,

    /// Error color for critical/error messages
    Error,
}

impl AlertColor {
    /// CSS class string
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

/// # Alert Direction Variants
///
/// Style enum for daisyUI alert direction classes that control the layout orientation
/// of alert components and their content arrangement.
#[derive(Clone, Debug, Default)]
pub enum AlertDirection {
    /// Default layout direction
    #[default]
    Default,

    /// Vertical layout with stacked content
    Vertical,

    /// Horizontal layout with inline content
    Horizontal,
}

impl AlertDirection {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            AlertDirection::Default => "",
            AlertDirection::Vertical => "alert-vertical",
            AlertDirection::Horizontal => "alert-horizontal",
        }
    }
}
