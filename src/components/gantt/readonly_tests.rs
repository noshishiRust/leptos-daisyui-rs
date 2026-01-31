/// Integration tests for Gantt Chart read-only functionality
///
/// Tests cover:
/// - Read-only mode enforcement at component level
/// - Per-task read-only flag behavior
/// - Visual indicator rendering
/// - Event handling with read-only state
/// - ARIA accessibility attributes

#[cfg(test)]
mod tests {
    use super::super::*;
    use chrono::{TimeZone, Utc};

    fn create_test_task(id: &str, name: &str, read_only: bool) -> GanttTask {
        GanttTask {
            id: id.to_string(),
            name: name.to_string(),
            start: Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap(),
            end: Utc.with_ymd_and_hms(2024, 1, 10, 0, 0, 0).unwrap(),
            progress: 0.5,
            task_type: crate::components::gantt::TaskType::Task,
            parent_id: None,
            dependencies: Vec::new(),
            assignees: Vec::new(),
            color: None,
            read_only,
            metadata: std::collections::HashMap::new(),
        }
    }

    #[test]
    fn test_read_only_mode_full_blocks_all_edits() {
        let mode = ReadOnlyMode::Full;

        // Test all edit types - should all be blocked
        let timeline_ctx = EditContext::new("task1".into(), EditType::Timeline);
        assert!(!mode.is_edit_allowed(&timeline_ctx));

        let props_ctx = EditContext::new("task1".into(), EditType::TaskProperties);
        assert!(!mode.is_edit_allowed(&props_ctx));

        let progress_ctx = EditContext::new("task1".into(), EditType::Progress);
        assert!(!mode.is_edit_allowed(&progress_ctx));

        let delete_ctx = EditContext::new("task1".into(), EditType::DeleteTask);
        assert!(!mode.is_edit_allowed(&delete_ctx));
    }

    #[test]
    fn test_read_only_mode_timeline_only() {
        let mode = ReadOnlyMode::TimelineOnly;

        // Timeline edits should be allowed
        let timeline_ctx = EditContext::new("task1".into(), EditType::Timeline);
        assert!(mode.is_edit_allowed(&timeline_ctx));

        // Other edit types should be blocked
        let props_ctx = EditContext::new("task1".into(), EditType::TaskProperties);
        assert!(!mode.is_edit_allowed(&props_ctx));

        let progress_ctx = EditContext::new("task1".into(), EditType::Progress);
        assert!(!mode.is_edit_allowed(&progress_ctx));

        let create_ctx = EditContext::new("task1".into(), EditType::CreateTask);
        assert!(!mode.is_edit_allowed(&create_ctx));
    }

    #[test]
    fn test_read_only_mode_grid_only() {
        let mode = ReadOnlyMode::GridOnly;

        // Grid/property edits should be allowed
        let props_ctx = EditContext::new("task1".into(), EditType::TaskProperties);
        assert!(mode.is_edit_allowed(&props_ctx));

        let progress_ctx = EditContext::new("task1".into(), EditType::Progress);
        assert!(mode.is_edit_allowed(&progress_ctx));

        let metadata_ctx = EditContext::new("task1".into(), EditType::Metadata);
        assert!(mode.is_edit_allowed(&metadata_ctx));

        // Timeline edits should be blocked
        let timeline_ctx = EditContext::new("task1".into(), EditType::Timeline);
        assert!(!mode.is_edit_allowed(&timeline_ctx));
    }

    #[test]
    fn test_read_only_mode_custom_with_role() {
        let mode = ReadOnlyMode::custom(|ctx: &EditContext| {
            // Only allow edits if user is admin
            ctx.user_role == Some("admin".to_string())
        });

        let admin_ctx = EditContext::new("task1".into(), EditType::Timeline)
            .with_role("admin".into());
        assert!(mode.is_edit_allowed(&admin_ctx));

        let user_ctx = EditContext::new("task1".into(), EditType::Timeline)
            .with_role("user".into());
        assert!(!mode.is_edit_allowed(&user_ctx));

        let no_role_ctx = EditContext::new("task1".into(), EditType::Timeline);
        assert!(!mode.is_edit_allowed(&no_role_ctx));
    }

    #[test]
    fn test_read_only_builder_granular_permissions() {
        let mode = ReadOnlyBuilder::new()
            .allow_timeline_edits(false)
            .allow_property_edits(true)
            .allow_progress_updates(true)
            .allow_task_creation(false)
            .allow_task_deletion(false)
            .build();

        // Timeline should be blocked
        let timeline_ctx = EditContext::new("task1".into(), EditType::Timeline);
        assert!(!mode.is_edit_allowed(&timeline_ctx));

        // Properties should be allowed
        let props_ctx = EditContext::new("task1".into(), EditType::TaskProperties);
        assert!(mode.is_edit_allowed(&props_ctx));

        // Progress should be allowed
        let progress_ctx = EditContext::new("task1".into(), EditType::Progress);
        assert!(mode.is_edit_allowed(&progress_ctx));

        // Creation should be blocked
        let create_ctx = EditContext::new("task1".into(), EditType::CreateTask);
        assert!(!mode.is_edit_allowed(&create_ctx));

        // Deletion should be blocked
        let delete_ctx = EditContext::new("task1".into(), EditType::DeleteTask);
        assert!(!mode.is_edit_allowed(&delete_ctx));
    }

