use super::types::*;
/// Unit tests for Kanban board component
///
/// Tests cover:
/// - Priority enum methods
/// - Label and Assignee builders
/// - KanbanCard builder methods and filter matching
/// - KanbanColumn WIP limits and card counting
/// - Filter logic (AND/OR) with search, priority, assignee, and label filters
use chrono::NaiveDate;

// Helper functions

fn create_test_card(id: &str, title: &str) -> KanbanCard {
    KanbanCard::new(id, title)
}

fn create_test_assignee(id: &str, name: &str) -> Assignee {
    Assignee::new(id, name)
}

fn create_test_label(id: &str, name: &str) -> Label {
    Label::new(id, name)
}

// Priority Tests

#[test]
fn test_priority_as_str() {
    assert_eq!(Priority::Low.as_str(), "low");
    assert_eq!(Priority::Medium.as_str(), "medium");
    assert_eq!(Priority::High.as_str(), "high");
    assert_eq!(Priority::Critical.as_str(), "critical");
}

#[test]
fn test_priority_color_class() {
    assert_eq!(Priority::Low.color_class(), "badge-info");
    assert_eq!(Priority::Medium.color_class(), "badge-primary");
    assert_eq!(Priority::High.color_class(), "badge-warning");
    assert_eq!(Priority::Critical.color_class(), "badge-error");
}

#[test]
fn test_priority_default() {
    assert_eq!(Priority::default(), Priority::Medium);
}

// Label Tests

#[test]
fn test_label_new() {
    let label = create_test_label("bug", "Bug");
    assert_eq!(label.id, "bug");
    assert_eq!(label.name, "Bug");
    assert_eq!(label.color, None);
}

#[test]
fn test_label_with_color() {
    let label = create_test_label("bug", "Bug").with_color("#ff0000");
    assert_eq!(label.color, Some("#ff0000".to_string()));
}

// Assignee Tests

#[test]
fn test_assignee_initials_two_words() {
    let assignee = create_test_assignee("1", "John Doe");
    assert_eq!(assignee.initials, "JD");
}

#[test]
fn test_assignee_initials_one_word() {
    let assignee = create_test_assignee("1", "Alice");
    assert_eq!(assignee.initials, "A");
}

#[test]
fn test_assignee_initials_three_words() {
    let assignee = create_test_assignee("1", "Mary Jane Watson");
    assert_eq!(assignee.initials, "MJ"); // Takes only first two
}

#[test]
fn test_assignee_with_avatar() {
    let assignee =
        create_test_assignee("1", "John Doe").with_avatar("https://example.com/avatar.jpg");
    assert_eq!(
        assignee.avatar_url,
        Some("https://example.com/avatar.jpg".to_string())
    );
}

// KanbanCard Tests

#[test]
fn test_card_new() {
    let card = create_test_card("1", "Test Task");
    assert_eq!(card.card_id, "1");
    assert_eq!(card.title, "Test Task");
    assert_eq!(card.description, None);
    assert_eq!(card.priority, Priority::Medium);
    assert!(card.labels.is_empty());
    assert!(card.assignees.is_empty());
}

#[test]
fn test_card_with_description() {
    let card = create_test_card("1", "Task").with_description("Do something");
    assert_eq!(card.description, Some("Do something".to_string()));
}

#[test]
fn test_card_with_priority() {
    let card = create_test_card("1", "Task").with_priority(Priority::High);
    assert_eq!(card.priority, Priority::High);
}

#[test]
fn test_card_with_label() {
    let card = create_test_card("1", "Task").with_label(create_test_label("bug", "Bug"));
    assert_eq!(card.labels.len(), 1);
    assert_eq!(card.labels[0].id, "bug");
}

#[test]
fn test_card_with_assignee() {
    let card = create_test_card("1", "Task").with_assignee(create_test_assignee("u1", "Alice"));
    assert_eq!(card.assignees.len(), 1);
    assert_eq!(card.assignees[0].id, "u1");
}

