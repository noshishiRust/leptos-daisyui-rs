use serde::{Deserialize, Serialize};

/// Type of task in the Gantt chart
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum TaskType {
    /// Regular task with start and end dates
    #[default]
    Task,

    /// Milestone task (zero duration, single date)
    Milestone,

    /// Project container task (parent of other tasks)
    Project,
}

impl TaskType {
    /// Returns the string representation of the task type
    pub fn as_str(&self) -> &'static str {
        match self {
            TaskType::Task => "task",
            TaskType::Milestone => "milestone",
            TaskType::Project => "project",
        }
    }
}

/// View mode for the Gantt chart timeline
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ViewMode {
    /// Hourly view (1 hour per column)
    Hour,

    /// Daily view (1 day per column)
    #[default]
    Day,

    /// Weekly view (1 week per column)
    Week,

    /// Monthly view (1 month per column)
    Month,

    /// Quarterly view (3 months per column)
    Quarter,

    /// Yearly view (1 year per column)
    Year,
}

impl ViewMode {
    /// Returns the string representation of the view mode
    pub fn as_str(&self) -> &'static str {
        match self {
            ViewMode::Hour => "hour",
            ViewMode::Day => "day",
            ViewMode::Week => "week",
            ViewMode::Month => "month",
            ViewMode::Quarter => "quarter",
            ViewMode::Year => "year",
        }
    }

    /// Zoom in to a more detailed view mode (if possible)
    pub fn zoom_in(self) -> Self {
        match self {
            ViewMode::Year => ViewMode::Quarter,
            ViewMode::Quarter => ViewMode::Month,
            ViewMode::Month => ViewMode::Week,
            ViewMode::Week => ViewMode::Day,
            ViewMode::Day => ViewMode::Hour,
            ViewMode::Hour => ViewMode::Hour, // Already at maximum zoom
        }
    }

    /// Zoom out to a less detailed view mode (if possible)
    pub fn zoom_out(self) -> Self {
        match self {
            ViewMode::Hour => ViewMode::Day,
            ViewMode::Day => ViewMode::Week,
            ViewMode::Week => ViewMode::Month,
            ViewMode::Month => ViewMode::Quarter,
            ViewMode::Quarter => ViewMode::Year,
            ViewMode::Year => ViewMode::Year, // Already at minimum zoom
        }
    }

    /// Check if zooming in is possible
    pub fn can_zoom_in(self) -> bool {
        !matches!(self, ViewMode::Hour)
    }

    /// Check if zooming out is possible
    pub fn can_zoom_out(self) -> bool {
        !matches!(self, ViewMode::Year)
    }
}

/// Height/density setting for task bars
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum GanttTaskHeight {
    /// Compact view (20px task bars)
    Compact,

    /// Medium view (30px task bars)
    #[default]
    Medium,

    /// Comfortable view (40px task bars)
    Comfortable,
}

