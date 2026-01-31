use std::collections::HashMap;
use chrono::{DateTime, Utc};

use crate::components::gantt::models::GanttTask;

/// Represents a task node in the hierarchical tree structure
#[derive(Clone, Debug)]
pub struct TaskTreeNode {
    /// The task data
    pub task: GanttTask,

    /// Child task IDs
    pub children: Vec<String>,

    /// Depth level in the tree (0 for root tasks)
    pub depth: usize,

    /// Whether this node is expanded in the UI
    pub is_expanded: bool,
}

impl TaskTreeNode {
    /// Create a new tree node from a task
    pub fn new(task: GanttTask, depth: usize) -> Self {
        Self {
            task,
            children: Vec::new(),
            depth,
            is_expanded: true, // Expanded by default
        }
    }

    /// Check if this node has children
    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }
}

/// Builds a tree structure from a flat list of tasks
pub struct TreeBuilder {
    /// Map of task ID to tree node
    nodes: HashMap<String, TaskTreeNode>,

    /// Root task IDs (tasks with no parent)
    root_ids: Vec<String>,
}

impl TreeBuilder {
    /// Create a new tree builder
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            root_ids: Vec::new(),
        }
    }

    /// Build a tree from a flat list of tasks
    pub fn build_tree(tasks: Vec<GanttTask>) -> TaskTree {
        let mut builder = Self::new();

        // First pass: Create all nodes and identify roots
        for task in tasks {
            let depth = 0; // Will be calculated in second pass
            let node = TaskTreeNode::new(task.clone(), depth);

            if task.parent_id.is_none() {
                builder.root_ids.push(task.id.clone());
            }

            builder.nodes.insert(task.id.clone(), node);
        }

        // Second pass: Build parent-child relationships and calculate depth
        let node_ids: Vec<String> = builder.nodes.keys().cloned().collect();
        for id in node_ids {
            if let Some(node) = builder.nodes.get(&id).cloned()
                && let Some(parent_id) = &node.task.parent_id {
                    // Add this task to parent's children
                    if let Some(parent) = builder.nodes.get_mut(parent_id)
                        && !parent.children.contains(&id) {
                            parent.children.push(id.clone());
                        }
                }
        }

        // Third pass: Calculate depths
        builder.calculate_depths();

        TaskTree {
            nodes: builder.nodes,
            root_ids: builder.root_ids,
        }
    }

    /// Calculate depth for all nodes using breadth-first traversal
    fn calculate_depths(&mut self) {
        let mut to_process: Vec<(String, usize)> = self.root_ids
            .iter()
            .map(|id| (id.clone(), 0))
            .collect();

        while let Some((id, depth)) = to_process.pop() {
            if let Some(node) = self.nodes.get_mut(&id) {
                node.depth = depth;

                // Queue children with depth + 1
                for child_id in node.children.clone() {
                    to_process.push((child_id, depth + 1));
                }
            }
        }
    }
}

/// Represents the complete task tree structure
#[derive(Clone, Debug)]
pub struct TaskTree {
    /// Map of task ID to tree node
    pub nodes: HashMap<String, TaskTreeNode>,

    /// Root task IDs (tasks with no parent)
    pub root_ids: Vec<String>,
}

impl TaskTree {
    /// Get a node by task ID
    pub fn get_node(&self, id: &str) -> Option<&TaskTreeNode> {
        self.nodes.get(id)
    }

    /// Get a mutable node by task ID
    pub fn get_node_mut(&mut self, id: &str) -> Option<&mut TaskTreeNode> {
        self.nodes.get_mut(id)
    }

    /// Get all root nodes
    pub fn get_roots(&self) -> Vec<&TaskTreeNode> {
        self.root_ids
            .iter()
            .filter_map(|id| self.nodes.get(id))
            .collect()
    }

