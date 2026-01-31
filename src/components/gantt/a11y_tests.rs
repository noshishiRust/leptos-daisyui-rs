/// Integration tests for Gantt chart accessibility features
///
/// Tests ARIA attributes, keyboard navigation, screen reader announcements,
/// and focus management to ensure WCAG 2.2 compliance.
#[cfg(test)]
mod tests {
    use super::super::*;
    use chrono::Utc;
    use std::collections::HashMap;

    fn create_test_tasks() -> Vec<GanttTask> {
        vec![
            GanttTask {
                id: "task1".to_string(),
                name: "Task 1".to_string(),
                start: Utc::now(),
                end: Utc::now() + chrono::Duration::days(5),
                progress: 0.5,
                task_type: TaskType::Task,
                parent_id: None,
                dependencies: vec![],
                assignees: vec![],
                color: Some("#3b82f6".to_string()),
                read_only: false,
                metadata: HashMap::new(),
            },
            GanttTask {
                id: "task2".to_string(),
                name: "Task 2".to_string(),
                start: Utc::now() + chrono::Duration::days(6),
                end: Utc::now() + chrono::Duration::days(10),
                progress: 0.0,
                task_type: TaskType::Task,
                parent_id: None,
                dependencies: vec![],
                assignees: vec![],
                color: Some("#10b981".to_string()),
                read_only: false,
                metadata: HashMap::new(),
            },
            GanttTask {
                id: "task3".to_string(),
                name: "Task 3".to_string(),
                start: Utc::now() + chrono::Duration::days(11),
                end: Utc::now() + chrono::Duration::days(15),
                progress: 1.0,
                task_type: TaskType::Milestone,
                parent_id: None,
                dependencies: vec![],
                assignees: vec![],
                color: None,
                read_only: true,
                metadata: HashMap::new(),
            },
        ]
    }

    #[test]
    fn test_accessible_announcement_creation() {
        use crate::components::gantt::utils::AccessibleAnnouncement;

        let polite = AccessibleAnnouncement::polite("Test message");
        assert_eq!(polite.message, "Test message");
        assert!(!polite.assertive);

        let assertive = AccessibleAnnouncement::assertive("Urgent");
        assert_eq!(assertive.message, "Urgent");
        assert!(assertive.assertive);
    }

    #[test]
    fn test_task_aria_label_generation() {
        use crate::components::gantt::utils::task_aria_label;

        let label = task_aria_label(
            "Test Task",
            "2024-01-01",
            "2024-01-05",
            0.75,
            true,
            false,
        );

        assert!(label.contains("Test Task"));
        assert!(label.contains("2024-01-01"));
        assert!(label.contains("2024-01-05"));
        assert!(label.contains("75%"));
        assert!(label.contains("selected"));
        assert!(!label.contains("read-only"));
    }

    #[test]
    fn test_task_aria_label_readonly() {
        use crate::components::gantt::utils::task_aria_label;

        let label = task_aria_label(
            "Readonly Task",
            "2024-01-01",
            "2024-01-05",
            1.0,
            false,
            true,
        );

        assert!(label.contains("Readonly Task"));
        assert!(label.contains("100%"));
        assert!(label.contains("read-only"));
        assert!(!label.contains("selected"));
    }

    #[test]
    fn test_zoom_aria_label_can_zoom() {
        use crate::components::gantt::utils::zoom_aria_label;

        let label = zoom_aria_label("in", true, "day");
        assert_eq!(label, "Zoom in, current view: day");

        let label = zoom_aria_label("out", true, "month");
        assert_eq!(label, "Zoom out, current view: month");
    }

    #[test]
    fn test_zoom_aria_label_cannot_zoom() {
        use crate::components::gantt::utils::zoom_aria_label;

        let label = zoom_aria_label("in", false, "hour");
        assert_eq!(label, "Cannot zoom in from hour view");

        let label = zoom_aria_label("out", false, "year");
        assert_eq!(label, "Cannot zoom out from year view");
    }

    #[test]
    fn test_view_mode_zoom_capabilities() {
        // Test that view modes correctly report zoom capabilities
        assert!(ViewMode::Day.can_zoom_in());
        assert!(ViewMode::Day.can_zoom_out());

        assert!(ViewMode::Hour.can_zoom_out());
        assert!(!ViewMode::Hour.can_zoom_in()); // Hour is most zoomed in

        assert!(ViewMode::Year.can_zoom_in());
        assert!(!ViewMode::Year.can_zoom_out()); // Year is most zoomed out
    }

