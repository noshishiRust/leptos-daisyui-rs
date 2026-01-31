use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Priority level for kanban cards
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Priority {
    /// Low priority
    Low,
    /// Medium priority (default)
    #[default]
    Medium,
    /// High priority
    High,
    /// Critical priority
    Critical,
}

impl Priority {
    /// Get the string representation of the priority
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
            Self::Critical => "critical",
        }
    }

    /// Get the daisyUI badge color class for this priority
    pub fn color_class(&self) -> &'static str {
        match self {
            Self::Low => "badge-info",
            Self::Medium => "badge-primary",
            Self::High => "badge-warning",
            Self::Critical => "badge-error",
        }
    }
}

/// Label/tag for categorizing cards
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Label {
    /// Unique identifier for the label
    pub id: String,
    /// Display name of the label
    pub name: String,
    /// Optional color for the label
    pub color: Option<String>,
}

impl Label {
    /// Create a new label with the given ID and name
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            color: None,
        }
    }

    /// Set the label color
    pub fn with_color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }
}

/// Assignee information for a card
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Assignee {
    /// Unique identifier for the assignee
    pub id: String,
    /// Display name of the assignee
    pub name: String,
    /// Optional avatar URL
    pub avatar_url: Option<String>,
    /// Automatically generated initials from name
    pub initials: String,
}

impl Assignee {
    /// Create a new assignee with auto-generated initials
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        let name_str = name.into();
        let initials = name_str
            .split_whitespace()
            .take(2)
            .filter_map(|word| word.chars().next())
            .collect::<String>()
            .to_uppercase();

        Self {
            id: id.into(),
            name: name_str,
            avatar_url: None,
            initials,
        }
    }

    /// Set the assignee avatar URL
    pub fn with_avatar(mut self, avatar_url: impl Into<String>) -> Self {
        self.avatar_url = Some(avatar_url.into());
        self
    }
}

/// A kanban card representing a task or item
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KanbanCard {
    /// Unique identifier for the card
    pub card_id: String,
    /// Card title
    pub title: String,
    /// Optional card description
    pub description: Option<String>,
    /// Labels/tags for categorizing the card
    pub labels: Vec<Label>,
    /// Assigned users
    pub assignees: Vec<Assignee>,
    /// Priority level
    pub priority: Priority,
    /// Optional due date
    pub due_date: Option<NaiveDate>,
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Additional metadata for extensibility
    pub metadata: HashMap<String, String>,
}

impl KanbanCard {
    /// Create a new card with the given ID and title
    pub fn new(card_id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            card_id: card_id.into(),
            title: title.into(),
            description: None,
            labels: Vec::new(),
            assignees: Vec::new(),
            priority: Priority::default(),
            due_date: None,
            created_at: chrono::Utc::now(),
            metadata: HashMap::new(),
        }
    }

    /// Set the card description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the card priority
    pub fn with_priority(mut self, priority: Priority) -> Self {
        self.priority = priority;
        self
    }

    /// Add a label to the card
    pub fn with_label(mut self, label: Label) -> Self {
        self.labels.push(label);
        self
    }

    /// Add an assignee to the card
    pub fn with_assignee(mut self, assignee: Assignee) -> Self {
        self.assignees.push(assignee);
        self
    }

    /// Set the card due date
    pub fn with_due_date(mut self, due_date: NaiveDate) -> Self {
        self.due_date = Some(due_date);
        self
    }

    /// Check if the card is overdue
    pub fn is_overdue(&self) -> bool {
        if let Some(due) = self.due_date {
            due < chrono::Local::now().date_naive()
        } else {
            false
        }
    }

    /// Check if card matches the provided filters
    pub fn matches_filters(&self, filters: &KanbanFilters) -> bool {
        let mut conditions = Vec::new();

        // Search query filter
        if let Some(ref query) = filters.search_query {
            let query_lower = query.to_lowercase();
            let matches = self.title.to_lowercase().contains(&query_lower)
                || self
                    .description
                    .as_ref()
                    .map(|d| d.to_lowercase().contains(&query_lower))
                    .unwrap_or(false);
            conditions.push(matches);
        }

        // Priority filter
        if !filters.priorities.is_empty() {
            conditions.push(filters.priorities.contains(&self.priority));
        }

        // Assignee filter
        if !filters.assignee_ids.is_empty() {
            let has_assignee = self
                .assignees
                .iter()
                .any(|a| filters.assignee_ids.contains(&a.id));
            conditions.push(has_assignee);
        }

        // Label filter
        if !filters.label_ids.is_empty() {
            let has_label = self
                .labels
                .iter()
                .any(|l| filters.label_ids.contains(&l.id));
            conditions.push(has_label);
        }

        // Due date range filter
        if filters.due_date_range.is_some() {
            // Placeholder for due date range logic
            conditions.push(true);
        }

        // Apply filter logic (AND/OR)
        if conditions.is_empty() {
            return true; // No filters applied
        }

        match filters.filter_logic {
            FilterLogic::And => conditions.iter().all(|&c| c),
            FilterLogic::Or => conditions.iter().any(|&c| c),
        }
    }
}

