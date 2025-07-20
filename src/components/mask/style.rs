#[derive(Clone, Debug, Default)]
pub enum MaskType {
    #[default]
    Squircle,
    Heart,
    Hexagon,
    HexagonTwo,
    Decagon,
    Pentagon,
    Diamond,
    Square,
    Circle,
    Parallelogram,
    ParallelogramTwo,
    ParallelogramThree,
    ParallelogramFour,
    Star,
    StarTwo,
    Triangle,
    TriangleTwo,
    TriangleThree,
    TriangleFour,
}

impl MaskType {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            MaskType::Squircle => "mask-squircle",
            MaskType::Heart => "mask-heart",
            MaskType::Hexagon => "mask-hexagon",
            MaskType::HexagonTwo => "mask-hexagon-2",
            MaskType::Decagon => "mask-decagon",
            MaskType::Pentagon => "mask-pentagon",
            MaskType::Diamond => "mask-diamond",
            MaskType::Square => "mask-square",
            MaskType::Circle => "mask-circle",
            MaskType::Parallelogram => "mask-parallelogram",
            MaskType::ParallelogramTwo => "mask-parallelogram-2",
            MaskType::ParallelogramThree => "mask-parallelogram-3",
            MaskType::ParallelogramFour => "mask-parallelogram-4",
            MaskType::Star => "mask-star",
            MaskType::StarTwo => "mask-star-2",
            MaskType::Triangle => "mask-triangle",
            MaskType::TriangleTwo => "mask-triangle-2",
            MaskType::TriangleThree => "mask-triangle-3",
            MaskType::TriangleFour => "mask-triangle-4",
        }
    }
}
