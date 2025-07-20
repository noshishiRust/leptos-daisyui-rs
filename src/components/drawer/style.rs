#[derive(Clone, Debug, Default)]
pub enum DrawerPlacement {
    #[default]
    Start,
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