/// A kanban column containing cards
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KanbanColumn {
    /// Unique identifier for the column
    pub column_id: String,
    /// Display title of the column
    pub title: String,
    /// Optional color for the column header
    pub color: Option<String>,
    /// Maximum number of cards allowed in this column (WIP limit)
    pub card_limit: Option<usize>,
    /// Cards in this column
    pub cards: Vec<KanbanCard>,
    /// Whether the column is currently collapsed
    pub collapsed: bool,
    /// Whether the column can be collapsed
    pub collapsible: bool,
    /// Whether the column has independent scrolling
    pub scrollable: bool,
}

impl KanbanColumn {
    /// Create a new column with the given ID and title
    pub fn new(column_id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            column_id: column_id.into(),
            title: title.into(),
            color: None,
            card_limit: None,
            cards: Vec::new(),
            collapsed: false,
            collapsible: true,
            scrollable: true,
        }
    }

    /// Set the column header color
    pub fn with_color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }

    /// Set the WIP (Work In Progress) limit for this column
    pub fn with_card_limit(mut self, limit: usize) -> Self {
        self.card_limit = Some(limit);
        self
    }

    /// Add a single card to this column
    pub fn with_card(mut self, card: KanbanCard) -> Self {
        self.cards.push(card);
        self
    }

    /// Set all cards in this column
    pub fn with_cards(mut self, cards: Vec<KanbanCard>) -> Self {
        self.cards = cards;
        self
    }

    /// Check if the column has exceeded its WIP limit
    pub fn is_over_limit(&self) -> bool {
        if let Some(limit) = self.card_limit {
            self.cards.len() > limit
        } else {
            false
        }
    }

    /// Get the number of cards in this column
    pub fn card_count(&self) -> usize {
        self.cards.len()
    }
}

/// Filter criteria for kanban cards
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct KanbanFilters {
    /// Filter by assignee IDs
    pub assignee_ids: Vec<String>,
    /// Filter by label IDs
    pub label_ids: Vec<String>,
    /// Filter by priority levels
    pub priorities: Vec<Priority>,
    /// Filter by due date range (start, end)
    pub due_date_range: Option<(Option<NaiveDate>, Option<NaiveDate>)>,
    /// Search query for card title and description
    pub search_query: Option<String>,
    /// Logic for combining filters (AND/OR)
    pub filter_logic: FilterLogic,
}

/// Logic for combining multiple filters
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum FilterLogic {
    /// All filters must match (AND)
    #[default]
    And,
    /// Any filter can match (OR)
    Or,
}

