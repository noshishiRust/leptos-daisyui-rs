/// # Tab Size Variants
///
/// Style enum for daisyUI tab size classes that control the physical dimensions
/// of tab components. Sizes scale proportionally for various contexts.
#[derive(Clone, Debug, Default)]
pub enum TabSize {
    /// Extra small size for compact layouts
    Xs,

    /// Small size for minimal space usage
    Sm,

    /// Medium size for standard usage
    #[default]
    Md,

    /// Large size for emphasis and visibility
    Lg,

    /// Extra large size for emphasis and visibility
    Xl,
}

impl TabSize {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            TabSize::Xs => "tabs-xs",
            TabSize::Sm => "tabs-sm",
            TabSize::Md => "tabs-md",
            TabSize::Lg => "tabs-lg",
            TabSize::Xl => "tabs-xl",
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

/// # Tab Placement
///
/// Style enum for daisyUI tab style classes that control the visual appearance
/// of tab containers.
#[derive(Clone, Debug, Default)]
pub enum TabPlacement {
    /// Puts tab buttons on top of the tab-content (default)
    #[default]
    Top,

    /// Puts tabs on under the tab-content
    Bottom,
}

impl TabPlacement {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            TabPlacement::Top => "tabs-top",
            TabPlacement::Bottom => "tabs-bottom",
        }
    }
}
