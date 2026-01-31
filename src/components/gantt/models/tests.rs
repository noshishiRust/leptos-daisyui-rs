#[cfg(test)]
mod tests {
    use super::super::*;
    use chrono::{TimeZone, Utc};

    #[test]
    fn test_gantt_task_new() {
        let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2024, 1, 15, 0, 0, 0).unwrap();

        let task = GanttTask::new("task-1".to_string(), "Test Task".to_string(), start, end);

        assert_eq!(task.id, "task-1");
        assert_eq!(task.name, "Test Task");
        assert_eq!(task.start, start);
        assert_eq!(task.end, end);
        assert_eq!(task.progress, 0.0);
        assert_eq!(task.task_type, TaskType::Task);
        assert!(task.parent_id.is_none());
    }

    #[test]
    fn test_gantt_task_with_progress() {
        let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2024, 1, 15, 0, 0, 0).unwrap();

        let task = GanttTask::new("task-1".to_string(), "Test Task".to_string(), start, end)
            .with_progress(0.75);

        assert_eq!(task.progress, 0.75);
    }

    #[test]
    fn test_gantt_task_progress_clamped() {
        let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2024, 1, 15, 0, 0, 0).unwrap();

        let task_over = GanttTask::new("task-1".to_string(), "Test Task".to_string(), start, end)
            .with_progress(1.5);
        assert_eq!(task_over.progress, 1.0);

        let task_under = GanttTask::new("task-2".to_string(), "Test Task".to_string(), start, end)
            .with_progress(-0.5);
        assert_eq!(task_under.progress, 0.0);
    }

    #[test]
    fn test_gantt_task_with_parent() {
        let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2024, 1, 15, 0, 0, 0).unwrap();

        let task = GanttTask::new("task-1".to_string(), "Test Task".to_string(), start, end)
            .with_parent("parent-1");

        assert_eq!(task.parent_id, Some("parent-1".to_string()));
    }

    #[test]
    fn test_gantt_task_with_type() {
        let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2024, 1, 15, 0, 0, 0).unwrap();

        let milestone = GanttTask::new("task-1".to_string(), "Milestone".to_string(), start, end)
            .with_type(TaskType::Milestone);
        assert_eq!(milestone.task_type, TaskType::Milestone);

        let project = GanttTask::new("task-2".to_string(), "Project".to_string(), start, end)
            .with_type(TaskType::Project);
        assert_eq!(project.task_type, TaskType::Project);
    }

    #[test]
    fn test_gantt_task_duration() {
        let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2024, 1, 15, 0, 0, 0).unwrap();

        let task = GanttTask::new("task-1".to_string(), "Test Task".to_string(), start, end);
        let duration = task.duration();

        assert_eq!(duration.num_days(), 14);
    }

    #[test]
    fn test_gantt_task_is_milestone() {
        let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();

        let milestone = GanttTask::new("task-1".to_string(), "Milestone".to_string(), start, end)
            .with_type(TaskType::Milestone);
        assert!(milestone.is_milestone());

        // Zero duration task is also milestone
        let zero_duration = GanttTask::new("task-2".to_string(), "Zero".to_string(), start, end);
        assert!(zero_duration.is_milestone());

        let regular = GanttTask::new("task-3".to_string(), "Regular".to_string(), start, Utc.with_ymd_and_hms(2024, 1, 15, 0, 0, 0).unwrap());
        assert!(!regular.is_milestone());
    }

    #[test]
    fn test_gantt_task_is_project() {
        let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2024, 1, 15, 0, 0, 0).unwrap();

        let project = GanttTask::new("task-1".to_string(), "Project".to_string(), start, end)
            .with_type(TaskType::Project);
        assert!(project.is_project());

        let task = GanttTask::new("task-2".to_string(), "Task".to_string(), start, end);
        assert!(!task.is_project());
    }

    #[test]
    fn test_task_dependency_new() {
        let dep = TaskDependency::new("task-2", "task-1");

        assert_eq!(dep.task_id, "task-2");
        assert_eq!(dep.depends_on_id, "task-1");
        assert_eq!(dep.dependency_type, DependencyType::FinishToStart);
    }

    #[test]
    fn test_task_dependency_with_type() {
        let dep = TaskDependency::with_type("task-2", "task-1", DependencyType::StartToStart);

        assert_eq!(dep.dependency_type, DependencyType::StartToStart);
    }

    #[test]
    fn test_task_dependency_types() {
        assert_eq!(DependencyType::default(), DependencyType::FinishToStart);
    }

    #[test]
    fn test_priority_default() {
        assert_eq!(Priority::default(), Priority::Normal);
    }

    #[test]
    fn test_priority_as_str() {
        assert_eq!(Priority::Low.as_str(), "low");
        assert_eq!(Priority::Normal.as_str(), "normal");
        assert_eq!(Priority::High.as_str(), "high");
        assert_eq!(Priority::Critical.as_str(), "critical");
    }

    #[test]
    fn test_gantt_task_with_priority() {
        let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2024, 1, 15, 0, 0, 0).unwrap();

        let task = GanttTask::new("task-1".to_string(), "Critical Task".to_string(), start, end)
            .with_priority(Priority::Critical);

        assert_eq!(task.priority, Priority::Critical);
    }

    #[test]
    fn test_gantt_task_with_assignees() {
        let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2024, 1, 15, 0, 0, 0).unwrap();

        let task = GanttTask::new("task-1".to_string(), "Team Task".to_string(), start, end)
            .with_assignee("user1")
            .with_assignee("user2");

        assert_eq!(task.assignees.len(), 2);
        assert!(task.assignees.contains(&"user1".to_string()));
        assert!(task.assignees.contains(&"user2".to_string()));
    }

    #[test]
    fn test_gantt_task_with_metadata() {
        let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2024, 1, 15, 0, 0, 0).unwrap();

        let task = GanttTask::new("task-1".to_string(), "Tagged Task".to_string(), start, end)
            .with_metadata("category", "development")
            .with_metadata("sprint", "sprint-1");

        assert_eq!(task.metadata.get("category"), Some(&"development".to_string()));
        assert_eq!(task.metadata.get("sprint"), Some(&"sprint-1".to_string()));
    }

    #[test]
    fn test_gantt_config_default() {
        let config = GanttConfig::default();

        assert_eq!(config.view_mode, ViewMode::Week);
        assert!(config.show_labels);
        assert!(config.show_progress);
        assert!(!config.draggable);
        assert!(!config.resizable);
        assert!(config.show_dependencies);
        assert!(config.show_today);
        assert_eq!(config.bar_height, 32);
        assert_eq!(config.bar_padding, 8);
        assert!(config.virtual_scroll);
    }

    #[test]
    fn test_gantt_task_serialization() {
        let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2024, 1, 15, 0, 0, 0).unwrap();

        let task = GanttTask::new("task-1".to_string(), "Test Task".to_string(), start, end)
            .with_progress(0.5)
            .with_priority(Priority::High);

        let json = serde_json::to_string(&task).unwrap();
        let deserialized: GanttTask = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.id, task.id);
        assert_eq!(deserialized.name, task.name);
        assert_eq!(deserialized.progress, task.progress);
        assert_eq!(deserialized.priority, task.priority);
    }

    #[test]
    fn test_task_dependency_serialization() {
        let dep = TaskDependency::with_type("task-2", "task-1", DependencyType::StartToStart);

        let json = serde_json::to_string(&dep).unwrap();
        let deserialized: TaskDependency = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.task_id, dep.task_id);
        assert_eq!(deserialized.depends_on_id, dep.depends_on_id);
        assert_eq!(deserialized.dependency_type, dep.dependency_type);
    }
}