impl KanbanFilters {
    /// Create a new empty filter set
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if a card matches the current filters
    pub fn matches_card(&self, card: &KanbanCard) -> bool {
        let filters_active = !self.assignee_ids.is_empty()
            || !self.label_ids.is_empty()
            || !self.priorities.is_empty()
            || self.due_date_range.is_some()
            || self.search_query.is_some();

        if !filters_active {
            return true;
        }

        let mut matches = Vec::new();

        // Assignee filter
        if !self.assignee_ids.is_empty() {
            let has_assignee = card
                .assignees
                .iter()
                .any(|a| self.assignee_ids.contains(&a.id));
            matches.push(has_assignee);
        }

        // Label filter
        if !self.label_ids.is_empty() {
            let has_label = card.labels.iter().any(|l| self.label_ids.contains(&l.id));
            matches.push(has_label);
        }

        // Priority filter
        if !self.priorities.is_empty() {
            matches.push(self.priorities.contains(&card.priority));
        }

        // Due date filter
        if let Some((start, end)) = self.due_date_range {
            if let Some(due) = card.due_date {
                let after_start = start.is_none_or(|s| due >= s);
                let before_end = end.is_none_or(|e| due <= e);
                matches.push(after_start && before_end);
            } else {
                matches.push(false);
            }
        }

        // Search query filter
        if let Some(ref query) = self.search_query {
            let query_lower = query.to_lowercase();
            let matches_title = card.title.to_lowercase().contains(&query_lower);
            let matches_desc = card
                .description
                .as_ref()
                .is_some_and(|d| d.to_lowercase().contains(&query_lower));
            let matches_label = card
                .labels
                .iter()
                .any(|l| l.name.to_lowercase().contains(&query_lower));
            let matches_assignee = card
                .assignees
                .iter()
                .any(|a| a.name.to_lowercase().contains(&query_lower));

            matches.push(matches_title || matches_desc || matches_label || matches_assignee);
        }

        // Apply filter logic
        match self.filter_logic {
            FilterLogic::And => matches.iter().all(|&m| m),
            FilterLogic::Or => matches.iter().any(|&m| m),
        }
    }
}

/// Event data for card movement
#[derive(Clone, Debug, PartialEq)]
pub struct CardMoveEvent {
    /// ID of the card being moved
    pub card_id: String,
    /// ID of the source column
    pub from_column_id: String,
    /// ID of the destination column
    pub to_column_id: String,
    /// Original position in the source column
    pub from_index: usize,
    /// New position in the destination column
    pub to_index: usize,
}

/// Configuration for the kanban board
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KanbanConfig {
    /// Maximum height for the board
    pub max_height: Option<String>,
    /// Enable drag-and-drop functionality
    pub enable_drag_drop: bool,
    /// Enable filtering functionality
    pub enable_filters: bool,
    /// Enable search functionality
    pub enable_search: bool,
    /// Search debounce delay in milliseconds
    pub search_debounce_ms: u64,
    /// Persist state to localStorage
    pub persist_state: bool,
    /// localStorage key for persisted state
    pub storage_key: String,
}

