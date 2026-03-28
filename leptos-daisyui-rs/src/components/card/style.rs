/// Defines the visual styles for card components.
#[derive(Clone, Debug, Default)]
pub enum CardStyle {
    /// Default card styling with no additional border or outline.
    #[default]
    Default,

    /// Applies a subtle border around the card.
    Border,

    /// Applies a dashed border style to the card.
    Dash,
}

impl CardStyle {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            CardStyle::Default => "",
            CardStyle::Border => "card-border",
            CardStyle::Dash => "card-dash",
        }
    }
}

/// Defines the size variants for card components.
#[derive(Clone, Debug, Default)]
pub enum CardSize {
    /// Extra small card size.
    Xs,

    /// Small card size.
    Sm,

    /// Medium card size.
    #[default]
    Md,

    /// Large card size.
    Lg,

    /// Extra large card size.
    Xl,
}

impl CardSize {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            CardSize::Xs => "card-xs",
            CardSize::Sm => "card-sm",
            CardSize::Md => "card-md",
            CardSize::Lg => "card-lg",
            CardSize::Xl => "card-xl",
        }
    }
}