#[test]
fn test_card_with_due_date() {
    let due = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
    let card = create_test_card("1", "Task").with_due_date(due);
    assert_eq!(card.due_date, Some(due));
}

#[test]
fn test_card_is_overdue_no_due_date() {
    let card = create_test_card("1", "Task");
    assert!(!card.is_overdue());
}

#[test]
fn test_card_is_overdue_past_date() {
    let past_date = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();
    let card = create_test_card("1", "Task").with_due_date(past_date);
    assert!(card.is_overdue());
}

#[test]
fn test_card_is_overdue_future_date() {
    let future_date = NaiveDate::from_ymd_opt(2099, 12, 31).unwrap();
    let card = create_test_card("1", "Task").with_due_date(future_date);
    assert!(!card.is_overdue());
}

// KanbanColumn Tests

#[test]
fn test_column_new() {
    let column = KanbanColumn::new("todo", "To Do");
    assert_eq!(column.column_id, "todo");
    assert_eq!(column.title, "To Do");
    assert_eq!(column.color, None);
    assert_eq!(column.card_limit, None);
    assert!(column.cards.is_empty());
    assert!(!column.collapsed);
    assert!(column.collapsible);
    assert!(column.scrollable);
}

#[test]
fn test_column_with_color() {
    let column = KanbanColumn::new("todo", "To Do").with_color("#ff0000");
    assert_eq!(column.color, Some("#ff0000".to_string()));
}

#[test]
fn test_column_with_card_limit() {
    let column = KanbanColumn::new("todo", "To Do").with_card_limit(5);
    assert_eq!(column.card_limit, Some(5));
}

#[test]
fn test_column_with_card() {
    let column = KanbanColumn::new("todo", "To Do").with_card(create_test_card("1", "Task 1"));
    assert_eq!(column.cards.len(), 1);
}

#[test]
fn test_column_with_cards() {
    let cards = vec![
        create_test_card("1", "Task 1"),
        create_test_card("2", "Task 2"),
    ];
    let column = KanbanColumn::new("todo", "To Do").with_cards(cards);
    assert_eq!(column.cards.len(), 2);
}

#[test]
fn test_column_card_count() {
    let column = KanbanColumn::new("todo", "To Do")
        .with_card(create_test_card("1", "Task 1"))
        .with_card(create_test_card("2", "Task 2"));
    assert_eq!(column.card_count(), 2);
}

#[test]
fn test_column_is_over_limit_no_limit() {
    let column = KanbanColumn::new("todo", "To Do").with_card(create_test_card("1", "Task 1"));
    assert!(!column.is_over_limit());
}

#[test]
fn test_column_is_over_limit_under() {
    let column = KanbanColumn::new("todo", "To Do")
        .with_card_limit(5)
        .with_card(create_test_card("1", "Task 1"));
    assert!(!column.is_over_limit());
}

#[test]
fn test_column_is_over_limit_at_limit() {
    let column = KanbanColumn::new("todo", "To Do")
        .with_card_limit(2)
        .with_card(create_test_card("1", "Task 1"))
        .with_card(create_test_card("2", "Task 2"));
    assert!(!column.is_over_limit());
}

#[test]
fn test_column_is_over_limit_exceeded() {
    let column = KanbanColumn::new("todo", "To Do")
        .with_card_limit(2)
        .with_card(create_test_card("1", "Task 1"))
        .with_card(create_test_card("2", "Task 2"))
        .with_card(create_test_card("3", "Task 3"));
    assert!(column.is_over_limit());
}

// Filter Tests

#[test]
fn test_filters_new() {
    let filters = KanbanFilters::new();
    assert!(filters.assignee_ids.is_empty());
    assert!(filters.label_ids.is_empty());
    assert!(filters.priorities.is_empty());
    assert_eq!(filters.due_date_range, None);
    assert_eq!(filters.search_query, None);
    assert_eq!(filters.filter_logic, FilterLogic::And);
}