impl Default for KanbanConfig {
    fn default() -> Self {
        Self {
            max_height: Some("600px".to_string()),
            enable_drag_drop: true,
            enable_filters: true,
            enable_search: true,
            search_debounce_ms: 300,
            persist_state: true,
            storage_key: "kanban-board-state".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_defaults() {
        assert_eq!(Priority::default(), Priority::Medium);
        assert_eq!(Priority::Low.as_str(), "low");
        assert_eq!(Priority::High.color_class(), "badge-warning");
    }

    #[test]
    fn test_label_creation() {
        let label = Label::new("label-1", "Bug").with_color("#ff0000");
        assert_eq!(label.id, "label-1");
        assert_eq!(label.name, "Bug");
        assert_eq!(label.color, Some("#ff0000".to_string()));
    }

    #[test]
    fn test_assignee_initials() {
        let assignee = Assignee::new("user-1", "John Doe");
        assert_eq!(assignee.initials, "JD");

        let single_name = Assignee::new("user-2", "Alice");
        assert_eq!(single_name.initials, "A");
    }

    #[test]
    fn test_kanban_card_builder() {
        let card = KanbanCard::new("card-1", "Fix bug")
            .with_description("Critical bug in authentication")
            .with_priority(Priority::Critical)
            .with_label(Label::new("bug", "Bug"))
            .with_assignee(Assignee::new("user-1", "John Doe"));

        assert_eq!(card.card_id, "card-1");
        assert_eq!(card.title, "Fix bug");
        assert_eq!(card.priority, Priority::Critical);
        assert_eq!(card.labels.len(), 1);
        assert_eq!(card.assignees.len(), 1);
    }

    #[test]
    fn test_card_overdue() {
        let yesterday = chrono::Local::now().date_naive() - chrono::Duration::days(1);
        let tomorrow = chrono::Local::now().date_naive() + chrono::Duration::days(1);

        let overdue_card = KanbanCard::new("card-1", "Overdue").with_due_date(yesterday);
        assert!(overdue_card.is_overdue());

        let future_card = KanbanCard::new("card-2", "Future").with_due_date(tomorrow);
        assert!(!future_card.is_overdue());
    }

    #[test]
    fn test_column_wip_limit() {
        let mut column = KanbanColumn::new("col-1", "In Progress").with_card_limit(3);
        assert!(!column.is_over_limit());

        for i in 0..4 {
            column = column.with_card(KanbanCard::new(format!("card-{}", i), "Task"));
        }

        assert!(column.is_over_limit());
        assert_eq!(column.card_count(), 4);
    }

    #[test]
    fn test_filter_by_assignee() {
        let card = KanbanCard::new("card-1", "Task").with_assignee(Assignee::new("user-1", "John"));

        let mut filters = KanbanFilters::new();
        filters.assignee_ids.push("user-1".to_string());

        assert!(filters.matches_card(&card));

        filters.assignee_ids.clear();
        filters.assignee_ids.push("user-2".to_string());
        assert!(!filters.matches_card(&card));
    }

    #[test]
    fn test_filter_by_priority() {
        let card = KanbanCard::new("card-1", "Task").with_priority(Priority::High);

        let mut filters = KanbanFilters::new();
        filters.priorities.push(Priority::High);
        assert!(filters.matches_card(&card));

        filters.priorities.clear();
        filters.priorities.push(Priority::Low);
        assert!(!filters.matches_card(&card));
    }

    #[test]
    fn test_search_filter() {
        let card = KanbanCard::new("card-1", "Fix authentication bug")
            .with_description("Critical security issue");

        let mut filters = KanbanFilters::new();
        filters.search_query = Some("authentication".to_string());
        assert!(filters.matches_card(&card));

        filters.search_query = Some("security".to_string());
        assert!(filters.matches_card(&card));

        filters.search_query = Some("nonexistent".to_string());
        assert!(!filters.matches_card(&card));
    }

    #[test]
    fn test_filter_logic_and() {
        let card = KanbanCard::new("card-1", "Task")
            .with_priority(Priority::High)
            .with_assignee(Assignee::new("user-1", "John"));

        let mut filters = KanbanFilters::new();
        filters.filter_logic = FilterLogic::And;
        filters.priorities.push(Priority::High);
        filters.assignee_ids.push("user-1".to_string());

        assert!(filters.matches_card(&card));

        filters.assignee_ids.clear();
        filters.assignee_ids.push("user-2".to_string());
        assert!(!filters.matches_card(&card)); // AND fails if any filter fails
    }

    #[test]
    fn test_filter_logic_or() {
        let card = KanbanCard::new("card-1", "Task")
            .with_priority(Priority::High)
            .with_assignee(Assignee::new("user-1", "John"));

        let mut filters = KanbanFilters::new();
        filters.filter_logic = FilterLogic::Or;
        filters.priorities.push(Priority::High);
        filters.assignee_ids.push("user-2".to_string());

        assert!(filters.matches_card(&card)); // OR succeeds if any filter matches
    }
}
