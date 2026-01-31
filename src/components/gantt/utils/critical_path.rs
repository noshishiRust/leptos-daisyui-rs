/// Critical path analysis for project scheduling
///
/// Implements the Critical Path Method (CPM) algorithm to identify the longest
/// sequence of dependent tasks that determines the minimum project duration.
use chrono::{DateTime, Utc};
use std::collections::{HashMap, HashSet};

use super::dependency_graph::DependencyGraph;
use crate::components::gantt::{DependencyType, GanttTask, TaskDependency};

/// Result of critical path calculation for a single task
#[derive(Clone, Debug, PartialEq)]
pub struct TaskScheduleInfo {
    /// Task ID
    pub task_id: String,

    /// Early start time (earliest the task can begin)
    pub early_start: DateTime<Utc>,

    /// Early finish time (earliest the task can complete)
    pub early_finish: DateTime<Utc>,

    /// Late start time (latest the task can begin without delaying project)
    pub late_start: DateTime<Utc>,

    /// Late finish time (latest the task can complete without delaying project)
    pub late_finish: DateTime<Utc>,

    /// Total slack/float (how much the task can be delayed)
    pub slack_days: i64,

    /// Free slack (delay without affecting successors)
    pub free_slack_days: i64,

    /// Whether this task is on the critical path
    pub is_critical: bool,
}

impl TaskScheduleInfo {
    /// Calculate duration of the task in days
    pub fn duration_days(&self) -> i64 {
        self.early_finish
            .signed_duration_since(self.early_start)
            .num_days()
    }
}

/// Critical path analysis result
#[derive(Clone, Debug)]
pub struct CriticalPathAnalysis {
    /// Schedule information for all tasks
    pub task_schedules: HashMap<String, TaskScheduleInfo>,

    /// List of task IDs on the critical path (in order)
    pub critical_path: Vec<String>,

    /// Project early finish date
    pub project_early_finish: DateTime<Utc>,

    /// Project late finish date
    pub project_late_finish: DateTime<Utc>,

    /// Total project slack (difference between early and late finish)
    pub project_slack_days: i64,
}

impl CriticalPathAnalysis {
    /// Get all critical tasks
    pub fn get_critical_tasks(&self) -> Vec<&TaskScheduleInfo> {
        self.critical_path
            .iter()
            .filter_map(|id| self.task_schedules.get(id))
            .collect()
    }

    /// Check if a task is critical
    pub fn is_task_critical(&self, task_id: &str) -> bool {
        self.critical_path.contains(&task_id.to_string())
    }

    /// Get schedule info for a task
    pub fn get_task_schedule(&self, task_id: &str) -> Option<&TaskScheduleInfo> {
        self.task_schedules.get(task_id)
    }
}

