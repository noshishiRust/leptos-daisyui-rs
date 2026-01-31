/// Drag and drop interaction system for Gantt chart
///
/// Provides complete drag-and-drop functionality including:
/// - Moving tasks (shifting dates)
/// - Resizing tasks (changing start/end dates)
/// - Dragging progress indicator
/// - Visual feedback and constraints
use chrono::{DateTime, Datelike, Duration, Utc};
use serde::{Deserialize, Serialize};

/// Mode of drag operation being performed
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DragMode {
    /// Dragging entire task bar to shift dates
    Move,

    /// Dragging left edge to change start date
    ResizeStart,

    /// Dragging right edge to change end date
    ResizeEnd,

    /// Dragging progress indicator
    DragProgress,

    /// Creating a dependency by dragging from one task to another
    CreateDependency,
}

/// State tracking for drag operations
#[derive(Clone, Debug)]
pub struct DragState {
    /// Whether a drag operation is currently active
    pub is_dragging: bool,

    /// ID of the task being dragged (or source task for dependency creation)
    pub task_id: Option<String>,

    /// Type of drag operation
    pub drag_mode: Option<DragMode>,

    /// X position where drag started (pixels)
    pub start_x: f64,

    /// Y position where drag started (pixels)
    pub start_y: f64,

    /// Current X position during drag (pixels)
    pub current_x: f64,

    /// Current Y position during drag (pixels)
    pub current_y: f64,

    /// Target task ID (for dependency creation)
    pub target_task_id: Option<String>,

    /// Original task start date before drag
    pub original_start: Option<DateTime<Utc>>,

    /// Original task end date before drag
    pub original_end: Option<DateTime<Utc>>,

    /// Original progress value before drag
    pub original_progress: Option<f32>,

    /// Column width in pixels (for date calculations)
    pub column_width: f64,

    /// Whether to show visual preview during drag
    pub show_preview: bool,

    /// Snap to grid enabled
    pub snap_to_grid: bool,
}

impl Default for DragState {
    fn default() -> Self {
        Self {
            is_dragging: false,
            task_id: None,
            drag_mode: None,
            start_x: 0.0,
            start_y: 0.0,
            current_x: 0.0,
            current_y: 0.0,
            target_task_id: None,
            original_start: None,
            original_end: None,
            original_progress: None,
            column_width: 60.0, // Default day width
            show_preview: true,
            snap_to_grid: false,
        }
    }
}

impl DragState {
    /// Start a drag operation
    #[allow(clippy::too_many_arguments)]
    pub fn start_drag(
        &mut self,
        task_id: String,
        mode: DragMode,
        start_x: f64,
        start_y: f64,
        original_start: DateTime<Utc>,
        original_end: DateTime<Utc>,
        original_progress: f32,
        column_width: f64,
    ) {
        self.is_dragging = true;
        self.task_id = Some(task_id);
        self.drag_mode = Some(mode);
        self.start_x = start_x;
        self.start_y = start_y;
        self.current_x = start_x;
        self.current_y = start_y;
        self.target_task_id = None;
        self.original_start = Some(original_start);
        self.original_end = Some(original_end);
        self.original_progress = Some(original_progress);
        self.column_width = column_width;
    }

    /// Start a dependency creation drag
    pub fn start_dependency_drag(&mut self, source_task_id: String, start_x: f64, start_y: f64) {
        self.is_dragging = true;
        self.task_id = Some(source_task_id);
        self.drag_mode = Some(DragMode::CreateDependency);
        self.start_x = start_x;
        self.start_y = start_y;
        self.current_x = start_x;
        self.current_y = start_y;
        self.target_task_id = None;
        self.original_start = None;
        self.original_end = None;
        self.original_progress = None;
    }

    /// Update drag position
    pub fn update_position(&mut self, current_x: f64, current_y: f64) {
        if self.is_dragging {
            self.current_x = current_x;
            self.current_y = current_y;
        }
    }

    /// Set the target task for dependency creation
    pub fn set_target_task(&mut self, target_task_id: Option<String>) {
        if self.drag_mode == Some(DragMode::CreateDependency) {
            self.target_task_id = target_task_id;
        }
    }

    /// End drag operation
    pub fn end_drag(&mut self) {
        self.is_dragging = false;
        self.task_id = None;
        self.drag_mode = None;
        self.start_x = 0.0;
        self.start_y = 0.0;
        self.current_x = 0.0;
        self.current_y = 0.0;
        self.target_task_id = None;
        self.original_start = None;
        self.original_end = None;
        self.original_progress = None;
    }

