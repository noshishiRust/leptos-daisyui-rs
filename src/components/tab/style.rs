/// # Tab Size Variants
///
/// Style enum for daisyUI tab size classes that control the physical dimensions
/// of tab components. Sizes scale proportionally for various contexts.
#[derive(Clone, Debug, Default)]
pub enum TabSize {
    /// Default size (no size class applied)
    #[default]
    Default,
    /// Extra small size for compact layouts
    Xs,
    /// Small size for minimal space usage
    Sm,
    /// Medium size for standard usage
    Md,
    /// Large size for emphasis and visibility
    Lg,
}

impl TabSize {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            TabSize::Default => "",
            TabSize::Xs => "tabs-xs",
            TabSize::Sm => "tabs-sm",
            TabSize::Md => "tabs-md",
            TabSize::Lg => "tabs-lg",
        }
    }
}

/// # Tab Style Variants
///
/// Style enum for daisyUI tab style classes that control the visual appearance
/// of tab containers.
#[derive(Clone, Debug, Default)]
pub enum TabVariant {
    /// Default tab style (no style class applied)
    #[default]
    Default,
    /// Box style with background and borders
    Boxed,
    /// Border style with bottom borders
    Border,
    /// Lift style with elevated appearance
    Lift,
}

impl TabVariant {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            TabVariant::Default => "",
            TabVariant::Boxed => "tabs-box",
            TabVariant::Border => "tabs-border",
            TabVariant::Lift => "tabs-lift",
        }
    }
}
