/// Dependency validation logic for interactive dependency creation
///
/// Uses the DependencyGraph to validate new dependencies before adding them
/// to prevent circular dependencies and other invalid states.
use crate::components::gantt::{DependencyType, GanttTask, TaskDependency};
use crate::components::gantt::utils::{DependencyGraph, DependencyGraphError};

/// Result of validating a potential new dependency
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ValidationResult {
    /// Dependency is valid and can be created
    Valid,

    /// Dependency would create a circular dependency
    CircularDependency(Vec<String>),

    /// Source or target task not found
    TaskNotFound(String),

    /// Task cannot depend on itself
    SelfDependency,

    /// Dependency already exists
    DuplicateDependency,
}

impl std::fmt::Display for ValidationResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationResult::Valid => write!(f, "Valid"),
            ValidationResult::CircularDependency(tasks) => {
                write!(f, "Would create circular dependency: {}", tasks.join(" -> "))
            }
            ValidationResult::TaskNotFound(id) => write!(f, "Task not found: {}", id),
            ValidationResult::SelfDependency => write!(f, "Task cannot depend on itself"),
            ValidationResult::DuplicateDependency => write!(f, "Dependency already exists"),
        }
    }
}

/// Validate a potential new dependency
///
/// # Arguments
/// * `tasks` - List of all tasks
/// * `dependencies` - List of existing dependencies
/// * `source_id` - ID of the source task
/// * `target_id` - ID of the target task
/// * `dep_type` - Type of dependency to create
///
/// # Returns
/// ValidationResult indicating whether the dependency is valid
pub fn validate_dependency(
    tasks: &[GanttTask],
    dependencies: &[TaskDependency],
    source_id: &str,
    target_id: &str,
    dep_type: DependencyType,
) -> ValidationResult {
    // Check for self-dependency
    if source_id == target_id {
        return ValidationResult::SelfDependency;
    }

    // Check if tasks exist
    if !tasks.iter().any(|t| t.id == source_id) {
        return ValidationResult::TaskNotFound(source_id.to_string());
    }
    if !tasks.iter().any(|t| t.id == target_id) {
        return ValidationResult::TaskNotFound(target_id.to_string());
    }

    // Check for duplicate dependency
    if dependencies.iter().any(|dep| {
        dep.source_id == source_id
            && dep.target_id == target_id
            && dep.dependency_type == dep_type
    }) {
        return ValidationResult::DuplicateDependency;
    }

    // Use DependencyGraph to check for cycles
    let graph = DependencyGraph::new(tasks, dependencies);

    match graph.validate_dependency(source_id, target_id) {
        Ok(_) => ValidationResult::Valid,
        Err(DependencyGraphError::CircularDependency(cycle)) => {
            ValidationResult::CircularDependency(cycle)
        }
        Err(DependencyGraphError::TaskNotFound(id)) => ValidationResult::TaskNotFound(id),
        Err(DependencyGraphError::InvalidDependency(_, _)) => ValidationResult::SelfDependency,
    }
}