    /// Calculate pixel delta from start of drag
    pub fn delta_x(&self) -> f64 {
        self.current_x - self.start_x
    }

    /// Convert pixel delta to time duration
    pub fn delta_to_duration(&self, delta_x: f64) -> Duration {
        let days = (delta_x / self.column_width).round() as i64;
        Duration::days(days)
    }

    /// Calculate new dates based on drag mode and position
    pub fn calculate_new_dates(&self) -> Option<(DateTime<Utc>, DateTime<Utc>)> {
        if !self.is_dragging {
            return None;
        }

        let original_start = self.original_start?;
        let original_end = self.original_end?;
        let mode = self.drag_mode?;

        let mut delta_x = self.delta_x();

        // Apply snap to grid if enabled
        if self.snap_to_grid {
            let snapped_days = (delta_x / self.column_width).round();
            delta_x = snapped_days * self.column_width;
        }

        let time_delta = self.delta_to_duration(delta_x);

        match mode {
            DragMode::Move => {
                // Shift both dates by the same amount
                let new_start = original_start + time_delta;
                let new_end = original_end + time_delta;
                Some((new_start, new_end))
            }
            DragMode::ResizeStart => {
                // Only change start date, keep end fixed
                let new_start = original_start + time_delta;
                Some((new_start, original_end))
            }
            DragMode::ResizeEnd => {
                // Only change end date, keep start fixed
                let new_end = original_end + time_delta;
                Some((original_start, new_end))
            }
            DragMode::DragProgress => {
                // Progress drag doesn't change dates
                Some((original_start, original_end))
            }
            DragMode::CreateDependency => {
                // Dependency creation doesn't change dates
                None
            }
        }
    }

    /// Calculate new progress value based on drag position
    pub fn calculate_new_progress(&self, task_bar_width: f64) -> Option<f32> {
        if !self.is_dragging || self.drag_mode != Some(DragMode::DragProgress) {
            return None;
        }

        let delta_x = self.delta_x();
        let progress_delta = delta_x / task_bar_width;
        let original_progress = self.original_progress?;

        let new_progress = (original_progress + progress_delta as f32).clamp(0.0, 1.0);
        Some(new_progress)
    }
}

/// Custom validation function type for drag constraints
pub type DateValidator = fn(&DateTime<Utc>, &DateTime<Utc>) -> Result<(), String>;

/// Drag constraints and validation
#[derive(Clone, Debug)]
pub struct DragConstraints {
    /// Minimum task duration in days
    pub min_duration_days: i64,

    /// Maximum task duration in days (None = unlimited)
    pub max_duration_days: Option<i64>,

    /// Only allow dragging to working days (skip weekends)
    pub working_days_only: bool,

    /// Earliest allowed date
    pub min_date: Option<DateTime<Utc>>,

    /// Latest allowed date
    pub max_date: Option<DateTime<Utc>>,

    /// Custom validation callback
    pub custom_validator: Option<DateValidator>,
}

impl Default for DragConstraints {
    fn default() -> Self {
        Self {
            min_duration_days: 1,
            max_duration_days: None,
            working_days_only: false,
            min_date: None,
            max_date: None,
            custom_validator: None,
        }
    }
}

impl DragConstraints {
    /// Validate proposed new dates against constraints
    pub fn validate(
        &self,
        new_start: DateTime<Utc>,
        new_end: DateTime<Utc>,
    ) -> Result<(DateTime<Utc>, DateTime<Utc>), String> {
        // Check date order
        if new_start >= new_end {
            return Err("Start date must be before end date".to_string());
        }

        // Check minimum duration
        let duration = new_end.signed_duration_since(new_start);
        if duration < Duration::days(self.min_duration_days) {
            return Err(format!(
                "Task duration must be at least {} days",
                self.min_duration_days
            ));
        }

        // Check maximum duration
        if let Some(max_days) = self.max_duration_days
            && duration > Duration::days(max_days)
        {
            return Err(format!("Task duration cannot exceed {} days", max_days));
        }

        // Check min/max dates
        if let Some(min_date) = self.min_date
            && new_start < min_date
        {
            return Err("Start date is before minimum allowed date".to_string());
        }

        if let Some(max_date) = self.max_date
            && new_end > max_date
        {
            return Err("End date is after maximum allowed date".to_string());
        }

        // Apply working days constraint
        let (validated_start, validated_end) = if self.working_days_only {
            (
                self.snap_to_working_day(new_start),
                self.snap_to_working_day(new_end),
            )
        } else {
            (new_start, new_end)
        };

        // Custom validation
        if let Some(validator) = self.custom_validator {
            validator(&validated_start, &validated_end)?;
        }

        Ok((validated_start, validated_end))
    }

