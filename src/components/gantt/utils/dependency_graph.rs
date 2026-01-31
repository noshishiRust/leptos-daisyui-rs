/// Dependency graph utilities for task dependencies
///
/// Provides cycle detection, topological sorting, and path finding
/// algorithms for managing task dependencies in Gantt charts.
use petgraph::algo::{is_cyclic_directed, toposort};
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;
use std::collections::HashMap;

use crate::components::gantt::{GanttTask, TaskDependency};

/// Result type for dependency graph operations
pub type GraphResult<T> = Result<T, DependencyGraphError>;

/// Errors that can occur during dependency graph operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DependencyGraphError {
    /// Circular dependency detected
    CircularDependency(Vec<String>),

    /// Task not found in graph
    TaskNotFound(String),

    /// Invalid dependency (e.g., task depends on itself)
    InvalidDependency(String, String),
}

impl std::fmt::Display for DependencyGraphError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DependencyGraphError::CircularDependency(tasks) => {
                write!(f, "Circular dependency detected: {}", tasks.join(" -> "))
            }
            DependencyGraphError::TaskNotFound(id) => {
                write!(f, "Task not found: {}", id)
            }
            DependencyGraphError::InvalidDependency(from, to) => {
                write!(f, "Invalid dependency from {} to {}", from, to)
            }
        }
    }
}

impl std::error::Error for DependencyGraphError {}

/// Dependency graph for task scheduling and validation
pub struct DependencyGraph {
    /// Directed graph where nodes are tasks and edges are dependencies
    graph: DiGraph<String, ()>,

    /// Mapping from task ID to graph node index
    task_indices: HashMap<String, NodeIndex>,
}

impl DependencyGraph {
    /// Create a new dependency graph from tasks and dependencies
    pub fn new(tasks: &[GanttTask], dependencies: &[TaskDependency]) -> Self {
        let mut graph = DiGraph::new();
        let mut task_indices = HashMap::new();

        // Add all tasks as nodes
        for task in tasks {
            let node = graph.add_node(task.id.clone());
            task_indices.insert(task.id.clone(), node);
        }

        // Add dependencies as edges
        for dep in dependencies {
            if let (Some(&source_idx), Some(&target_idx)) = (
                task_indices.get(&dep.source_id),
                task_indices.get(&dep.target_id),
            ) {
                graph.add_edge(source_idx, target_idx, ());
            }
        }

        Self {
            graph,
            task_indices,
        }
    }

    /// Check if the dependency graph contains cycles
    pub fn has_cycles(&self) -> bool {
        is_cyclic_directed(&self.graph)
    }

    /// Detect and return all cycles in the graph
    pub fn find_cycles(&self) -> Vec<Vec<String>> {
        let mut cycles = Vec::new();

        // Simple cycle detection using DFS
        let mut visited = HashMap::new();
        let mut rec_stack = HashMap::new();

        for node in self.graph.node_indices() {
            if !visited.get(&node).copied().unwrap_or(false)
                && let Some(cycle) = self.dfs_find_cycle(node, &mut visited, &mut rec_stack) {
                    cycles.push(cycle);
                }
        }

        cycles
    }

    /// DFS helper to find a single cycle
    fn dfs_find_cycle(
        &self,
        node: NodeIndex,
        visited: &mut HashMap<NodeIndex, bool>,
        rec_stack: &mut HashMap<NodeIndex, bool>,
    ) -> Option<Vec<String>> {
        visited.insert(node, true);
        rec_stack.insert(node, true);

        for edge in self.graph.edges(node) {
            let neighbor = edge.target();

            if !visited.get(&neighbor).copied().unwrap_or(false) {
                if let Some(cycle) = self.dfs_find_cycle(neighbor, visited, rec_stack) {
                    return Some(cycle);
                }
            } else if rec_stack.get(&neighbor).copied().unwrap_or(false) {
                // Found a cycle - trace it back
                let mut cycle = vec![self.graph[neighbor].clone()];
                cycle.push(self.graph[node].clone());
                return Some(cycle);
            }
        }

        rec_stack.insert(node, false);
        None
    }

