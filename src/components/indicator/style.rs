#[derive(Clone, Debug, Default)]
pub enum IndicatorPlacement {
    #[default]
    TopEnd,
    Start,
    Center,
    End,
    Top,
    Middle,
    Bottom,
}

impl IndicatorPlacement {
    pub fn as_str(&self) -> &'static str {
        match self {
            IndicatorPlacement::TopEnd => "indicator-top indicator-end",
            IndicatorPlacement::Start => "indicator-start",
            IndicatorPlacement::Center => "indicator-center",
            IndicatorPlacement::End => "indicator-end",
            IndicatorPlacement::Top => "indicator-top",
            IndicatorPlacement::Middle => "indicator-middle",
            IndicatorPlacement::Bottom => "indicator-bottom",
        }
    }
}