    /// Snap a date to the nearest working day (skip weekends)
    fn snap_to_working_day(&self, date: DateTime<Utc>) -> DateTime<Utc> {
        let weekday = date.weekday();

        // If Saturday (6), move to Monday (+2 days)
        // If Sunday (0), move to Monday (+1 day)
        use chrono::Weekday;
        match weekday {
            Weekday::Sat => date + Duration::days(2),
            Weekday::Sun => date + Duration::days(1),
            _ => date,
        }
    }
}

/// Result of a drag operation attempt
#[derive(Clone, Debug)]
pub enum DragResult {
    /// Drag operation succeeded with new dates
    Success {
        /// ID of the task that was dragged
        task_id: String,
        /// New start date after drag
        new_start: DateTime<Utc>,
        /// New end date after drag
        new_end: DateTime<Utc>,
        /// New progress value (if changed)
        new_progress: Option<f32>,
    },

    /// Drag operation was cancelled
    Cancelled,

    /// Drag operation failed validation
    ValidationError(String),
}

/// Undo/Redo state for drag operations
#[derive(Clone, Debug)]
pub struct DragHistory {
    /// Stack of previous states (for undo)
    undo_stack: Vec<DragHistoryEntry>,

    /// Stack of undone states (for redo)
    redo_stack: Vec<DragHistoryEntry>,

    /// Maximum history size
    max_size: usize,
}

#[derive(Clone, Debug)]
struct DragHistoryEntry {
    task_id: String,
    old_start: DateTime<Utc>,
    old_end: DateTime<Utc>,
    old_progress: f32,
    new_start: DateTime<Utc>,
    new_end: DateTime<Utc>,
    new_progress: f32,
}

impl Default for DragHistory {
    fn default() -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_size: 50,
        }
    }
}

impl DragHistory {
    /// Record a drag operation
    #[allow(clippy::too_many_arguments)]
    pub fn record(
        &mut self,
        task_id: String,
        old_start: DateTime<Utc>,
        old_end: DateTime<Utc>,
        old_progress: f32,
        new_start: DateTime<Utc>,
        new_end: DateTime<Utc>,
        new_progress: f32,
    ) {
        let entry = DragHistoryEntry {
            task_id,
            old_start,
            old_end,
            old_progress,
            new_start,
            new_end,
            new_progress,
        };

        self.undo_stack.push(entry);

        // Clear redo stack when new action is performed
        self.redo_stack.clear();

        // Limit stack size
        if self.undo_stack.len() > self.max_size {
            self.undo_stack.remove(0);
        }
    }

    /// Undo last drag operation
    pub fn undo(&mut self) -> Option<(String, DateTime<Utc>, DateTime<Utc>, f32)> {
        if let Some(entry) = self.undo_stack.pop() {
            let result = (
                entry.task_id.clone(),
                entry.old_start,
                entry.old_end,
                entry.old_progress,
            );

            self.redo_stack.push(entry);
            Some(result)
        } else {
            None
        }
    }

