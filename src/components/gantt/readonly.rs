/// Read-only mode and permission system for Gantt chart
///
/// Provides granular control over which aspects of the Gantt chart are editable.
/// Supports full read-only, timeline-only editing, grid-only editing, custom logic,
/// and per-task/per-column read-only flags.
///
/// # Examples
///
/// ```rust
/// use leptos_daisyui_rs::components::gantt::{ReadOnlyMode, EditContext, EditType};
///
/// // Full read-only mode (no editing allowed)
/// let mode = ReadOnlyMode::Full;
///
/// // Timeline-only editing (dates can be changed, but not task properties)
/// let mode = ReadOnlyMode::TimelineOnly;
///
/// // Custom read-only logic based on user role
/// let mode = ReadOnlyMode::custom(|ctx: &EditContext| {
///     // Only admins can edit critical tasks
///     ctx.user_role == Some("admin".to_string())
/// });
/// ```
use std::sync::Arc;

/// Type of edit operation being attempted
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EditType {
    /// Editing task name or description
    TaskProperties,
    /// Changing task start/end dates or duration
    Timeline,
    /// Editing progress percentage
    Progress,
    /// Changing task dependencies
    Dependencies,
    /// Modifying task metadata (priority, assignees, etc.)
    Metadata,
    /// Creating a new task
    CreateTask,
    /// Deleting a task
    DeleteTask,
    /// Moving task to different position
    MoveTask,
}

/// Context provided to custom read-only evaluation logic
#[derive(Clone, Debug)]
pub struct EditContext {
    /// ID of the task being edited
    pub task_id: String,

    /// Type of edit operation
    pub edit_type: EditType,

    /// Optional user role for RBAC
    pub user_role: Option<String>,

    /// Optional user ID for fine-grained permissions
    pub user_id: Option<String>,

    /// Additional context data (JSON-serializable)
    pub metadata: Option<serde_json::Value>,
}

impl EditContext {
    /// Create a new edit context for a task
    pub fn new(task_id: String, edit_type: EditType) -> Self {
        Self {
            task_id,
            edit_type,
            user_role: None,
            user_id: None,
            metadata: None,
        }
    }

    /// Set the user role for RBAC evaluation
    pub fn with_role(mut self, role: String) -> Self {
        self.user_role = Some(role);
        self
    }

    /// Set the user ID for fine-grained permissions
    pub fn with_user_id(mut self, user_id: String) -> Self {
        self.user_id = Some(user_id);
        self
    }

    /// Set additional metadata for custom logic
    pub fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

/// Custom read-only evaluation function
///
/// Returns `true` if the edit should be allowed, `false` to block it.
/// Function must be Send + Sync for thread safety.
pub type ReadOnlyEvaluator = Arc<dyn Fn(&EditContext) -> bool + Send + Sync>;

/// Read-only mode configuration for Gantt chart
#[derive(Clone)]
#[derive(Default)]
pub enum ReadOnlyMode {
    /// Fully editable (no restrictions)
    #[default]
    Editable,

    /// Fully read-only (no editing allowed)
    Full,

    /// Only timeline editing allowed (dates, duration)
    TimelineOnly,

    /// Only grid/properties editing allowed (name, metadata, etc.)
    GridOnly,

    /// Custom read-only logic via callback
    ///
    /// The callback receives an `EditContext` and returns `true` to allow
    /// the edit or `false` to block it.
    Custom(ReadOnlyEvaluator),
}

impl ReadOnlyMode {
    /// Check if an edit is allowed based on this read-only mode
    pub fn is_edit_allowed(&self, context: &EditContext) -> bool {
        match self {
            ReadOnlyMode::Editable => true,
            ReadOnlyMode::Full => false,
            ReadOnlyMode::TimelineOnly => matches!(
                context.edit_type,
                EditType::Timeline
            ),
            ReadOnlyMode::GridOnly => matches!(
                context.edit_type,
                EditType::TaskProperties
                    | EditType::Progress
                    | EditType::Metadata
            ),
            ReadOnlyMode::Custom(evaluator) => evaluator(context),
        }
    }

    /// Create a custom read-only mode with a static function
    pub fn custom<F>(f: F) -> Self
    where
        F: Fn(&EditContext) -> bool + Send + Sync + 'static,
    {
        ReadOnlyMode::Custom(Arc::new(f))
    }
}


impl std::fmt::Debug for ReadOnlyMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReadOnlyMode::Editable => write!(f, "ReadOnlyMode::Editable"),
            ReadOnlyMode::Full => write!(f, "ReadOnlyMode::Full"),
            ReadOnlyMode::TimelineOnly => write!(f, "ReadOnlyMode::TimelineOnly"),
            ReadOnlyMode::GridOnly => write!(f, "ReadOnlyMode::GridOnly"),
            ReadOnlyMode::Custom(_) => write!(f, "ReadOnlyMode::Custom(...)"),
        }
    }
}

/// Builder for complex read-only configurations
///
/// # Examples
///
/// ```rust
/// use leptos_daisyui_rs::components::gantt::{ReadOnlyBuilder, EditType};
///
/// let config = ReadOnlyBuilder::new()
///     .allow_timeline_edits(false)
///     .allow_progress_updates(true)
///     .require_role("editor")
///     .build();
/// ```
#[derive(Default)]
pub struct ReadOnlyBuilder {
    allow_timeline: Option<bool>,
    allow_properties: Option<bool>,
    allow_progress: Option<bool>,
    allow_dependencies: Option<bool>,
    allow_metadata: Option<bool>,
    allow_create: Option<bool>,
    allow_delete: Option<bool>,
    allow_move: Option<bool>,
    required_role: Option<String>,
}

