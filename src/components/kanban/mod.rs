/// Kanban board component for task management
///
/// This module provides a production-ready Kanban board implementation with:
/// - Drag-and-drop card movement between columns
/// - Collapsible columns with localStorage persistence
/// - Independent column scrolling
/// - Advanced filtering (assignee, label, priority, due date)
/// - Real-time search with debouncing
/// - WIP (Work In Progress) limits
/// - Responsive design and accessibility
///
/// # Example
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui_rs::components::*;
///
/// #[component]
/// pub fn App() -> impl IntoView {
///     let columns = vec![
///         KanbanColumn::new("todo", "To Do"),
///         KanbanColumn::new("in-progress", "In Progress"),
///         KanbanColumn::new("done", "Done"),
///     ];
///
///     view! {
///         <KanbanBoard
///             board_id="my-board"
///             columns=columns
///         />
///     }
/// }
/// ```

mod board;
mod column;
mod card;
mod types;
mod filters;

pub use board::*;
pub use column::*;
pub use card::*;
pub use types::*;
pub use filters::*;

#[cfg(test)]
mod tests;