#[test]
fn test_filter_logic_default() {
    assert_eq!(FilterLogic::default(), FilterLogic::And);
}

#[test]
fn test_card_matches_filters_no_filters() {
    let card = create_test_card("1", "Task");
    let filters = KanbanFilters::new();
    assert!(card.matches_filters(&filters));
}

#[test]
fn test_card_matches_filters_search_title() {
    let card = create_test_card("1", "Fix login bug");
    let mut filters = KanbanFilters::new();
    filters.search_query = Some("login".to_string());
    assert!(card.matches_filters(&filters));
}

#[test]
fn test_card_matches_filters_search_case_insensitive() {
    let card = create_test_card("1", "Fix Login Bug");
    let mut filters = KanbanFilters::new();
    filters.search_query = Some("login".to_string());
    assert!(card.matches_filters(&filters));
}

#[test]
fn test_card_matches_filters_search_description() {
    let card = create_test_card("1", "Task").with_description("Fix the authentication problem");
    let mut filters = KanbanFilters::new();
    filters.search_query = Some("authentication".to_string());
    assert!(card.matches_filters(&filters));
}

#[test]
fn test_card_matches_filters_search_no_match() {
    let card = create_test_card("1", "Fix bug");
    let mut filters = KanbanFilters::new();
    filters.search_query = Some("feature".to_string());
    assert!(!card.matches_filters(&filters));
}

#[test]
fn test_card_matches_filters_priority_match() {
    let card = create_test_card("1", "Task").with_priority(Priority::High);
    let mut filters = KanbanFilters::new();
    filters.priorities = vec![Priority::High, Priority::Critical];
    assert!(card.matches_filters(&filters));
}

#[test]
fn test_card_matches_filters_priority_no_match() {
    let card = create_test_card("1", "Task").with_priority(Priority::Low);
    let mut filters = KanbanFilters::new();
    filters.priorities = vec![Priority::High, Priority::Critical];
    assert!(!card.matches_filters(&filters));
}

#[test]
fn test_card_matches_filters_assignee_match() {
    let card = create_test_card("1", "Task").with_assignee(create_test_assignee("u1", "Alice"));
    let mut filters = KanbanFilters::new();
    filters.assignee_ids = vec!["u1".to_string()];
    assert!(card.matches_filters(&filters));
}

#[test]
fn test_card_matches_filters_assignee_no_match() {
    let card = create_test_card("1", "Task").with_assignee(create_test_assignee("u1", "Alice"));
    let mut filters = KanbanFilters::new();
    filters.assignee_ids = vec!["u2".to_string()];
    assert!(!card.matches_filters(&filters));
}

#[test]
fn test_card_matches_filters_label_match() {
    let card = create_test_card("1", "Task").with_label(create_test_label("bug", "Bug"));
    let mut filters = KanbanFilters::new();
    filters.label_ids = vec!["bug".to_string()];
    assert!(card.matches_filters(&filters));
}

#[test]
fn test_card_matches_filters_label_no_match() {
    let card = create_test_card("1", "Task").with_label(create_test_label("bug", "Bug"));
    let mut filters = KanbanFilters::new();
    filters.label_ids = vec!["feature".to_string()];
    assert!(!card.matches_filters(&filters));
}

#[test]
fn test_card_matches_filters_and_logic_all_match() {
    let card = create_test_card("1", "Fix login bug")
        .with_priority(Priority::High)
        .with_assignee(create_test_assignee("u1", "Alice"));

    let mut filters = KanbanFilters::new();
    filters.filter_logic = FilterLogic::And;
    filters.search_query = Some("login".to_string());
    filters.priorities = vec![Priority::High];
    filters.assignee_ids = vec!["u1".to_string()];

    assert!(card.matches_filters(&filters));
}