    /// Get children of a task
    pub fn get_children(&self, id: &str) -> Vec<&TaskTreeNode> {
        if let Some(node) = self.nodes.get(id) {
            node.children
                .iter()
                .filter_map(|child_id| self.nodes.get(child_id))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get all descendants of a task (recursive)
    pub fn get_descendants(&self, id: &str) -> Vec<&TaskTreeNode> {
        let mut descendants = Vec::new();
        let mut to_process = vec![id];

        while let Some(current_id) = to_process.pop() {
            if let Some(node) = self.nodes.get(current_id) {
                for child_id in &node.children {
                    if let Some(child_node) = self.nodes.get(child_id) {
                        descendants.push(child_node);
                        to_process.push(child_id);
                    }
                }
            }
        }

        descendants
    }

    /// Calculate parent task dates from children
    /// Returns a map of task ID to (start, end) dates that need updating
    pub fn calculate_parent_dates(&self) -> HashMap<String, (DateTime<Utc>, DateTime<Utc>)> {
        let mut updates = HashMap::new();

        // Process from leaves to roots (bottom-up)
        let mut processed = HashMap::new();

        for root_id in &self.root_ids {
            self.calculate_parent_dates_recursive(root_id, &mut updates, &mut processed);
        }

        updates
    }

    /// Recursive helper to calculate parent dates
    fn calculate_parent_dates_recursive(
        &self,
        id: &str,
        updates: &mut HashMap<String, (DateTime<Utc>, DateTime<Utc>)>,
        processed: &mut HashMap<String, bool>,
    ) -> Option<(DateTime<Utc>, DateTime<Utc>)> {
        // Avoid reprocessing
        if processed.contains_key(id) {
            if let Some(node) = self.nodes.get(id) {
                return Some((node.task.start, node.task.end));
            }
            return None;
        }

        let node = self.nodes.get(id)?;
        processed.insert(id.to_string(), true);

        // If no children, use task's own dates
        if node.children.is_empty() {
            return Some((node.task.start, node.task.end));
        }

        // Calculate from children
        let mut min_start: Option<DateTime<Utc>> = None;
        let mut max_end: Option<DateTime<Utc>> = None;

        for child_id in &node.children {
            if let Some((child_start, child_end)) =
                self.calculate_parent_dates_recursive(child_id, updates, processed)
            {
                min_start = Some(match min_start {
                    Some(current) => current.min(child_start),
                    None => child_start,
                });

                max_end = Some(match max_end {
                    Some(current) => current.max(child_end),
                    None => child_end,
                });
            }
        }

        if let (Some(start), Some(end)) = (min_start, max_end) {
            // Only add to updates if dates changed
            if start != node.task.start || end != node.task.end {
                updates.insert(id.to_string(), (start, end));
            }
            Some((start, end))
        } else {
            Some((node.task.start, node.task.end))
        }
    }

    /// Get tasks in display order (depth-first, respecting expanded state)
    pub fn get_display_order(&self, expanded_state: &HashMap<String, bool>) -> Vec<String> {
        let mut result = Vec::new();

        for root_id in &self.root_ids {
            self.add_to_display_order(root_id, &mut result, expanded_state);
        }

        result
    }

    /// Recursive helper for display order
    fn add_to_display_order(
        &self,
        id: &str,
        result: &mut Vec<String>,
        expanded_state: &HashMap<String, bool>,
    ) {
        result.push(id.to_string());

        // Check if this node is expanded
        let is_expanded = expanded_state.get(id).copied().unwrap_or(true);

        if is_expanded
            && let Some(node) = self.nodes.get(id) {
                for child_id in &node.children {
                    self.add_to_display_order(child_id, result, expanded_state);
                }
            }
    }

    /// Toggle expand/collapse state for a task
    pub fn toggle_expanded(&mut self, id: &str) {
        if let Some(node) = self.nodes.get_mut(id) {
            node.is_expanded = !node.is_expanded;
        }
    }

    /// Check if a task is expanded
    pub fn is_expanded(&self, id: &str) -> bool {
        self.nodes
            .get(id)
            .map(|node| node.is_expanded)
            .unwrap_or(true)
    }
}

impl Default for TreeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::gantt::style::TaskType;
    use chrono::TimeZone;

    fn create_test_task(id: &str, name: &str, parent_id: Option<String>) -> GanttTask {
        let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2024, 1, 10, 0, 0, 0).unwrap();

        GanttTask {
            id: id.to_string(),
            name: name.to_string(),
            start,
            end,
            progress: 0.0,
            task_type: TaskType::Task,
            parent_id,
            dependencies: Vec::new(),
            assignees: Vec::new(),
            color: None,
            read_only: false,
            metadata: HashMap::new(),
        }
    }

    #[test]
    fn test_tree_node_creation() {
        let task = create_test_task("task-1", "Test Task", None);
        let node = TaskTreeNode::new(task, 0);

        assert_eq!(node.depth, 0);
        assert!(node.is_expanded);
        assert!(!node.has_children());
    }

    #[test]
    fn test_build_flat_tree() {
        let tasks = vec![
            create_test_task("task-1", "Task 1", None),
            create_test_task("task-2", "Task 2", None),
        ];

        let tree = TreeBuilder::build_tree(tasks);

        assert_eq!(tree.root_ids.len(), 2);
        assert_eq!(tree.nodes.len(), 2);
    }

