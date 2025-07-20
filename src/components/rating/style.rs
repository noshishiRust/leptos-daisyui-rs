#[derive(Clone, Debug, Default)]
pub enum RatingSize {
    #[default]
    Default,
    Xs,
    Sm,
    Md,
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
