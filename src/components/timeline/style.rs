#[derive(Clone, Debug, Default)]
pub enum TimelineDirection {
    #[default]
    Vertical,
    Horizontal,
}

impl TimelineDirection {
    pub fn as_str(&self) -> &'static str {
        match self {
            TimelineDirection::Vertical => "timeline-vertical",
            TimelineDirection::Horizontal => "timeline-horizontal",
        }
    }
}
