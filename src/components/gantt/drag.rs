use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Type of drag operation being performed
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DragMode {
    /// Moving the entire task (changing both start and end dates)
    Move,

    /// Resizing the task by dragging the start edge
    ResizeStart,

    /// Resizing the task by dragging the end edge
    ResizeEnd,

    /// Dragging the progress indicator
    Progress,
}

/// State of an active drag operation
#[derive(Clone, Debug, PartialEq)]
pub struct DragState {
    /// Whether a drag operation is currently active
    pub is_dragging: bool,

    /// ID of the task being dragged
    pub task_id: Option<String>,

    /// Type of drag operation
    pub drag_mode: Option<DragMode>,

    /// X position where the drag started (in pixels)
    pub start_x: f64,

    /// Current X position during drag (in pixels)
    pub current_x: f64,

    /// Original start date of the task (before drag)
    pub original_start: Option<DateTime<Utc>>,

    /// Original end date of the task (before drag)
    pub original_end: Option<DateTime<Utc>>,

    /// Original progress value (before drag)
    pub original_progress: Option<f32>,
}

impl Default for DragState {
    fn default() -> Self {
        Self {
            is_dragging: false,
            task_id: None,
            drag_mode: None,
            start_x: 0.0,
            current_x: 0.0,
            original_start: None,
            original_end: None,
            original_progress: None,
        }
    }
}

impl DragState {
    /// Create a new empty drag state
    pub fn new() -> Self {
        Self::default()
    }

    /// Start a new drag operation
    pub fn start_drag(
        &mut self,
        task_id: String,
        mode: DragMode,
        x: f64,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
        progress: f32,
    ) {
        self.is_dragging = true;
        self.task_id = Some(task_id);
        self.drag_mode = Some(mode);
        self.start_x = x;
        self.current_x = x;
        self.original_start = Some(start_date);
        self.original_end = Some(end_date);
        self.original_progress = Some(progress);
    }

    /// Update the current position during drag
    pub fn update_position(&mut self, x: f64) {
        self.current_x = x;
    }

    /// End the drag operation and return to idle state
    pub fn end_drag(&mut self) {
        *self = Self::default();
    }

    /// Calculate the delta in pixels from the start of the drag
    pub fn delta_x(&self) -> f64 {
        self.current_x - self.start_x
    }

    /// Check if currently dragging a specific task
    pub fn is_dragging_task(&self, task_id: &str) -> bool {
        self.is_dragging && self.task_id.as_deref() == Some(task_id)
    }
}

/// Validation result for drag operations
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DragValidation {
    /// Drag operation is allowed
    Allow,

    /// Drag operation is not allowed with a reason
    Deny(String),

    /// Drag operation is allowed but with a warning
    Warn(String),
}

/// Information about a completed drag operation
#[derive(Clone, Debug, PartialEq)]
pub struct DragResult {
    /// ID of the task that was dragged
    pub task_id: String,

    /// Type of drag operation performed
    pub drag_mode: DragMode,

    /// New start date (if changed)
    pub new_start: Option<DateTime<Utc>>,

    /// New end date (if changed)
    pub new_end: Option<DateTime<Utc>>,

    /// New progress value (if changed)
    pub new_progress: Option<f32>,

    /// Original start date (before drag)
    pub original_start: DateTime<Utc>,

    /// Original end date (before drag)
    pub original_end: DateTime<Utc>,

    /// Original progress value (before drag)
    pub original_progress: f32,
}

impl DragResult {
    /// Check if the drag operation actually changed anything
    pub fn has_changes(&self) -> bool {
        self.new_start.is_some() || self.new_end.is_some() || self.new_progress.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_drag_state_default() {
        let state = DragState::default();
        assert!(!state.is_dragging);
        assert!(state.task_id.is_none());
        assert!(state.drag_mode.is_none());
        assert_eq!(state.start_x, 0.0);
        assert_eq!(state.current_x, 0.0);
    }

    #[test]
    fn test_start_drag() {
        let mut state = DragState::new();
        let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2024, 1, 10, 0, 0, 0).unwrap();

        state.start_drag("task-1".to_string(), DragMode::Move, 100.0, start, end, 0.5);

        assert!(state.is_dragging);
        assert_eq!(state.task_id, Some("task-1".to_string()));
        assert_eq!(state.drag_mode, Some(DragMode::Move));
        assert_eq!(state.start_x, 100.0);
        assert_eq!(state.original_start, Some(start));
        assert_eq!(state.original_end, Some(end));
        assert_eq!(state.original_progress, Some(0.5));
    }

    #[test]
    fn test_update_position() {
        let mut state = DragState::new();
        let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2024, 1, 10, 0, 0, 0).unwrap();

        state.start_drag("task-1".to_string(), DragMode::Move, 100.0, start, end, 0.5);
        state.update_position(150.0);

        assert_eq!(state.current_x, 150.0);
        assert_eq!(state.delta_x(), 50.0);
    }

    #[test]
    fn test_end_drag() {
        let mut state = DragState::new();
        let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2024, 1, 10, 0, 0, 0).unwrap();

        state.start_drag("task-1".to_string(), DragMode::Move, 100.0, start, end, 0.5);
        state.end_drag();

        assert!(!state.is_dragging);
        assert!(state.task_id.is_none());
        assert!(state.drag_mode.is_none());
    }

    #[test]
    fn test_is_dragging_task() {
        let mut state = DragState::new();
        let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2024, 1, 10, 0, 0, 0).unwrap();

        state.start_drag("task-1".to_string(), DragMode::Move, 100.0, start, end, 0.5);

        assert!(state.is_dragging_task("task-1"));
        assert!(!state.is_dragging_task("task-2"));
    }
}