    /// Get topological sort of tasks (dependency-ordered)
    ///
    /// Returns tasks in an order where dependencies come before dependents.
    /// Returns error if the graph contains cycles.
    pub fn topological_sort(&self) -> GraphResult<Vec<String>> {
        if self.has_cycles() {
            let cycles = self.find_cycles();
            if let Some(cycle) = cycles.first() {
                return Err(DependencyGraphError::CircularDependency(cycle.clone()));
            }
        }

        match toposort(&self.graph, None) {
            Ok(sorted) => Ok(sorted
                .into_iter()
                .map(|idx| self.graph[idx].clone())
                .collect()),
            Err(_) => {
                let cycles = self.find_cycles();
                Err(DependencyGraphError::CircularDependency(
                    cycles.first().cloned().unwrap_or_default(),
                ))
            }
        }
    }

    /// Get all tasks that the given task depends on (predecessors)
    pub fn get_dependencies(&self, task_id: &str) -> GraphResult<Vec<String>> {
        let node = self
            .task_indices
            .get(task_id)
            .ok_or_else(|| DependencyGraphError::TaskNotFound(task_id.to_string()))?;

        let deps = self
            .graph
            .edges_directed(*node, petgraph::Direction::Incoming)
            .map(|edge| self.graph[edge.source()].clone())
            .collect();

        Ok(deps)
    }

    /// Get all tasks that depend on the given task (successors)
    pub fn get_dependents(&self, task_id: &str) -> GraphResult<Vec<String>> {
        let node = self
            .task_indices
            .get(task_id)
            .ok_or_else(|| DependencyGraphError::TaskNotFound(task_id.to_string()))?;

        let deps = self
            .graph
            .edges_directed(*node, petgraph::Direction::Outgoing)
            .map(|edge| self.graph[edge.target()].clone())
            .collect();

        Ok(deps)
    }

    /// Validate a potential new dependency
    ///
    /// Returns Ok if adding the dependency would not create a cycle.
    pub fn validate_dependency(
        &self,
        source_id: &str,
        target_id: &str,
    ) -> GraphResult<()> {
        // Check if task depends on itself
        if source_id == target_id {
            return Err(DependencyGraphError::InvalidDependency(
                source_id.to_string(),
                target_id.to_string(),
            ));
        }

        // Check if tasks exist
        if !self.task_indices.contains_key(source_id) {
            return Err(DependencyGraphError::TaskNotFound(source_id.to_string()));
        }
        if !self.task_indices.contains_key(target_id) {
            return Err(DependencyGraphError::TaskNotFound(target_id.to_string()));
        }

        // Create a temporary graph with the new dependency
        let mut temp_graph = self.graph.clone();
        let source_idx = self.task_indices[source_id];
        let target_idx = self.task_indices[target_id];
        temp_graph.add_edge(source_idx, target_idx, ());

        // Check if it creates a cycle
        if is_cyclic_directed(&temp_graph) {
            return Err(DependencyGraphError::CircularDependency(vec![
                source_id.to_string(),
                target_id.to_string(),
            ]));
        }

        Ok(())
    }

