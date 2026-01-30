/// Icon size variants
#[derive(Clone, Debug, Default, PartialEq)]
pub enum IconSize {
    /// Extra small (16px)
    XSmall,
    /// Small (20px)
    Small,
    /// Medium (24px)
    #[default]
    Medium,
    /// Large (32px)
    Large,
    /// Extra large (48px)
    XLarge,
}

impl IconSize {
    /// Returns the size in pixels
    pub fn as_px(&self) -> u32 {
        match self {
            Self::XSmall => 16,
            Self::Small => 20,
            Self::Medium => 24,
            Self::Large => 32,
            Self::XLarge => 48,
        }
    }

    /// Returns the CSS class string for this size variant
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::XSmall => "w-4 h-4",
            Self::Small => "w-5 h-5",
            Self::Medium => "w-6 h-6",
            Self::Large => "w-8 h-8",
            Self::XLarge => "w-12 h-12",
        }
    }
}
