//! # DataTable Component
//!
//! Production-ready data table with sorting, pagination, loading states,
//! and efficient handling of large datasets.
//!
//! ## Features
//! - Column-based sorting (click headers to toggle Asc/Desc)
//! - Pagination with customizable page size
//! - Loading and empty states
//! - Fully themed with daisyUI
//! - Efficient index-based operations for 10,000+ rows
//!
//! ## Example
//! ```rust,no_run
//! use std::collections::HashMap;
//! use leptos::prelude::*;
//! use leptos_daisyui_rs::components::*;
//!
//! #[component]
//! fn MyTable() -> impl IntoView {
//!     let columns = vec![
//!         Column::new("name", "Name"),
//!         Column::new("email", "Email"),
//!         Column::new_non_sortable("status", "Status"),
//!     ];
//!
//!     let data = vec![
//!         HashMap::from([
//!             ("name", "Alice".to_string()),
//!             ("email", "alice@example.com".to_string()),
//!             ("status", "Active".to_string()),
//!         ]),
//!     ];
//!
//!     view! {
//!         <DataTable
//!             columns=Signal::derive(move || columns.clone())
//!             data=Signal::derive(move || data.clone())
//!             page_size=10
//!         />
//!     }
//! }
//! ```

mod body;
mod component;
mod controls;
mod header;

/// Types for DataTable component including Column, SortOrder, and configuration structs
pub mod types;

pub use component::*;
pub use types::*;
