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

#[derive(Clone, Debug, Default)]
pub enum TimelineItemPosition {
    #[default]
    Start,
    End,
    Between,
}

impl TimelineItemPosition {
    pub fn is_start(&self) -> bool {
        matches!(self, TimelineItemPosition::Start)
    }

    pub fn is_end(&self) -> bool {
        matches!(self, TimelineItemPosition::End)
    }

    pub fn is_between(&self) -> bool {
        matches!(self, TimelineItemPosition::Between)
    }
}
