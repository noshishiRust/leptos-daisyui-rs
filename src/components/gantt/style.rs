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
