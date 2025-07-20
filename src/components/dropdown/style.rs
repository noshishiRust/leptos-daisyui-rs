#[derive(Clone, Debug, Default)]
pub enum DropdownAlignment {
    #[default]
    Start,

    Center,

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

#[derive(Clone, Debug, Default)]
pub enum DropdownPlacement {
    Top,

    #[default]
    Bottom,

    Left,

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