impl ReadOnlyBuilder {
    /// Create a new read-only configuration builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Allow or disallow timeline edits (dates, duration)
    pub fn allow_timeline_edits(mut self, allow: bool) -> Self {
        self.allow_timeline = Some(allow);
        self
    }

    /// Allow or disallow task property edits (name, description)
    pub fn allow_property_edits(mut self, allow: bool) -> Self {
        self.allow_properties = Some(allow);
        self
    }

    /// Allow or disallow progress updates
    pub fn allow_progress_updates(mut self, allow: bool) -> Self {
        self.allow_progress = Some(allow);
        self
    }

    /// Allow or disallow dependency edits
    pub fn allow_dependency_edits(mut self, allow: bool) -> Self {
        self.allow_dependencies = Some(allow);
        self
    }

    /// Allow or disallow metadata edits
    pub fn allow_metadata_edits(mut self, allow: bool) -> Self {
        self.allow_metadata = Some(allow);
        self
    }

    /// Allow or disallow task creation
    pub fn allow_task_creation(mut self, allow: bool) -> Self {
        self.allow_create = Some(allow);
        self
    }

    /// Allow or disallow task deletion
    pub fn allow_task_deletion(mut self, allow: bool) -> Self {
        self.allow_delete = Some(allow);
        self
    }

    /// Allow or disallow task movement
    pub fn allow_task_movement(mut self, allow: bool) -> Self {
        self.allow_move = Some(allow);
        self
    }

    /// Require a specific user role for any edits
    pub fn require_role(mut self, role: impl Into<String>) -> Self {
        self.required_role = Some(role.into());
        self
    }

    /// Build the read-only mode configuration
    pub fn build(self) -> ReadOnlyMode {
        let required_role = self.required_role.clone();
        let allow_timeline = self.allow_timeline.unwrap_or(true);
        let allow_properties = self.allow_properties.unwrap_or(true);
        let allow_progress = self.allow_progress.unwrap_or(true);
        let allow_dependencies = self.allow_dependencies.unwrap_or(true);
        let allow_metadata = self.allow_metadata.unwrap_or(true);
        let allow_create = self.allow_create.unwrap_or(true);
        let allow_delete = self.allow_delete.unwrap_or(true);
        let allow_move = self.allow_move.unwrap_or(true);

        ReadOnlyMode::custom(move |ctx| {
            // Check role if required
            if let Some(ref role) = required_role
                && ctx.user_role.as_ref() != Some(role) {
                    return false;
                }

            // Check edit type permissions
            match ctx.edit_type {
                EditType::Timeline => allow_timeline,
                EditType::TaskProperties => allow_properties,
                EditType::Progress => allow_progress,
                EditType::Dependencies => allow_dependencies,
                EditType::Metadata => allow_metadata,
                EditType::CreateTask => allow_create,
                EditType::DeleteTask => allow_delete,
                EditType::MoveTask => allow_move,
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_only_mode_editable() {
        let mode = ReadOnlyMode::Editable;
        let ctx = EditContext::new("task1".into(), EditType::Timeline);
        assert!(mode.is_edit_allowed(&ctx));
    }

    #[test]
    fn test_read_only_mode_full() {
        let mode = ReadOnlyMode::Full;
        let ctx = EditContext::new("task1".into(), EditType::Timeline);
        assert!(!mode.is_edit_allowed(&ctx));
    }

    #[test]
    fn test_read_only_mode_timeline_only() {
        let mode = ReadOnlyMode::TimelineOnly;

        let timeline_ctx = EditContext::new("task1".into(), EditType::Timeline);
        assert!(mode.is_edit_allowed(&timeline_ctx));

        let props_ctx = EditContext::new("task1".into(), EditType::TaskProperties);
        assert!(!mode.is_edit_allowed(&props_ctx));
    }

    #[test]
    fn test_read_only_mode_grid_only() {
        let mode = ReadOnlyMode::GridOnly;

        let props_ctx = EditContext::new("task1".into(), EditType::TaskProperties);
        assert!(mode.is_edit_allowed(&props_ctx));

        let timeline_ctx = EditContext::new("task1".into(), EditType::Timeline);
        assert!(!mode.is_edit_allowed(&timeline_ctx));
    }

    #[test]
    fn test_read_only_mode_custom() {
        let mode = ReadOnlyMode::custom(|ctx| {
            ctx.user_role == Some("admin".to_string())
        });

        let admin_ctx = EditContext::new("task1".into(), EditType::Timeline)
            .with_role("admin".into());
        assert!(mode.is_edit_allowed(&admin_ctx));

        let user_ctx = EditContext::new("task1".into(), EditType::Timeline)
            .with_role("user".into());
        assert!(!mode.is_edit_allowed(&user_ctx));
    }

    #[test]
    fn test_read_only_builder() {
        let mode = ReadOnlyBuilder::new()
            .allow_timeline_edits(false)
            .allow_progress_updates(true)
            .build();

        let timeline_ctx = EditContext::new("task1".into(), EditType::Timeline);
        assert!(!mode.is_edit_allowed(&timeline_ctx));

        let progress_ctx = EditContext::new("task1".into(), EditType::Progress);
        assert!(mode.is_edit_allowed(&progress_ctx));
    }

    #[test]
    fn test_read_only_builder_with_role() {
        let mode = ReadOnlyBuilder::new()
            .require_role("editor")
            .build();

        let editor_ctx = EditContext::new("task1".into(), EditType::Timeline)
            .with_role("editor".into());
        assert!(mode.is_edit_allowed(&editor_ctx));

        let viewer_ctx = EditContext::new("task1".into(), EditType::Timeline)
            .with_role("viewer".into());
        assert!(!mode.is_edit_allowed(&viewer_ctx));
    }
}
