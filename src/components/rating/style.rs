/// # Rating Size Variants
///
/// Style enum for daisyUI rating size classes that control the physical dimensions
/// of rating components. Sizes scale proportionally from extra small to large.
#[derive(Clone, Debug, Default)]
pub enum RatingSize {
    /// Default size (no size class applied)
    #[default]
    Default,

    /// Extra small size for compact layouts
    Xs,

    /// Small size for minimal space usage
    Sm,

    /// Medium size for standard usage
    Md,

    /// Large size for emphasis and visibility
    Lg,
}

impl RatingSize {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            RatingSize::Default => "",
            RatingSize::Xs => "rating-xs",
            RatingSize::Sm => "rating-sm",
            RatingSize::Md => "rating-md",
            RatingSize::Lg => "rating-lg",
        }
    }
}
