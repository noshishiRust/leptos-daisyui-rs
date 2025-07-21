/// # Radial Progress Color Variants
///
/// Style enum for daisyUI radial progress color classes that control the semantic color scheme
/// of radial progress indicators. Colors follow daisyUI's semantic system for context and meaning.
#[derive(Clone, Debug, Default)]
pub enum RadialProgressColor {
    /// Default radial progress color (no color class applied)
    #[default]
    Default,

    /// Primary brand color for main progress indicators
    Primary,

    /// Secondary brand color for secondary progress indicators
    Secondary,

    /// Accent brand color for highlighted progress indicators
    Accent,

    /// Success color for positive progress indicators
    Success,

    /// Info color for informational progress indicators
    Info,

    /// Warning color for cautionary progress indicators
    Warning,

    /// Error color for error state progress indicators
    Error,
}

impl RadialProgressColor {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            RadialProgressColor::Default => "",
            RadialProgressColor::Primary => "text-primary",
            RadialProgressColor::Secondary => "text-secondary",
            RadialProgressColor::Accent => "text-accent",
            RadialProgressColor::Success => "text-success",
            RadialProgressColor::Info => "text-info",
            RadialProgressColor::Warning => "text-warning",
            RadialProgressColor::Error => "text-error",
        }
    }
}

/// # Radial Progress Size Variants
///
/// Style enum for daisyUI radial progress size classes that control the physical dimensions
/// of radial progress indicators using Tailwind CSS width and height utilities.
#[derive(Clone, Debug, Default)]
pub enum RadialProgressSize {
    /// Extra small size
    Xs,

    /// Small size
    Sm,

    /// Medium size
    #[default]
    Md,

    /// Large size
    Lg,

    /// Extra large size
    Xl,
}

impl RadialProgressSize {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            RadialProgressSize::Xs => "w-8 h-8",
            RadialProgressSize::Sm => "w-12 h-12",
            RadialProgressSize::Md => "w-16 h-16",
            RadialProgressSize::Lg => "w-20 h-20",
            RadialProgressSize::Xl => "w-24 h-24",
        }
    }
}