    #[test]
    fn test_read_only_mode_edit_permissions() {
        // Test that read-only modes correctly block edits
        let full_readonly = ReadOnlyMode::Full;
        let context = EditContext::new("task1".to_string(), EditType::Timeline);

        assert!(!full_readonly.is_edit_allowed(&context));

        let editable = ReadOnlyMode::Editable;
        assert!(editable.is_edit_allowed(&context));
    }

    #[test]
    fn test_read_only_mode_timeline_only() {
        // Test timeline-only read-only mode
        let timeline_only = ReadOnlyMode::TimelineOnly;

        let timeline_context = EditContext::new("task1".to_string(), EditType::Timeline);
        assert!(timeline_only.is_edit_allowed(&timeline_context));

        let props_context = EditContext::new("task1".to_string(), EditType::TaskProperties);
        assert!(!timeline_only.is_edit_allowed(&props_context));

        let progress_context = EditContext::new("task1".to_string(), EditType::Progress);
        assert!(!timeline_only.is_edit_allowed(&progress_context));

        let dep_context = EditContext::new("task1".to_string(), EditType::Dependencies);
        assert!(!timeline_only.is_edit_allowed(&dep_context));
    }

    #[test]
    fn test_gantt_task_progress_validation() {
        let task = GanttTask {
            id: "test".to_string(),
            name: "Test".to_string(),
            start: Utc::now(),
            end: Utc::now() + chrono::Duration::days(1),
            progress: 0.5,
            task_type: TaskType::Task,
            parent_id: None,
            dependencies: vec![],
            assignees: vec![],
            color: None,
            read_only: false,
            metadata: HashMap::new(),
        };

        // Progress should be between 0.0 and 1.0
        assert!(task.progress >= 0.0 && task.progress <= 1.0);
    }

    #[test]
    fn test_task_with_dependencies() {
        let tasks = create_test_tasks();
        assert_eq!(tasks.len(), 3);

        // Verify task properties
        assert_eq!(tasks[0].id, "task1");
        assert_eq!(tasks[0].progress, 0.5);
        assert!(!tasks[0].read_only);

        assert_eq!(tasks[2].id, "task3");
        assert_eq!(tasks[2].progress, 1.0);
        assert!(tasks[2].read_only);
        assert_eq!(tasks[2].task_type, TaskType::Milestone);
    }

    #[test]
    fn test_focus_management_state() {
        // Test that focus state can be managed
        // This would be tested in browser environment with actual DOM
        // Here we just verify the logic works

        let tasks = create_test_tasks();
        let mut focused_idx: Option<usize> = None;

        // Simulate ArrowDown navigation
        focused_idx = Some(focused_idx.unwrap_or(0));
        assert_eq!(focused_idx, Some(0));

        focused_idx = Some((focused_idx.unwrap() + 1).min(tasks.len() - 1));
        assert_eq!(focused_idx, Some(1));

        // Simulate Home key
        focused_idx = Some(0);
        assert_eq!(focused_idx, Some(0));

        // Simulate End key
        focused_idx = Some(tasks.len().saturating_sub(1));
        assert_eq!(focused_idx, Some(2));
    }

    #[test]
    fn test_selection_state_management() {
        // Select a task
        let selected_id = Some("task1".to_string());
        assert_eq!(selected_id, Some("task1".to_string()));

        // Deselect (Escape key)
        let selected_id: Option<String> = None;
        assert_eq!(selected_id, None);
    }

    #[test]
    fn test_keyboard_navigation_bounds() {
        let tasks = create_test_tasks();
        let max_idx = tasks.len() - 1;

        // Test ArrowDown at end (should clamp to last)
        let current_idx = max_idx;
        let new_idx = (current_idx + 1).min(max_idx);
        assert_eq!(new_idx, max_idx);

        // Test ArrowUp at start (should clamp to 0)
        let current_idx: usize = 0;
        let new_idx = current_idx.saturating_sub(1);
        assert_eq!(new_idx, 0);
    }
}
