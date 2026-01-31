use std::rc::Rc;

/// Drag state for managing card dragging operations
#[derive(Clone, Debug)]
#[derive(Default)]
pub struct DragState {
    /// The card currently being dragged (card_id)
    pub dragged_card: Option<String>,
    /// The source column of the dragged card
    pub source_column: Option<String>,
    /// The column currently being hovered over (drop target)
    pub target_column: Option<String>,
    /// Position where the card should be inserted (index)
    pub target_position: Option<usize>,
}


impl DragState {
    /// Check if a card is currently being dragged
    pub fn is_dragging(&self) -> bool {
        self.dragged_card.is_some()
    }

    /// Check if a specific column is the drop target
    pub fn is_drop_target(&self, column_id: &str) -> bool {
        self.target_column.as_ref().map(|id| id == column_id).unwrap_or(false)
    }

    /// Start dragging a card
    pub fn start_drag(&mut self, card_id: String, column_id: String) {
        self.dragged_card = Some(card_id);
        self.source_column = Some(column_id);
        self.target_column = None;
        self.target_position = None;
    }

    /// Update drop target
    pub fn update_target(&mut self, column_id: String, position: Option<usize>) {
        self.target_column = Some(column_id);
        self.target_position = position;
    }

    /// Clear drop target
    pub fn clear_target(&mut self) {
        self.target_column = None;
        self.target_position = None;
    }

    /// End dragging (cancel or complete)
    pub fn end_drag(&mut self) {
        self.dragged_card = None;
        self.source_column = None;
        self.target_column = None;
        self.target_position = None;
    }
}

/// Callback for drag operations
pub type DragCallback = Rc<dyn Fn(DragOperation)>;

/// Drag operation data
#[derive(Clone, Debug)]
pub struct DragOperation {
    /// The card being moved
    pub card_id: String,
    /// Source column ID
    pub from_column: String,
    /// Target column ID
    pub to_column: String,
    /// Target position in the column
    pub to_position: usize,
}

impl DragOperation {
    /// Create a new drag operation
    pub fn new(
        card_id: String,
        from_column: String,
        to_column: String,
        to_position: usize,
    ) -> Self {
        Self {
            card_id,
            from_column,
            to_column,
            to_position,
        }
    }
}

/// Validation result for drag operations
#[derive(Clone, Debug, PartialEq)]
pub enum DragValidation {
    /// The drop is allowed
    Allow,
    /// The drop is not allowed (with optional reason)
    Deny(Option<String>),
    /// Show a warning but allow the drop
    Warn(String),
}

/// Callback for validating drag operations
pub type DragValidator = Rc<dyn Fn(&DragOperation) -> DragValidation>;