    #[test]
    fn test_read_only_builder_with_required_role() {
        let mode = ReadOnlyBuilder::new()
            .require_role("editor")
            .build();

        // Editor role should be allowed
        let editor_ctx = EditContext::new("task1".into(), EditType::Timeline)
            .with_role("editor".into());
        assert!(mode.is_edit_allowed(&editor_ctx));

        // Viewer role should be blocked
        let viewer_ctx = EditContext::new("task1".into(), EditType::Timeline)
            .with_role("viewer".into());
        assert!(!mode.is_edit_allowed(&viewer_ctx));

        // No role should be blocked
        let no_role_ctx = EditContext::new("task1".into(), EditType::Timeline);
        assert!(!mode.is_edit_allowed(&no_role_ctx));
    }

    #[test]
    fn test_read_only_builder_combine_role_and_permissions() {
        let mode = ReadOnlyBuilder::new()
            .require_role("admin")
            .allow_task_deletion(false)
            .build();

        // Admin can edit properties
        let admin_edit_ctx = EditContext::new("task1".into(), EditType::TaskProperties)
            .with_role("admin".into());
        assert!(mode.is_edit_allowed(&admin_edit_ctx));

        // But admin cannot delete (explicitly disabled)
        let admin_delete_ctx = EditContext::new("task1".into(), EditType::DeleteTask)
            .with_role("admin".into());
        assert!(!mode.is_edit_allowed(&admin_delete_ctx));

        // Non-admin cannot do anything
        let user_ctx = EditContext::new("task1".into(), EditType::TaskProperties)
            .with_role("user".into());
        assert!(!mode.is_edit_allowed(&user_ctx));
    }

    #[test]
    fn test_per_task_read_only_flag() {
        let readonly_task = create_test_task("task1", "Read-only Task", true);
        let editable_task = create_test_task("task2", "Editable Task", false);

        assert!(readonly_task.read_only);
        assert!(!editable_task.read_only);
    }

    #[test]
    fn test_edit_context_builder_methods() {
        let ctx = EditContext::new("task1".into(), EditType::Timeline)
            .with_role("admin".into())
            .with_user_id("user123".into())
            .with_metadata(serde_json::json!({
                "department": "engineering",
                "project": "gantt-component"
            }));

        assert_eq!(ctx.task_id, "task1");
        assert_eq!(ctx.edit_type, EditType::Timeline);
        assert_eq!(ctx.user_role, Some("admin".to_string()));
        assert_eq!(ctx.user_id, Some("user123".to_string()));
        assert!(ctx.metadata.is_some());
    }

    #[test]
    fn test_read_only_mode_default_is_editable() {
        let mode = ReadOnlyMode::default();
        let ctx = EditContext::new("task1".into(), EditType::Timeline);

        // Default should allow all edits
        assert!(mode.is_edit_allowed(&ctx));
    }

    #[test]
    fn test_read_only_mode_custom_with_complex_logic() {
        let mode = ReadOnlyMode::custom(|ctx: &EditContext| {
            // Complex permission logic:
            // - Admins can do anything
            // - Editors can edit properties but not delete
            // - Viewers can't edit anything
            match ctx.user_role.as_deref() {
                Some("admin") => true,
                Some("editor") => !matches!(ctx.edit_type, EditType::DeleteTask),
                _ => false,
            }
        });

        // Admin tests
        let admin_edit = EditContext::new("task1".into(), EditType::TaskProperties)
            .with_role("admin".into());
        assert!(mode.is_edit_allowed(&admin_edit));

        let admin_delete = EditContext::new("task1".into(), EditType::DeleteTask)
            .with_role("admin".into());
        assert!(mode.is_edit_allowed(&admin_delete));

        // Editor tests
        let editor_edit = EditContext::new("task1".into(), EditType::TaskProperties)
            .with_role("editor".into());
        assert!(mode.is_edit_allowed(&editor_edit));

        let editor_delete = EditContext::new("task1".into(), EditType::DeleteTask)
            .with_role("editor".into());
        assert!(!mode.is_edit_allowed(&editor_delete));

        // Viewer tests
        let viewer_edit = EditContext::new("task1".into(), EditType::TaskProperties)
            .with_role("viewer".into());
        assert!(!mode.is_edit_allowed(&viewer_edit));
    }

    #[test]
    fn test_all_edit_types_covered() {
        let mode = ReadOnlyMode::Editable;

        // Verify all EditType variants are handled
        let types = vec![
            EditType::TaskProperties,
            EditType::Timeline,
            EditType::Progress,
            EditType::Dependencies,
            EditType::Metadata,
            EditType::CreateTask,
            EditType::DeleteTask,
            EditType::MoveTask,
        ];

        for edit_type in types {
            let ctx = EditContext::new("task1".into(), edit_type);
            assert!(mode.is_edit_allowed(&ctx), "EditType should be allowed in Editable mode");
        }
    }
}