/// Calculate critical path for a set of tasks and dependencies
///
/// Uses the Critical Path Method (CPM) algorithm:
/// 1. Forward pass: Calculate early start/finish for all tasks
/// 2. Backward pass: Calculate late start/finish for all tasks
/// 3. Calculate slack for each task
/// 4. Identify critical path (tasks with zero slack)
pub fn calculate_critical_path(
    tasks: &[GanttTask],
    dependencies: &[TaskDependency],
) -> Result<CriticalPathAnalysis, String> {
    if tasks.is_empty() {
        return Err("Cannot calculate critical path for empty task list".to_string());
    }

    let graph = DependencyGraph::new(tasks, dependencies);

    // Check for cycles
    if graph.has_cycles() {
        return Err("Cannot calculate critical path: circular dependencies detected".to_string());
    }

    // Get topological order for forward pass
    let topo_order = graph
        .topological_sort()
        .map_err(|e| format!("Topological sort failed: {}", e))?;

    // Create task lookup map
    let task_map: HashMap<String, &GanttTask> = tasks.iter().map(|t| (t.id.clone(), t)).collect();

    // Forward pass: Calculate early start and early finish
    let mut early_start: HashMap<String, DateTime<Utc>> = HashMap::new();
    let mut early_finish: HashMap<String, DateTime<Utc>> = HashMap::new();

    for task_id in &topo_order {
        let task = task_map
            .get(task_id)
            .ok_or_else(|| format!("Task {} not found", task_id))?;

        // Find maximum early finish of all predecessors
        let predecessors = graph.get_dependencies(task_id).unwrap_or_default();

        let es = if predecessors.is_empty() {
            // No predecessors: start at task's scheduled start
            task.start
        } else {
            // Start after all predecessors finish (considering dependency types and lag)
            predecessors
                .iter()
                .filter_map(|pred_id| {
                    let pred_ef = early_finish.get(pred_id)?;
                    let dep = dependencies
                        .iter()
                        .find(|d| &d.source_id == pred_id && &d.target_id == task_id)?;

                    // Apply lag time
                    let lag = chrono::Duration::days(dep.lag_days as i64);

                    // Calculate start based on dependency type
                    match dep.dependency_type {
                        DependencyType::FS => Some(*pred_ef + lag), // Finish-to-Start
                        DependencyType::SS => {
                            // Start-to-Start: can start when predecessor starts
                            early_start.get(pred_id).map(|&es| es + lag)
                        }
                        DependencyType::FF | DependencyType::SF => {
                            // These types are less common, use conservative estimate
                            Some(*pred_ef + lag)
                        }
                    }
                })
                .max()
                .unwrap_or(task.start)
        };

        let ef = es + task.end.signed_duration_since(task.start);

        early_start.insert(task_id.clone(), es);
        early_finish.insert(task_id.clone(), ef);
    }

    // Find project early finish (maximum early finish across all tasks)
    let project_early_finish = early_finish
        .values()
        .max()
        .copied()
        .ok_or_else(|| "No tasks found".to_string())?;

    // Backward pass: Calculate late start and late finish
    let mut late_start: HashMap<String, DateTime<Utc>> = HashMap::new();
    let mut late_finish: HashMap<String, DateTime<Utc>> = HashMap::new();

    // Initialize tasks without successors
    for task_id in tasks.iter().map(|t| &t.id) {
        let successors = graph.get_dependents(task_id).unwrap_or_default();

        if successors.is_empty() {
            // No successors: late finish = project early finish
            late_finish.insert(task_id.clone(), project_early_finish);

            let task = task_map.get(task_id).unwrap();
            let duration = task.end.signed_duration_since(task.start);
            late_start.insert(task_id.clone(), project_early_finish - duration);
        }
    }

    // Process in reverse topological order
    for task_id in topo_order.iter().rev() {
        if late_finish.contains_key(task_id) {
            continue; // Already processed
        }

        let task = task_map.get(task_id).unwrap();
        let duration = task.end.signed_duration_since(task.start);

        // Find minimum late start of all successors
        let successors = graph.get_dependents(task_id).unwrap_or_default();

        let lf = successors
            .iter()
            .filter_map(|succ_id| {
                let succ_ls = late_start.get(succ_id)?;
                let dep = dependencies
                    .iter()
                    .find(|d| &d.source_id == task_id && &d.target_id == succ_id)?;

                let lag = chrono::Duration::days(dep.lag_days as i64);

                match dep.dependency_type {
                    DependencyType::FS => Some(*succ_ls - lag),
                    DependencyType::SS => {
                        // Conservative estimate
                        Some(*succ_ls - lag)
                    }
                    DependencyType::FF | DependencyType::SF => {
                        late_finish.get(succ_id).map(|&lf| lf - lag)
                    }
                }
            })
            .min()
            .unwrap_or(project_early_finish);

        let ls = lf - duration;

        late_finish.insert(task_id.clone(), lf);
        late_start.insert(task_id.clone(), ls);
    }

    // Calculate slack and identify critical tasks
    let mut task_schedules = HashMap::new();
    let mut critical_tasks = Vec::new();

    for task_id in tasks.iter().map(|t| &t.id) {
        let es = early_start
            .get(task_id)
            .copied()
            .unwrap_or_else(|| task_map.get(task_id).unwrap().start);
        let ef = early_finish
            .get(task_id)
            .copied()
            .unwrap_or_else(|| task_map.get(task_id).unwrap().end);
        let ls = late_start.get(task_id).copied().unwrap_or(es);
        let lf = late_finish.get(task_id).copied().unwrap_or(ef);

        let slack_days = ls.signed_duration_since(es).num_days();

        // Free slack: delay without affecting immediate successors
        let successors = graph.get_dependents(task_id).unwrap_or_default();
        let free_slack_days = if successors.is_empty() {
            slack_days
        } else {
            successors
                .iter()
                .filter_map(|succ_id| early_start.get(succ_id))
                .map(|&succ_es| succ_es.signed_duration_since(ef).num_days())
                .min()
                .unwrap_or(0)
        };

        let is_critical = slack_days == 0;

        if is_critical {
            critical_tasks.push(task_id.clone());
        }

        task_schedules.insert(
            task_id.clone(),
            TaskScheduleInfo {
                task_id: task_id.clone(),
                early_start: es,
                early_finish: ef,
                late_start: ls,
                late_finish: lf,
                slack_days,
                free_slack_days,
                is_critical,
            },
        );
    }

    // Build critical path by following critical tasks through dependencies
    let critical_path = build_critical_path_sequence(&critical_tasks, &graph);

    Ok(CriticalPathAnalysis {
        task_schedules,
        critical_path,
        project_early_finish,
        project_late_finish: project_early_finish, // Same for simple CPM
        project_slack_days: 0,
    })
}

/// Build ordered sequence of critical path tasks
fn build_critical_path_sequence(critical_tasks: &[String], graph: &DependencyGraph) -> Vec<String> {
    if critical_tasks.is_empty() {
        return Vec::new();
    }

    // Find start tasks (critical tasks with no critical predecessors)
    let critical_set: HashSet<_> = critical_tasks.iter().collect();

    let mut start_tasks: Vec<String> = critical_tasks
        .iter()
        .filter(|&task_id| {
            let predecessors = graph.get_dependencies(task_id).unwrap_or_default();
            !predecessors.iter().any(|pred| critical_set.contains(pred))
        })
        .cloned()
        .collect();

    if start_tasks.is_empty() && !critical_tasks.is_empty() {
        // Fallback: use first critical task
        start_tasks.push(critical_tasks[0].clone());
    }

    // Build path using topological sort of critical tasks
    let mut visited = HashSet::new();
    let mut path = Vec::new();

    for start in &start_tasks {
        build_path_dfs(start, &critical_set, graph, &mut visited, &mut path);
    }

    path
}

