#[derive(Clone, Debug, Default)]
pub enum CardStyle {
    #[default]
    Default,
    Border,
    Dash,
}

impl CardStyle {
    pub fn as_str(&self) -> &'static str {
        match self {
            CardStyle::Default => "",
            CardStyle::Border => "card-border",
            CardStyle::Dash => "card-dash",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum CardSize {
    #[default]
    Default,
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
}

impl CardSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            CardSize::Default => "",
            CardSize::Xs => "card-xs",
            CardSize::Sm => "card-sm",
            CardSize::Md => "card-md",
            CardSize::Lg => "card-lg",
            CardSize::Xl => "card-xl",
        }
    }
}