    #[test]
    fn test_build_hierarchical_tree() {
        let tasks = vec![
            create_test_task("parent", "Parent Task", None),
            create_test_task("child-1", "Child 1", Some("parent".to_string())),
            create_test_task("child-2", "Child 2", Some("parent".to_string())),
        ];

        let tree = TreeBuilder::build_tree(tasks);

        assert_eq!(tree.root_ids.len(), 1);
        assert_eq!(tree.root_ids[0], "parent");

        let parent = tree.get_node("parent").unwrap();
        assert_eq!(parent.children.len(), 2);
        assert!(parent.children.contains(&"child-1".to_string()));
        assert!(parent.children.contains(&"child-2".to_string()));

        let child1 = tree.get_node("child-1").unwrap();
        assert_eq!(child1.depth, 1);
    }

    #[test]
    fn test_get_children() {
        let tasks = vec![
            create_test_task("parent", "Parent", None),
            create_test_task("child-1", "Child 1", Some("parent".to_string())),
            create_test_task("child-2", "Child 2", Some("parent".to_string())),
        ];

        let tree = TreeBuilder::build_tree(tasks);
        let children = tree.get_children("parent");

        assert_eq!(children.len(), 2);
    }

    #[test]
    fn test_get_descendants() {
        let tasks = vec![
            create_test_task("root", "Root", None),
            create_test_task("child", "Child", Some("root".to_string())),
            create_test_task("grandchild", "Grandchild", Some("child".to_string())),
        ];

        let tree = TreeBuilder::build_tree(tasks);
        let descendants = tree.get_descendants("root");

        assert_eq!(descendants.len(), 2);
    }

    #[test]
    fn test_calculate_parent_dates() {
        let start1 = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let end1 = Utc.with_ymd_and_hms(2024, 1, 5, 0, 0, 0).unwrap();
        let start2 = Utc.with_ymd_and_hms(2024, 1, 3, 0, 0, 0).unwrap();
        let end2 = Utc.with_ymd_and_hms(2024, 1, 10, 0, 0, 0).unwrap();

        let tasks = vec![
            GanttTask {
                id: "parent".to_string(),
                name: "Parent".to_string(),
                start: start1, // Should be updated to earliest child
                end: end1,     // Should be updated to latest child
                progress: 0.0,
                task_type: TaskType::Task,
                parent_id: None,
                dependencies: Vec::new(),
                assignees: Vec::new(),
                color: None,
                read_only: false,
                metadata: HashMap::new(),
            },
            GanttTask {
                id: "child-1".to_string(),
                name: "Child 1".to_string(),
                start: start1,
                end: end1,
                progress: 0.0,
                task_type: TaskType::Task,
                parent_id: Some("parent".to_string()),
                dependencies: Vec::new(),
                assignees: Vec::new(),
                color: None,
                read_only: false,
                metadata: HashMap::new(),
            },
            GanttTask {
                id: "child-2".to_string(),
                name: "Child 2".to_string(),
                start: start2,
                end: end2,
                progress: 0.0,
                task_type: TaskType::Task,
                parent_id: Some("parent".to_string()),
                dependencies: Vec::new(),
                assignees: Vec::new(),
                color: None,
                read_only: false,
                metadata: HashMap::new(),
            },
        ];

        let tree = TreeBuilder::build_tree(tasks);
        let updates = tree.calculate_parent_dates();

        assert!(updates.contains_key("parent"));
        let (start, end) = updates.get("parent").unwrap();
        assert_eq!(*start, start1); // Earliest child start
        assert_eq!(*end, end2);     // Latest child end
    }

    #[test]
    fn test_display_order() {
        let tasks = vec![
            create_test_task("root", "Root", None),
            create_test_task("child-1", "Child 1", Some("root".to_string())),
            create_test_task("child-2", "Child 2", Some("root".to_string())),
        ];

        let tree = TreeBuilder::build_tree(tasks);
        let expanded = HashMap::new();
        let order = tree.get_display_order(&expanded);

        assert_eq!(order.len(), 3);
        assert_eq!(order[0], "root");
        // Children order follows tree structure
        assert!(order.contains(&"child-1".to_string()));
        assert!(order.contains(&"child-2".to_string()));
    }

    #[test]
    fn test_display_order_collapsed() {
        let tasks = vec![
            create_test_task("root", "Root", None),
            create_test_task("child-1", "Child 1", Some("root".to_string())),
            create_test_task("child-2", "Child 2", Some("root".to_string())),
        ];

        let tree = TreeBuilder::build_tree(tasks);
        let mut expanded = HashMap::new();
        expanded.insert("root".to_string(), false); // Collapsed

        let order = tree.get_display_order(&expanded);

        // Should only show root when collapsed
        assert_eq!(order.len(), 1);
        assert_eq!(order[0], "root");
    }
}