#[test]
fn test_card_matches_filters_and_logic_partial_match() {
    let card = create_test_card("1", "Fix login bug")
        .with_priority(Priority::Low) // Doesn't match priority filter
        .with_assignee(create_test_assignee("u1", "Alice"));

    let mut filters = KanbanFilters::new();
    filters.filter_logic = FilterLogic::And;
    filters.search_query = Some("login".to_string());
    filters.priorities = vec![Priority::High];
    filters.assignee_ids = vec!["u1".to_string()];

    assert!(!card.matches_filters(&filters)); // Should fail because not all match
}

#[test]
fn test_card_matches_filters_or_logic_partial_match() {
    let card = create_test_card("1", "Fix bug")
        .with_priority(Priority::High)
        .with_assignee(create_test_assignee("u2", "Bob"));

    let mut filters = KanbanFilters::new();
    filters.filter_logic = FilterLogic::Or;
    filters.search_query = Some("login".to_string()); // Doesn't match
    filters.priorities = vec![Priority::High]; // Matches!
    filters.assignee_ids = vec!["u1".to_string()]; // Doesn't match

    assert!(card.matches_filters(&filters)); // Should pass because one matches
}

#[test]
fn test_card_matches_filters_or_logic_no_match() {
    let card = create_test_card("1", "Fix bug")
        .with_priority(Priority::Low)
        .with_assignee(create_test_assignee("u3", "Charlie"));

    let mut filters = KanbanFilters::new();
    filters.filter_logic = FilterLogic::Or;
    filters.search_query = Some("login".to_string());
    filters.priorities = vec![Priority::High];
    filters.assignee_ids = vec!["u1".to_string()];

    assert!(!card.matches_filters(&filters));
}

#[test]
fn test_kanban_filters_matches_card_integration() {
    // Test using the KanbanFilters.matches_card method
    let card = create_test_card("1", "Implement feature")
        .with_priority(Priority::Medium)
        .with_assignee(create_test_assignee("u1", "Alice"))
        .with_label(create_test_label("feature", "Feature"));

    let mut filters = KanbanFilters::new();
    filters.assignee_ids = vec!["u1".to_string()];
    filters.label_ids = vec!["feature".to_string()];

    assert!(filters.matches_card(&card));
}

#[test]
fn test_kanban_filters_matches_card_search() {
    let card = create_test_card("1", "Fix authentication issue");

    let mut filters = KanbanFilters::new();
    filters.search_query = Some("authentication".to_string());

    assert!(filters.matches_card(&card));
}

#[test]
fn test_kanban_filters_due_date_in_range() {
    let due_date = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();
    let card = create_test_card("1", "Task").with_due_date(due_date);

    let mut filters = KanbanFilters::new();
    filters.due_date_range = Some((
        Some(NaiveDate::from_ymd_opt(2024, 6, 1).unwrap()),
        Some(NaiveDate::from_ymd_opt(2024, 6, 30).unwrap()),
    ));

    assert!(filters.matches_card(&card));
}

#[test]
fn test_kanban_filters_due_date_out_of_range() {
    let due_date = NaiveDate::from_ymd_opt(2024, 7, 15).unwrap();
    let card = create_test_card("1", "Task").with_due_date(due_date);

    let mut filters = KanbanFilters::new();
    filters.due_date_range = Some((
        Some(NaiveDate::from_ymd_opt(2024, 6, 1).unwrap()),
        Some(NaiveDate::from_ymd_opt(2024, 6, 30).unwrap()),
    ));

    assert!(!filters.matches_card(&card));
}

#[test]
fn test_kanban_filters_no_due_date_with_range_filter() {
    let card = create_test_card("1", "Task"); // No due date

    let mut filters = KanbanFilters::new();
    filters.due_date_range = Some((
        Some(NaiveDate::from_ymd_opt(2024, 6, 1).unwrap()),
        Some(NaiveDate::from_ymd_opt(2024, 6, 30).unwrap()),
    ));

    assert!(!filters.matches_card(&card));
}
