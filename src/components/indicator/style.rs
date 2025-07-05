#[derive(Clone, Debug, Default)]
pub enum IndicatorVerticalPlacement {
    #[default]
    Top,
    Middle,
    Bottom,
}

impl IndicatorVerticalPlacement {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Top => "indicator-top",
            Self::Middle => "indicator-middle",
            Self::Bottom => "indicator-bottom",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum IndicatorHorizontalPlacement {
    Start,
    Center,
    #[default]
    End,
}

impl IndicatorHorizontalPlacement {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Start => "indicator-start",
            Self::Center => "indicator-center",
            Self::End => "indicator-end",
        }
    }
}