/// Create a new task dependency
///
/// This is a convenience function that validates and creates a TaskDependency
/// if validation succeeds.
///
/// # Arguments
/// * `tasks` - List of all tasks
/// * `dependencies` - List of existing dependencies
/// * `source_id` - ID of the source task
/// * `target_id` - ID of the target task
/// * `dep_type` - Type of dependency to create
/// * `lag_days` - Optional lag time in days (default: 0)
///
/// # Returns
/// Result containing the new TaskDependency or a ValidationResult error
pub fn create_dependency(
    tasks: &[GanttTask],
    dependencies: &[TaskDependency],
    source_id: String,
    target_id: String,
    dep_type: DependencyType,
    lag_days: i32,
) -> Result<TaskDependency, ValidationResult> {
    let validation = validate_dependency(tasks, dependencies, &source_id, &target_id, dep_type);

    match validation {
        ValidationResult::Valid => Ok(TaskDependency {
            source_id,
            target_id,
            dependency_type: dep_type,
            lag_days,
        }),
        other => Err(other),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::gantt::TaskType;
    use chrono::Utc;
    use std::collections::HashMap;

    fn create_test_task(id: &str) -> GanttTask {
        GanttTask {
            id: id.to_string(),
            name: format!("Task {}", id),
            start: Utc::now(),
            end: Utc::now(),
            progress: 0.0,
            task_type: TaskType::Task,
            parent_id: None,
            dependencies: Vec::new(),
            assignees: Vec::new(),
            color: None,
            read_only: false,
            metadata: HashMap::new(),
        }
    }

    fn create_test_dependency(source: &str, target: &str) -> TaskDependency {
        TaskDependency {
            source_id: source.to_string(),
            target_id: target.to_string(),
            dependency_type: DependencyType::FS,
            lag_days: 0,
        }
    }

    #[test]
    fn test_validate_valid_dependency() {
        let tasks = vec![create_test_task("task1"), create_test_task("task2")];
        let dependencies = vec![];

        let result = validate_dependency(&tasks, &dependencies, "task1", "task2", DependencyType::FS);
        assert_eq!(result, ValidationResult::Valid);
    }

    #[test]
    fn test_validate_self_dependency() {
        let tasks = vec![create_test_task("task1")];
        let dependencies = vec![];

        let result = validate_dependency(&tasks, &dependencies, "task1", "task1", DependencyType::FS);
        assert_eq!(result, ValidationResult::SelfDependency);
    }

    #[test]
    fn test_validate_task_not_found() {
        let tasks = vec![create_test_task("task1")];
        let dependencies = vec![];

        let result = validate_dependency(
            &tasks,
            &dependencies,
            "task1",
            "nonexistent",
            DependencyType::FS,
        );
        match result {
            ValidationResult::TaskNotFound(id) => assert_eq!(id, "nonexistent"),
            _ => panic!("Expected TaskNotFound"),
        }
    }

    #[test]
    fn test_validate_duplicate_dependency() {
        let tasks = vec![create_test_task("task1"), create_test_task("task2")];
        let dependencies = vec![create_test_dependency("task1", "task2")];

        let result = validate_dependency(&tasks, &dependencies, "task1", "task2", DependencyType::FS);
        assert_eq!(result, ValidationResult::DuplicateDependency);
    }

    #[test]
    fn test_validate_circular_dependency() {
        let tasks = vec![
            create_test_task("task1"),
            create_test_task("task2"),
            create_test_task("task3"),
        ];
        let dependencies = vec![
            create_test_dependency("task1", "task2"),
            create_test_dependency("task2", "task3"),
        ];

        // Trying to add task3 -> task1 would create a cycle
        let result = validate_dependency(&tasks, &dependencies, "task3", "task1", DependencyType::FS);
        match result {
            ValidationResult::CircularDependency(_) => {}
            _ => panic!("Expected CircularDependency, got {:?}", result),
        }
    }

    #[test]
    fn test_create_dependency_success() {
        let tasks = vec![create_test_task("task1"), create_test_task("task2")];
        let dependencies = vec![];

        let result = create_dependency(
            &tasks,
            &dependencies,
            "task1".to_string(),
            "task2".to_string(),
            DependencyType::FS,
            0,
        );

        assert!(result.is_ok());
        let dep = result.unwrap();
        assert_eq!(dep.source_id, "task1");
        assert_eq!(dep.target_id, "task2");
        assert_eq!(dep.dependency_type, DependencyType::FS);
        assert_eq!(dep.lag_days, 0);
    }

    #[test]
    fn test_create_dependency_with_lag() {
        let tasks = vec![create_test_task("task1"), create_test_task("task2")];
        let dependencies = vec![];

        let result = create_dependency(
            &tasks,
            &dependencies,
            "task1".to_string(),
            "task2".to_string(),
            DependencyType::FS,
            5,
        );

        assert!(result.is_ok());
        let dep = result.unwrap();
        assert_eq!(dep.lag_days, 5);
    }

    #[test]
    fn test_create_dependency_failure() {
        let tasks = vec![create_test_task("task1")];
        let dependencies = vec![];

        // Self-dependency should fail
        let result = create_dependency(
            &tasks,
            &dependencies,
            "task1".to_string(),
            "task1".to_string(),
            DependencyType::FS,
            0,
        );

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ValidationResult::SelfDependency);
    }
}