impl GanttTaskHeight {
    /// Returns the string representation of the task height setting
    pub fn as_str(&self) -> &'static str {
        match self {
            GanttTaskHeight::Compact => "compact",
            GanttTaskHeight::Medium => "medium",
            GanttTaskHeight::Comfortable => "comfortable",
        }
    }

    /// Returns the pixel height for this setting
    pub const fn height_px(&self) -> u32 {
        match self {
            GanttTaskHeight::Compact => 20,
            GanttTaskHeight::Medium => 30,
            GanttTaskHeight::Comfortable => 40,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TaskType tests
    #[test]
    fn test_task_type_as_str() {
        assert_eq!(TaskType::Task.as_str(), "task");
        assert_eq!(TaskType::Milestone.as_str(), "milestone");
        assert_eq!(TaskType::Project.as_str(), "project");
    }

    #[test]
    fn test_task_type_default() {
        assert_eq!(TaskType::default(), TaskType::Task);
    }

    #[test]
    fn test_task_type_serialization() {
        let task_type = TaskType::Milestone;
        let json = serde_json::to_string(&task_type).unwrap();
        let deserialized: TaskType = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, task_type);
    }

    // ViewMode tests
    #[test]
    fn test_view_mode_as_str() {
        assert_eq!(ViewMode::Hour.as_str(), "hour");
        assert_eq!(ViewMode::Day.as_str(), "day");
        assert_eq!(ViewMode::Week.as_str(), "week");
        assert_eq!(ViewMode::Month.as_str(), "month");
        assert_eq!(ViewMode::Quarter.as_str(), "quarter");
        assert_eq!(ViewMode::Year.as_str(), "year");
    }

    #[test]
    fn test_view_mode_default() {
        assert_eq!(ViewMode::default(), ViewMode::Day);
    }

    #[test]
    fn test_view_mode_zoom_in() {
        assert_eq!(ViewMode::Year.zoom_in(), ViewMode::Quarter);
        assert_eq!(ViewMode::Quarter.zoom_in(), ViewMode::Month);
        assert_eq!(ViewMode::Month.zoom_in(), ViewMode::Week);
        assert_eq!(ViewMode::Week.zoom_in(), ViewMode::Day);
        assert_eq!(ViewMode::Day.zoom_in(), ViewMode::Hour);
        assert_eq!(ViewMode::Hour.zoom_in(), ViewMode::Hour); // Max zoom
    }

    #[test]
    fn test_view_mode_zoom_out() {
        assert_eq!(ViewMode::Hour.zoom_out(), ViewMode::Day);
        assert_eq!(ViewMode::Day.zoom_out(), ViewMode::Week);
        assert_eq!(ViewMode::Week.zoom_out(), ViewMode::Month);
        assert_eq!(ViewMode::Month.zoom_out(), ViewMode::Quarter);
        assert_eq!(ViewMode::Quarter.zoom_out(), ViewMode::Year);
        assert_eq!(ViewMode::Year.zoom_out(), ViewMode::Year); // Min zoom
    }

    #[test]
    fn test_view_mode_can_zoom_in() {
        assert!(ViewMode::Year.can_zoom_in());
        assert!(ViewMode::Quarter.can_zoom_in());
        assert!(ViewMode::Month.can_zoom_in());
        assert!(ViewMode::Week.can_zoom_in());
        assert!(ViewMode::Day.can_zoom_in());
        assert!(!ViewMode::Hour.can_zoom_in()); // Cannot zoom in from Hour
    }

    #[test]
    fn test_view_mode_can_zoom_out() {
        assert!(!ViewMode::Year.can_zoom_out()); // Cannot zoom out from Year
        assert!(ViewMode::Quarter.can_zoom_out());
        assert!(ViewMode::Month.can_zoom_out());
        assert!(ViewMode::Week.can_zoom_out());
        assert!(ViewMode::Day.can_zoom_out());
        assert!(ViewMode::Hour.can_zoom_out());
    }

    #[test]
    fn test_view_mode_zoom_sequence() {
        // Test full zoom in sequence
        let mut mode = ViewMode::Year;
        mode = mode.zoom_in();
        assert_eq!(mode, ViewMode::Quarter);
        mode = mode.zoom_in();
        assert_eq!(mode, ViewMode::Month);
        mode = mode.zoom_in();
        assert_eq!(mode, ViewMode::Week);
        mode = mode.zoom_in();
        assert_eq!(mode, ViewMode::Day);
        mode = mode.zoom_in();
        assert_eq!(mode, ViewMode::Hour);

        // Test full zoom out sequence
        mode = mode.zoom_out();
        assert_eq!(mode, ViewMode::Day);
        mode = mode.zoom_out();
        assert_eq!(mode, ViewMode::Week);
        mode = mode.zoom_out();
        assert_eq!(mode, ViewMode::Month);
        mode = mode.zoom_out();
        assert_eq!(mode, ViewMode::Quarter);
        mode = mode.zoom_out();
        assert_eq!(mode, ViewMode::Year);
    }

    #[test]
    fn test_view_mode_serialization() {
        let mode = ViewMode::Week;
        let json = serde_json::to_string(&mode).unwrap();
        let deserialized: ViewMode = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, mode);
    }

    // GanttTaskHeight tests
    #[test]
    fn test_gantt_task_height_as_str() {
        assert_eq!(GanttTaskHeight::Compact.as_str(), "compact");
        assert_eq!(GanttTaskHeight::Medium.as_str(), "medium");
        assert_eq!(GanttTaskHeight::Comfortable.as_str(), "comfortable");
    }

    #[test]
    fn test_gantt_task_height_default() {
        assert_eq!(GanttTaskHeight::default(), GanttTaskHeight::Medium);
    }

    #[test]
    fn test_gantt_task_height_height_px() {
        assert_eq!(GanttTaskHeight::Compact.height_px(), 20);
        assert_eq!(GanttTaskHeight::Medium.height_px(), 30);
        assert_eq!(GanttTaskHeight::Comfortable.height_px(), 40);
    }

    #[test]
    fn test_gantt_task_height_serialization() {
        let height = GanttTaskHeight::Comfortable;
        let json = serde_json::to_string(&height).unwrap();
        let deserialized: GanttTaskHeight = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, height);
    }

    #[test]
    fn test_gantt_task_height_ordering() {
        // Verify height progression
        assert!(GanttTaskHeight::Compact.height_px() < GanttTaskHeight::Medium.height_px());
        assert!(GanttTaskHeight::Medium.height_px() < GanttTaskHeight::Comfortable.height_px());
    }
}
