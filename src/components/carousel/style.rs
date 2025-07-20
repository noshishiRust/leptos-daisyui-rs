#[derive(Clone, Debug, Default)]
pub enum CarouselModifier {
    #[default]
    Default,
    Start,
    Center,
    End,
}

impl CarouselModifier {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            CarouselModifier::Default => "",
            CarouselModifier::Start => "carousel-start",
            CarouselModifier::Center => "carousel-center",
            CarouselModifier::End => "carousel-end",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum CarouselDirection {
    #[default]
    Horizontal,
    Vertical,
}

impl CarouselDirection {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            CarouselDirection::Horizontal => "carousel-horizontal",
            CarouselDirection::Vertical => "carousel-vertical",
        }
    }
}
