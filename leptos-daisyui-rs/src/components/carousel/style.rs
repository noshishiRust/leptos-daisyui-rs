/// # Carousel Modifier Variants
///
/// Style enum for daisyUI carousel modifier classes that control the alignment
/// and positioning of carousel items.
#[derive(Clone, Debug, Default)]
pub enum CarouselModifier {
    /// Default alignment (no modifier applied)
    #[default]
    Default,

    /// Align carousel items to the start
    Start,

    /// Center carousel items in the container
    Center,

    /// Align carousel items to the end
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

/// # Carousel Direction Variants
///
/// Style enum for daisyUI carousel direction classes that control the scroll
/// direction of the carousel container.
#[derive(Clone, Debug, Default)]
pub enum CarouselDirection {
    /// Horizontal scrolling (default) - items scroll left/right
    #[default]
    Horizontal,

    /// Vertical scrolling - items scroll up/down
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
