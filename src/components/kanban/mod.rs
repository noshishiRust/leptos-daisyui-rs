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
/// use leptos_daisyui_rs::components::kanban::{KanbanBoard, KanbanColumn};
///
/// #[component]
/// pub fn App() -> impl IntoView {
///     let columns = Signal::derive(|| vec![
///         KanbanColumn::new("todo", "To Do"),
///         KanbanColumn::new("in-progress", "In Progress"),
///         KanbanColumn::new("done", "Done"),
///     ]);
///
///     view! {
///         <KanbanBoard
///             _board_id="my-board"
///             title="My Board"
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
mod drag;

pub use board::*;
pub use column::*;
pub use card::*;
pub use types::*;
pub use filters::*;
pub use drag::*;

#[cfg(test)]
mod tests;
