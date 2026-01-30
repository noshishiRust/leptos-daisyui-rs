/// Text size variants
#[derive(Clone, Debug, Default, PartialEq)]
pub enum TextSize {
    /// Extra small text (text-xs)
    XSmall,
    /// Small text (text-sm)
    Small,
    /// Base text size (text-base)
    #[default]
    Base,
    /// Large text (text-lg)
    Large,
    /// Extra large text (text-xl)
    XLarge,
    /// 2XL text (text-2xl)
    XXLarge,
    /// 3XL text (text-3xl)
    XXXLarge,
}

impl TextSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::XSmall => "text-xs",
            Self::Small => "text-sm",
            Self::Base => "text-base",
            Self::Large => "text-lg",
            Self::XLarge => "text-xl",
            Self::XXLarge => "text-2xl",
            Self::XXXLarge => "text-3xl",
        }
    }
}

/// Text weight variants
#[derive(Clone, Debug, Default, PartialEq)]
pub enum TextWeight {
    /// Thin weight (font-thin)
    Thin,
    /// Normal weight (font-normal)
    #[default]
    Normal,
    /// Medium weight (font-medium)
    Medium,
    /// Semibold weight (font-semibold)
    Semibold,
    /// Bold weight (font-bold)
    Bold,
}

impl TextWeight {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Thin => "font-thin",
            Self::Normal => "font-normal",
            Self::Medium => "font-medium",
            Self::Semibold => "font-semibold",
            Self::Bold => "font-bold",
        }
    }
}

/// Text color variants
#[derive(Clone, Debug, Default, PartialEq)]
pub enum TextColor {
    /// Default color
    #[default]
    Default,
    /// Primary color (text-primary)
    Primary,
    /// Secondary color (text-secondary)
    Secondary,
    /// Accent color (text-accent)
    Accent,
    /// Neutral color (text-neutral)
    Neutral,
    /// Info color (text-info)
    Info,
    /// Success color (text-success)
    Success,
    /// Warning color (text-warning)
    Warning,
    /// Error color (text-error)
    Error,
}

impl TextColor {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "",
            Self::Primary => "text-primary",
            Self::Secondary => "text-secondary",
            Self::Accent => "text-accent",
            Self::Neutral => "text-neutral",
            Self::Info => "text-info",
            Self::Success => "text-success",
            Self::Warning => "text-warning",
            Self::Error => "text-error",
        }
    }
}

/// Text alignment variants
#[derive(Clone, Debug, Default, PartialEq)]
pub enum TextAlign {
    /// Left aligned (text-left)
    #[default]
    Left,
    /// Center aligned (text-center)
    Center,
    /// Right aligned (text-right)
    Right,
    /// Justified (text-justify)
    Justify,
}

impl TextAlign {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Left => "text-left",
            Self::Center => "text-center",
            Self::Right => "text-right",
            Self::Justify => "text-justify",
        }
    }
}
