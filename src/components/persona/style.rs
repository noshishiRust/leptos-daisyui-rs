/// Persona size variants
#[derive(Clone, Debug, Default, PartialEq)]
pub enum PersonaSize {
    /// Extra small
    XSmall,
    /// Small
    Small,
    /// Medium (default)
    #[default]
    Medium,
    /// Large
    Large,
    /// Extra large
    XLarge,
}

impl PersonaSize {
    /// Returns the avatar size class
    pub fn avatar_class(&self) -> &'static str {
        match self {
            Self::XSmall => "w-8",
            Self::Small => "w-10",
            Self::Medium => "w-12",
            Self::Large => "w-16",
            Self::XLarge => "w-24",
        }
    }

    /// Returns the text size class for the name
    pub fn name_class(&self) -> &'static str {
        match self {
            Self::XSmall => "text-xs",
            Self::Small => "text-sm",
            Self::Medium => "text-base",
            Self::Large => "text-lg",
            Self::XLarge => "text-xl",
        }
    }

    /// Returns the text size class for the secondary text
    pub fn secondary_class(&self) -> &'static str {
        match self {
            Self::XSmall => "text-xs",
            Self::Small => "text-xs",
            Self::Medium => "text-sm",
            Self::Large => "text-base",
            Self::XLarge => "text-lg",
        }
    }
}
