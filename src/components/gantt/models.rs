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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    #[test]
    fn test_gantt_task_default() {
        let task = GanttTask::default();

        assert_eq!(task.id, "");
        assert_eq!(task.name, "");
        assert_eq!(task.progress, 0.0);
        assert_eq!(task.task_type, TaskType::Task);
        assert!(task.parent_id.is_none());
        assert!(task.dependencies.is_empty());
        assert!(task.assignees.is_empty());
        assert!(task.color.is_none());
        assert!(task.metadata.is_empty());
    }

    #[test]
    fn test_gantt_task_custom() {
        let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2024, 1, 15, 0, 0, 0).unwrap();

        let task = GanttTask {
            id: "task-1".to_string(),
            name: "Test Task".to_string(),
            start,
            end,
            progress: 0.5,
            task_type: TaskType::Task,
            parent_id: Some("parent-1".to_string()),
            dependencies: vec!["dep-1".to_string()],
            assignees: vec!["user1".to_string(), "user2".to_string()],
            color: Some("#ff0000".to_string()),
            metadata: HashMap::from([("category".to_string(), "development".to_string())]),
        };

        assert_eq!(task.id, "task-1");
        assert_eq!(task.name, "Test Task");
        assert_eq!(task.start, start);
        assert_eq!(task.end, end);
        assert_eq!(task.progress, 0.5);
        assert_eq!(task.task_type, TaskType::Task);
        assert_eq!(task.parent_id, Some("parent-1".to_string()));
        assert_eq!(task.dependencies.len(), 1);
        assert_eq!(task.assignees.len(), 2);
        assert_eq!(task.color, Some("#ff0000".to_string()));
        assert_eq!(
            task.metadata.get("category"),
            Some(&"development".to_string())
        );
    }

    #[test]
    fn test_task_types() {
        assert_eq!(TaskType::Task.as_str(), "task");
        assert_eq!(TaskType::Milestone.as_str(), "milestone");
        assert_eq!(TaskType::Project.as_str(), "project");
        assert_eq!(TaskType::default(), TaskType::Task);
    }

    #[test]
    fn test_task_dependency() {
        let dep = TaskDependency {
            source_id: "task-1".to_string(),
            target_id: "task-2".to_string(),
            dependency_type: DependencyType::FS,
            lag_days: 2,
        };

        assert_eq!(dep.source_id, "task-1");
        assert_eq!(dep.target_id, "task-2");
        assert_eq!(dep.dependency_type, DependencyType::FS);
        assert_eq!(dep.lag_days, 2);
    }

    #[test]
    fn test_dependency_types() {
        assert_eq!(DependencyType::FS.as_str(), "FS");
        assert_eq!(DependencyType::SS.as_str(), "SS");
        assert_eq!(DependencyType::FF.as_str(), "FF");
        assert_eq!(DependencyType::SF.as_str(), "SF");
        assert_eq!(DependencyType::default(), DependencyType::FS);
    }

    #[test]
    fn test_gantt_task_serialization() {
        let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2024, 1, 15, 0, 0, 0).unwrap();

        let task = GanttTask {
            id: "task-1".to_string(),
            name: "Test Task".to_string(),
            start,
            end,
            progress: 0.75,
            task_type: TaskType::Milestone,
            parent_id: None,
            dependencies: vec![],
            assignees: vec!["user1".to_string()],
            color: None,
            metadata: HashMap::new(),
        };

        let json = serde_json::to_string(&task).unwrap();
        let deserialized: GanttTask = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.id, task.id);
        assert_eq!(deserialized.name, task.name);
        assert_eq!(deserialized.progress, task.progress);
        assert_eq!(deserialized.task_type, task.task_type);
    }

    #[test]
    fn test_task_dependency_serialization() {
        let dep = TaskDependency {
            source_id: "task-1".to_string(),
            target_id: "task-2".to_string(),
            dependency_type: DependencyType::SS,
            lag_days: 0,
        };

        let json = serde_json::to_string(&dep).unwrap();
        let deserialized: TaskDependency = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.source_id, dep.source_id);
        assert_eq!(deserialized.target_id, dep.target_id);
        assert_eq!(deserialized.dependency_type, dep.dependency_type);
        assert_eq!(deserialized.lag_days, dep.lag_days);
    }
}
