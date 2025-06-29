#[derive(Clone, Debug, Default)]
pub enum MenuDirection {
    #[default]
    Vertical,
    Horizontal,
}

impl MenuDirection {
    pub fn as_str(&self) -> &'static str {
        match self {
            MenuDirection::Vertical => "",
            MenuDirection::Horizontal => "menu-horizontal",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum MenuSize {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

impl MenuSize {
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
