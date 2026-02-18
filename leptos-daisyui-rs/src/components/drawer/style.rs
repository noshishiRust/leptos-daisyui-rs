/// # Drawer Placement Variants
///
/// Style enum for daisyUI drawer placement classes that control which side
/// of the screen the drawer slides out from.
#[derive(Clone, Debug, Default)]
pub enum DrawerPlacement {
    /// Drawer slides from the left side (default)
    #[default]
    Start,

    /// Drawer slides from the right side
    End,
}

impl DrawerPlacement {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            DrawerPlacement::Start => "",
            DrawerPlacement::End => "drawer-end",
        }
    }
}
