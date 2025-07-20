//! # Menu Component Module
//!
//! Navigation menu components for creating hierarchical navigation structures.
//! Provides a complete menu system with items, titles, and submenus.
//!
//! ## Components
//!
//! - [`Menu`] - Root menu container with direction and size configuration
//! - [`MenuItem`] - Individual menu items with selection state
//! - [`MenuTitle`] - Section titles within menus
//! - [`SubMenu`] - Nested submenu containers
//!
//! ## Features
//!
//! - **Responsive Layout**: Horizontal and vertical orientations
//! - **Size Variants**: Five size options from extra small to extra large
//! - **State Management**: Automatic or manual selection tracking
//! - **Accessibility**: Proper ARIA attributes and keyboard navigation
//! - **Nested Structure**: Support for submenus and hierarchical organization
//!
//! ## Usage
//!
//! ```rust
//! use leptos::prelude::*;
//! use leptos_daisyui::Menu;
//!
//! #[component]
//! fn App() -> impl IntoView {
//!     let selected = RwSignal::new(Some("home".to_string()));
//!
//!     view! {
//!         <Menu selected=selected>
//!             <MenuItem value="home".to_string()>"Home"</MenuItem>
//!             <MenuItem value="about".to_string()>"About"</MenuItem>
//!             <MenuTitle>"Settings"</MenuTitle>
//!             <MenuItem value="profile".to_string()>"Profile"</MenuItem>
//!         </Menu>
//!     }
//! }
//! ```
//!
//! ## CSS Classes
//!
//! - `.menu` - Base menu container
//! - `.menu-horizontal` - Horizontal layout
//! - `.menu-{size}` - Size variants (xs, sm, md, lg, xl)
//! - `.menu-title` - Section title styling
//! - `.menu-active` - Active menu item

mod component;
mod style;

pub use component::*;
pub use style::*;
