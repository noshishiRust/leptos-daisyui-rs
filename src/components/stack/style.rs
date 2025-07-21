/// # Stack Placement Variants
#[derive(Clone, Debug, Default)]
pub enum StackPlacement {
    /// Top vertical placement
    Top,

    /// Bottom vertical placement
    #[default]
    Bottom,

    /// Start horizontal placement
    Start,

    /// End horizaonahorizontall placement
    End,
}

impl StackPlacement {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Top => "stack-top",
            Self::Bottom => "indicator-bottom",
            Self::Start => "stack-start",
            Self::End => "stack-end",
        }
    }
}
