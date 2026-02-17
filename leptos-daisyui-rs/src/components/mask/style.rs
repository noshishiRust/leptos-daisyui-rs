/// # Mask Type Variants
///
/// Style enum for daisyUI mask shape classes that crop content to various geometric shapes.
/// Perfect for creating shaped avatars, decorative elements, and custom visual layouts.
#[derive(Clone, Debug, Default)]
pub enum MaskType {
    /// Rounded square with subtle curves (default shape)
    #[default]
    Squircle,

    /// Heart shape for romantic or favorite indicators
    Heart,

    /// Six-sided polygon shape
    Hexagon,

    /// Alternative hexagon shape variation
    HexagonTwo,

    /// Ten-sided polygon shape
    Decagon,

    /// Five-sided polygon shape
    Pentagon,

    /// Diamond/rhombus shape rotated 45 degrees
    Diamond,

    /// Perfect square shape
    Square,

    /// Perfect circle shape
    Circle,

    /// Four-sided parallelogram shape
    Parallelogram,

    /// Alternative parallelogram shape variation
    ParallelogramTwo,

    /// Third parallelogram shape variation
    ParallelogramThree,

    /// Fourth parallelogram shape variation
    ParallelogramFour,

    /// Five-pointed star shape
    Star,

    /// Alternative star shape variation
    StarTwo,

    /// Three-sided triangle shape
    Triangle,

    /// Alternative triangle shape variation
    TriangleTwo,

    /// Third triangle shape variation
    TriangleThree,

    /// Fourth triangle shape variation
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
