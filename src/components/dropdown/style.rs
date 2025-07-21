/// # Dropdown Alignment Variants
///
/// Style enum for daisyUI dropdown alignment classes that control horizontal positioning
/// of dropdown menus relative to their trigger element.
#[derive(Clone, Debug, Default)]
pub enum DropdownAlignment {
    /// Left-aligned dropdown (default)
    #[default]
    Start,

    /// Center-aligned dropdown
    Center,

    /// Right-aligned dropdown
    End,
}

impl DropdownAlignment {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            DropdownAlignment::Start => "dropdown-start",
            DropdownAlignment::Center => "dropdown-center",
            DropdownAlignment::End => "dropdown-end",
        }
    }
}

/// # Dropdown Placement Variants
///
/// Style enum for daisyUI dropdown placement classes that control the direction
/// dropdown menus expand from their trigger element.
#[derive(Clone, Debug, Default)]
pub enum DropdownPlacement {
    /// Dropdown expands upward
    Top,

    /// Dropdown expands downward (default)
    #[default]
    Bottom,

    /// Dropdown expands to the left
    Left,

    /// Dropdown expands to the right
    Right,
}

impl DropdownPlacement {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            DropdownPlacement::Top => "dropdown-top",
            DropdownPlacement::Bottom => "dropdown-bottom",
            DropdownPlacement::Left => "dropdown-left",
            DropdownPlacement::Right => "dropdown-right",
        }
    }
}
