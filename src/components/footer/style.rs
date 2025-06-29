#[derive(Clone, Debug, Default)]
pub enum FooterPlacement {
    #[default]
    Default,
    Center,
}

impl FooterPlacement {
    pub fn as_str(&self) -> &'static str {
        match self {
            FooterPlacement::Default => "",
            FooterPlacement::Center => "footer-center",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum FooterDirection {
    #[default]
    Default,
    Horizontal,
    Vertical,
}

impl FooterDirection {
    pub fn as_str(&self) -> &'static str {
        match self {
            FooterDirection::Default => "",
            FooterDirection::Horizontal => "footer-horizontal",
            FooterDirection::Vertical => "footer-vertical",
        }
    }
}