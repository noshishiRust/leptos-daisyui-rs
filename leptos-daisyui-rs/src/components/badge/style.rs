/// # Badge Style Variants
///
/// Style enum for daisyUI badge style classes that control the visual appearance
/// and treatment of badge components.
#[derive(Clone, Debug, Default)]
pub enum BadgeStyle {
    /// Default filled badge style
    #[default]
    Default,

    /// Transparent background with colored border
    Outline,

    /// Dashed border style
    Dash,

    /// Subtle background with soft appearance
    Soft,

    /// Minimal styling with transparent background
    Ghost,
}

impl BadgeStyle {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            BadgeStyle::Default => "",
            BadgeStyle::Outline => "badge-outline",
            BadgeStyle::Dash => "badge-dash",
            BadgeStyle::Soft => "badge-soft",
            BadgeStyle::Ghost => "badge-ghost",
        }
    }
}

/// # Badge Color Variants
///
/// Style enum for daisyUI badge color classes that control the semantic color scheme
/// of badge components. Colors follow daisyUI's semantic color system and convey
/// meaning about status, categories, or importance.
#[derive(Clone, Debug, Default)]
pub enum BadgeColor {
    /// Default badge color (no color class applied)
    #[default]
    Default,

    /// Neutral dark color for non-saturated UI elements
    Neutral,

    /// Primary brand color for main elements
    Primary,

    /// Secondary brand color for secondary elements
    Secondary,

    /// Accent brand color for highlighted elements
    Accent,

    /// Info color for informational badges
    Info,

    /// Success color for positive status
    Success,

    /// Warning color for caution status
    Warning,

    /// Error color for critical/error status
    Error,
}

impl BadgeColor {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            BadgeColor::Default => "",
            BadgeColor::Neutral => "badge-neutral",
            BadgeColor::Primary => "badge-primary",
            BadgeColor::Secondary => "badge-secondary",
            BadgeColor::Accent => "badge-accent",
            BadgeColor::Info => "badge-info",
            BadgeColor::Success => "badge-success",
            BadgeColor::Warning => "badge-warning",
            BadgeColor::Error => "badge-error",
        }
    }
}

/// # Badge Size Variants
///
/// Style enum for daisyUI badge size classes that control the physical dimensions
/// of badge components. Sizes scale proportionally from extra small to extra large.
#[derive(Clone, Debug, Default)]
pub enum BadgeSize {
    /// Extra small size for minimal space usage
    Xs,

    /// Small size for compact layouts
    Sm,

    /// Medium size (default) for standard usage
    #[default]
    Md,

    /// Large size for emphasis
    Lg,

    /// Extra large size for high visibility
    Xl,
}

impl BadgeSize {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            BadgeSize::Xs => "badge-xs",
            BadgeSize::Sm => "badge-sm",
            BadgeSize::Md => "badge-md",
            BadgeSize::Lg => "badge-lg",
            BadgeSize::Xl => "badge-xl",
        }
    }
}