/// DFS to build critical path sequence
fn build_path_dfs(
    task_id: &str,
    critical_set: &HashSet<&String>,
    graph: &DependencyGraph,
    visited: &mut HashSet<String>,
    path: &mut Vec<String>,
) {
    if visited.contains(task_id) {
        return;
    }

    visited.insert(task_id.to_string());
    path.push(task_id.to_string());

    // Visit critical successors
    if let Ok(successors) = graph.get_dependents(task_id) {
        for succ in successors {
            if critical_set.contains(&succ) {
                build_path_dfs(&succ, critical_set, graph, visited, path);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::gantt::TaskType;
    use chrono::TimeZone;
    use std::collections::HashMap;

    fn create_test_task(id: &str, start_day: i64, duration_days: i64) -> GanttTask {
        let start = Utc
            .with_ymd_and_hms(2024, 1, start_day as u32, 0, 0, 0)
            .unwrap();
        let end = start + chrono::Duration::days(duration_days);

        GanttTask {
            id: id.to_string(),
            name: format!("Task {}", id),
            start,
            end,
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

    fn create_dependency(source: &str, target: &str) -> TaskDependency {
        TaskDependency {
            source_id: source.to_string(),
            target_id: target.to_string(),
            dependency_type: DependencyType::FS,
            lag_days: 0,
        }
    }

    #[test]
    fn test_simple_critical_path() {
        // Simple linear chain: A(3) -> B(2) -> C(1)
        let tasks = vec![
            create_test_task("A", 1, 3),
            create_test_task("B", 4, 2),
            create_test_task("C", 6, 1),
        ];

        let dependencies = vec![create_dependency("A", "B"), create_dependency("B", "C")];

        let result = calculate_critical_path(&tasks, &dependencies).unwrap();

        // All tasks should be critical in a linear chain
        assert_eq!(result.critical_path.len(), 3);
        assert!(result.is_task_critical("A"));
        assert!(result.is_task_critical("B"));
        assert!(result.is_task_critical("C"));

        // Check slack
        for task_id in &["A", "B", "C"] {
            let schedule = result.get_task_schedule(task_id).unwrap();
            assert_eq!(
                schedule.slack_days, 0,
                "Task {} should have 0 slack",
                task_id
            );
        }
    }

    #[test]
    fn test_parallel_tasks_with_slack() {
        // A(5) -> C(2)
        // B(2) -> C(2)
        // A is critical, B has slack
        let tasks = vec![
            create_test_task("A", 1, 5),
            create_test_task("B", 1, 2),
            create_test_task("C", 6, 2),
        ];

        let dependencies = vec![create_dependency("A", "C"), create_dependency("B", "C")];

        let result = calculate_critical_path(&tasks, &dependencies).unwrap();

        // A should be critical
        assert!(result.is_task_critical("A"));
        assert!(result.is_task_critical("C"));

        // B should have slack
        let schedule_b = result.get_task_schedule("B").unwrap();
        assert!(
            schedule_b.slack_days > 0,
            "Task B should have positive slack"
        );
        assert!(!schedule_b.is_critical);
    }

    #[test]
    fn test_empty_task_list() {
        let tasks = vec![];
        let dependencies = vec![];

        let result = calculate_critical_path(&tasks, &dependencies);
        assert!(result.is_err());
    }

    #[test]
    fn test_circular_dependencies() {
        let tasks = vec![
            create_test_task("A", 1, 3),
            create_test_task("B", 4, 2),
            create_test_task("C", 6, 1),
        ];

        let dependencies = vec![
            create_dependency("A", "B"),
            create_dependency("B", "C"),
            create_dependency("C", "A"), // Creates cycle
        ];

        let result = calculate_critical_path(&tasks, &dependencies);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("circular"));
    }

    #[test]
    fn test_task_with_lag() {
        let tasks = vec![create_test_task("A", 1, 3), create_test_task("B", 4, 2)];

        let dependencies = vec![TaskDependency {
            source_id: "A".to_string(),
            target_id: "B".to_string(),
            dependency_type: DependencyType::FS,
            lag_days: 2, // 2-day lag
        }];

        let result = calculate_critical_path(&tasks, &dependencies).unwrap();

        // Both should be critical
        assert!(result.is_task_critical("A"));
        assert!(result.is_task_critical("B"));
    }

    #[test]
    fn test_schedule_info_duration() {
        let tasks = vec![create_test_task("A", 1, 5)];
        let dependencies = vec![];

        let result = calculate_critical_path(&tasks, &dependencies).unwrap();
        let schedule = result.get_task_schedule("A").unwrap();

        assert_eq!(schedule.duration_days(), 5);
    }
}
