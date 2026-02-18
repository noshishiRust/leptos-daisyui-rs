/// Style enums for Menu component variants.
#[derive(Clone, Debug, Default)]
pub enum MenuDirection {
    /// Default vertical layout with items stacked vertically
    #[default]
    Vertical,
    /// Horizontal layout with items arranged inline
    Horizontal,
}

impl MenuDirection {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            MenuDirection::Vertical => "",
            MenuDirection::Horizontal => "menu-horizontal",
        }
    }
}

/// Defines the size/scale of menu items.
#[derive(Clone, Debug, Default)]
pub enum MenuSize {
    /// Extra small menu items with minimal padding
    Xs,

    /// Small menu items with reduced padding
    Sm,

    /// Medium menu items with standard padding (default)
    #[default]
    Md,

    /// Large menu items with increased padding
    Lg,

    /// Extra large menu items with maximum padding
    Xl,
}

impl MenuSize {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            MenuSize::Xs => "menu-xs",
            MenuSize::Sm => "menu-sm",
            MenuSize::Md => "menu-md",
            MenuSize::Lg => "menu-lg",
            MenuSize::Xl => "menu-xl",
        }
    }
}