    /// Get all tasks in dependency chain from start to end
    pub fn get_dependency_chain(
        &self,
        start_id: &str,
        end_id: &str,
    ) -> GraphResult<Vec<String>> {
        let start_idx = self
            .task_indices
            .get(start_id)
            .ok_or_else(|| DependencyGraphError::TaskNotFound(start_id.to_string()))?;
        let end_idx = self
            .task_indices
            .get(end_id)
            .ok_or_else(|| DependencyGraphError::TaskNotFound(end_id.to_string()))?;

        // Use BFS to find path
        let path = petgraph::algo::astar(
            &self.graph,
            *start_idx,
            |n| n == *end_idx,
            |_| 1,
            |_| 0,
        );

        match path {
            Some((_cost, path)) => Ok(path
                .into_iter()
                .map(|idx| self.graph[idx].clone())
                .collect()),
            None => Ok(Vec::new()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::gantt::{DependencyType, TaskType};
    use chrono::Utc;

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
            metadata: std::collections::HashMap::new(),
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
    fn test_graph_creation() {
        let tasks = vec![
            create_test_task("task1"),
            create_test_task("task2"),
            create_test_task("task3"),
        ];

        let dependencies = vec![
            create_dependency("task1", "task2"),
            create_dependency("task2", "task3"),
        ];

        let graph = DependencyGraph::new(&tasks, &dependencies);
        assert_eq!(graph.task_indices.len(), 3);
        assert_eq!(graph.graph.node_count(), 3);
        assert_eq!(graph.graph.edge_count(), 2);
    }

    #[test]
    fn test_no_cycles_in_linear_chain() {
        let tasks = vec![
            create_test_task("task1"),
            create_test_task("task2"),
            create_test_task("task3"),
        ];

        let dependencies = vec![
            create_dependency("task1", "task2"),
            create_dependency("task2", "task3"),
        ];

        let graph = DependencyGraph::new(&tasks, &dependencies);
        assert!(!graph.has_cycles());
    }

    #[test]
    fn test_cycle_detection() {
        let tasks = vec![
            create_test_task("task1"),
            create_test_task("task2"),
            create_test_task("task3"),
        ];

        let dependencies = vec![
            create_dependency("task1", "task2"),
            create_dependency("task2", "task3"),
            create_dependency("task3", "task1"), // Creates cycle
        ];

        let graph = DependencyGraph::new(&tasks, &dependencies);
        assert!(graph.has_cycles());
    }

    #[test]
    fn test_topological_sort() {
        let tasks = vec![
            create_test_task("task1"),
            create_test_task("task2"),
            create_test_task("task3"),
        ];

        let dependencies = vec![
            create_dependency("task1", "task2"),
            create_dependency("task2", "task3"),
        ];

        let graph = DependencyGraph::new(&tasks, &dependencies);
        let sorted = graph.topological_sort().unwrap();

        // task1 should come before task2, task2 before task3
        let task1_pos = sorted.iter().position(|t| t == "task1").unwrap();
        let task2_pos = sorted.iter().position(|t| t == "task2").unwrap();
        let task3_pos = sorted.iter().position(|t| t == "task3").unwrap();

        assert!(task1_pos < task2_pos);
        assert!(task2_pos < task3_pos);
    }

    #[test]
    fn test_validate_self_dependency() {
        let tasks = vec![create_test_task("task1")];
        let graph = DependencyGraph::new(&tasks, &[]);

        let result = graph.validate_dependency("task1", "task1");
        assert!(matches!(
            result,
            Err(DependencyGraphError::InvalidDependency(_, _))
        ));
    }

    #[test]
    fn test_validate_would_create_cycle() {
        let tasks = vec![
            create_test_task("task1"),
            create_test_task("task2"),
            create_test_task("task3"),
        ];

        let dependencies = vec![
            create_dependency("task1", "task2"),
            create_dependency("task2", "task3"),
        ];

        let graph = DependencyGraph::new(&tasks, &dependencies);

        // Adding task3 -> task1 would create a cycle
        let result = graph.validate_dependency("task3", "task1");
        assert!(matches!(
            result,
            Err(DependencyGraphError::CircularDependency(_))
        ));
    }

    #[test]
    fn test_get_dependencies() {
        let tasks = vec![
            create_test_task("task1"),
            create_test_task("task2"),
            create_test_task("task3"),
        ];

        let dependencies = vec![
            create_dependency("task1", "task3"),
            create_dependency("task2", "task3"),
        ];

        let graph = DependencyGraph::new(&tasks, &dependencies);
        let deps = graph.get_dependencies("task3").unwrap();

        assert_eq!(deps.len(), 2);
        assert!(deps.contains(&"task1".to_string()));
        assert!(deps.contains(&"task2".to_string()));
    }

    #[test]
    fn test_get_dependents() {
        let tasks = vec![
            create_test_task("task1"),
            create_test_task("task2"),
            create_test_task("task3"),
        ];

        let dependencies = vec![
            create_dependency("task1", "task2"),
            create_dependency("task1", "task3"),
        ];

        let graph = DependencyGraph::new(&tasks, &dependencies);
        let dependents = graph.get_dependents("task1").unwrap();

        assert_eq!(dependents.len(), 2);
        assert!(dependents.contains(&"task2".to_string()));
        assert!(dependents.contains(&"task3".to_string()));
    }
}
