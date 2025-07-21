/// # Indicator Vertical Placement Variants
///
/// Style enum for daisyUI indicator vertical positioning classes that control
/// where indicators appear along the vertical axis of their parent element.
#[derive(Clone, Debug, Default)]
pub enum IndicatorVerticalPlacement {
    /// Top vertical placement (default)
    #[default]
    Top,

    /// Middle vertical placement
    Middle,

    /// Bottom vertical placement
    Bottom,
}

impl IndicatorVerticalPlacement {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Top => "indicator-top",
            Self::Middle => "indicator-middle",
            Self::Bottom => "indicator-bottom",
        }
    }
}

/// # Indicator Horizontal Placement Variants
///
/// Style enum for daisyUI indicator horizontal positioning classes that control
/// where indicators appear along the horizontal axis of their parent element.
#[derive(Clone, Debug, Default)]
pub enum IndicatorHorizontalPlacement {
    /// Left horizontal placement
    Start,

    /// Center horizontal placement
    Center,

    /// Right horizontal placement (default)
    #[default]
    End,
}

impl IndicatorHorizontalPlacement {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Start => "indicator-start",
            Self::Center => "indicator-center",
            Self::End => "indicator-end",
        }
    }
}
