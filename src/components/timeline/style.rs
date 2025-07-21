/// Direction variants for timeline layout orientation.
///
/// The `TimelineDirection` enum controls how the timeline is oriented,
/// affecting the flow and positioning of timeline items and their content.

#[derive(Clone, Debug, Default)]
pub enum TimelineDirection {
    /// Vertical timeline flowing from top to bottom (default)
    #[default]
    Vertical,

    /// Horizontal timeline flowing from left to right
    Horizontal,
}

impl TimelineDirection {
    /// CSS class string
    pub fn as_str(&self) -> &'static str {
        match self {
            TimelineDirection::Vertical => "timeline-vertical",
            TimelineDirection::Horizontal => "timeline-horizontal",
        }
    }
}

/// Position variants for timeline items affecting connector line display.
///
/// The `TimelineItemPosition` enum determines which connector lines are shown
/// for a timeline item, enabling proper visual flow between timeline events.
/// This is crucial for creating cohesive timeline layouts.
#[derive(Clone, Debug, Default)]
pub enum TimelineItemPosition {
    /// First item in the timeline (shows connector line after)
    #[default]
    Start,

    /// Last item in the timeline (shows connector line before)
    End,

    /// Middle item in the timeline (shows connector lines before and after)
    Between,
}

impl TimelineItemPosition {
    /// Returns true if this is the start position.
    ///
    /// Used internally to determine connector line rendering.
    pub fn is_start(&self) -> bool {
        matches!(self, TimelineItemPosition::Start)
    }

    /// Returns true if this is the end position.
    ///
    /// Used internally to determine connector line rendering.
    pub fn is_end(&self) -> bool {
        matches!(self, TimelineItemPosition::End)
    }

    /// Returns true if this is the between position.
    ///
    /// Used internally to determine connector line rendering.
    pub fn is_between(&self) -> bool {
        matches!(self, TimelineItemPosition::Between)
    }
}