    /// Redo last undone operation
    pub fn redo(&mut self) -> Option<(String, DateTime<Utc>, DateTime<Utc>, f32)> {
        if let Some(entry) = self.redo_stack.pop() {
            let result = (
                entry.task_id.clone(),
                entry.new_start,
                entry.new_end,
                entry.new_progress,
            );

            self.undo_stack.push(entry);
            Some(result)
        } else {
            None
        }
    }

    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Check if redo is available
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Clear all history
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
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
        assert_eq!(state.column_width, 60.0);
    }

    #[test]
    fn test_start_drag() {
        let mut state = DragState::default();
        let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2024, 1, 5, 0, 0, 0).unwrap();

        state.start_drag(
            "task-1".to_string(),
            DragMode::Move,
            100.0,
            0.0,
            start,
            end,
            0.5,
            60.0,
        );

        assert!(state.is_dragging);
        assert_eq!(state.task_id, Some("task-1".to_string()));
        assert_eq!(state.drag_mode, Some(DragMode::Move));
        assert_eq!(state.start_x, 100.0);
    }

    #[test]
    fn test_calculate_move_dates() {
        let mut state = DragState::default();
        let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2024, 1, 5, 0, 0, 0).unwrap();

        state.start_drag(
            "task-1".to_string(),
            DragMode::Move,
            0.0,
            0.0,
            start,
            end,
            0.5,
            60.0,
        );

        // Move 2 days forward (120 pixels)
        state.update_position(120.0, 0.0);

        let (new_start, new_end) = state.calculate_new_dates().unwrap();
        assert_eq!(new_start, start + Duration::days(2));
        assert_eq!(new_end, end + Duration::days(2));
    }

    #[test]
    fn test_calculate_resize_start() {
        let mut state = DragState::default();
        let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2024, 1, 5, 0, 0, 0).unwrap();

        state.start_drag(
            "task-1".to_string(),
            DragMode::ResizeStart,
            0.0,
            0.0,
            start,
            end,
            0.5,
            60.0,
        );

        // Drag left edge 1 day later (60 pixels)
        state.update_position(60.0, 0.0);

        let (new_start, new_end) = state.calculate_new_dates().unwrap();
        assert_eq!(new_start, start + Duration::days(1));
        assert_eq!(new_end, end); // End unchanged
    }

    #[test]
    fn test_drag_constraints_min_duration() {
        let constraints = DragConstraints {
            min_duration_days: 2,
            ..Default::default()
        };

        let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2024, 1, 2, 0, 0, 0).unwrap(); // Only 1 day

        let result = constraints.validate(start, end);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("at least 2 days"));
    }

    #[test]
    fn test_drag_constraints_working_days() {
        let constraints = DragConstraints {
            working_days_only: true,
            ..Default::default()
        };

        // Saturday
        let saturday = Utc.with_ymd_and_hms(2024, 1, 6, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2024, 1, 10, 0, 0, 0).unwrap();

        let (validated_start, _) = constraints.validate(saturday, end).unwrap();

        // Should be moved to Monday (Jan 8)
        let monday = Utc.with_ymd_and_hms(2024, 1, 8, 0, 0, 0).unwrap();
        assert_eq!(validated_start, monday);
    }

    #[test]
    fn test_drag_history() {
        let mut history = DragHistory::default();

        let old_start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let old_end = Utc.with_ymd_and_hms(2024, 1, 5, 0, 0, 0).unwrap();
        let new_start = Utc.with_ymd_and_hms(2024, 1, 3, 0, 0, 0).unwrap();
        let new_end = Utc.with_ymd_and_hms(2024, 1, 7, 0, 0, 0).unwrap();

        history.record(
            "task-1".to_string(),
            old_start,
            old_end,
            0.5,
            new_start,
            new_end,
            0.5,
        );

        assert!(history.can_undo());
        assert!(!history.can_redo());

        let (task_id, undo_start, undo_end, _) = history.undo().unwrap();
        assert_eq!(task_id, "task-1");
        assert_eq!(undo_start, old_start);
        assert_eq!(undo_end, old_end);

        assert!(history.can_redo());
    }

    #[test]
    fn test_snap_to_grid() {
        let mut state = DragState {
            snap_to_grid: true,
            ..Default::default()
        };

        let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2024, 1, 5, 0, 0, 0).unwrap();

        state.start_drag(
            "task-1".to_string(),
            DragMode::Move,
            0.0,
            0.0,
            start,
            end,
            0.5,
            60.0,
        );

        // Drag 75 pixels (should snap to 60 = 1 day)
        state.update_position(75.0, 0.0);

        let (new_start, new_end) = state.calculate_new_dates().unwrap();
        assert_eq!(new_start, start + Duration::days(1));
        assert_eq!(new_end, end + Duration::days(1));
    }

    #[test]
    fn test_start_dependency_drag() {
        let mut state = DragState::default();

        state.start_dependency_drag("task-1".to_string(), 100.0, 50.0);

        assert!(state.is_dragging);
        assert_eq!(state.task_id, Some("task-1".to_string()));
        assert_eq!(state.drag_mode, Some(DragMode::CreateDependency));
        assert_eq!(state.start_x, 100.0);
        assert_eq!(state.start_y, 50.0);
        assert!(state.target_task_id.is_none());
    }

    #[test]
    fn test_set_target_task() {
        let mut state = DragState::default();

        state.start_dependency_drag("task-1".to_string(), 100.0, 50.0);
        state.set_target_task(Some("task-2".to_string()));

        assert_eq!(state.target_task_id, Some("task-2".to_string()));

        // Clear target
        state.set_target_task(None);
        assert!(state.target_task_id.is_none());
    }

    #[test]
    fn test_dependency_drag_no_date_changes() {
        let mut state = DragState::default();

        state.start_dependency_drag("task-1".to_string(), 100.0, 50.0);
        state.update_position(200.0, 100.0);

        // Dependency creation doesn't change dates
        assert!(state.calculate_new_dates().is_none());
    }
}
