use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::style::TaskType;

/// Represents a task in the Gantt chart
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GanttTask {
    /// Unique identifier for the task
    pub id: String,

    /// Display name of the task
    pub name: String,

    /// Task start date/time
    pub start: DateTime<Utc>,

    /// Task end date/time
    pub end: DateTime<Utc>,

    /// Task completion progress (0.0 to 1.0)
    pub progress: f32,

    /// Type of task (Task, Milestone, Project)
    pub task_type: TaskType,

    /// Parent task ID for hierarchical tasks
    pub parent_id: Option<String>,

    /// List of task dependencies
    pub dependencies: Vec<String>,

    /// Assigned users/resources
    pub assignees: Vec<String>,

    /// Custom color for the task bar (hex color)
    pub color: Option<String>,

    /// Additional metadata for custom extensions
    pub metadata: HashMap<String, String>,
}

/// Represents a dependency relationship between two tasks
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskDependency {
    /// Source task ID (the task that must be completed)
    pub source_id: String,

    /// Target task ID (the task that depends on the source)
    pub target_id: String,

    /// Type of dependency relationship
    pub dependency_type: DependencyType,

    /// Number of days lag/lead time (positive for lag, negative for lead)
    pub lag_days: i32,
}

/// Type of dependency relationship between tasks
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum DependencyType {
    /// Finish-to-Start: Source task must finish before target can start
    #[default]
    FS,

    /// Start-to-Start: Both tasks must start at the same time
    SS,

    /// Finish-to-Finish: Both tasks must finish at the same time
    FF,

    /// Start-to-Finish: Source task must start before target can finish
    SF,
}

impl DependencyType {
    /// Returns the string representation of the dependency type
    pub fn as_str(&self) -> &'static str {
        match self {
            DependencyType::FS => "FS",
            DependencyType::SS => "SS",
            DependencyType::FF => "FF",
            DependencyType::SF => "SF",
        }
    }
}

impl Default for GanttTask {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            start: Utc::now(),
            end: Utc::now(),
            progress: 0.0,
            task_type: TaskType::default(),
            parent_id: None,
            dependencies: Vec::new(),
            assignees: Vec::new(),
            color: None,
            metadata: HashMap::new(),
        }
    }
}
