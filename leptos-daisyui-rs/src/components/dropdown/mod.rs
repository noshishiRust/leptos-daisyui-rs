//! # Dropdown Component Module
//!
//! Dropdown menu components for contextual actions and selections.
//! Built on HTML `<details>` element with accessible trigger and content areas.
//!
//! ## Components
//!
//! - [`Dropdown`] - Root dropdown container with state management
//! - [`DropdownTrigger`] - Clickable trigger element (button, link, etc.)
//! - [`DropdownContent`] - Container for dropdown menu items
//!
//! ## Features
//!
//! - **Native Behavior**: Built on `<details>` element for accessibility
//! - **Hover Support**: Optional hover-to-open functionality
//! - **State Control**: Manual open/close state management
//! - **Positioning**: Automatic positioning with overflow handling
//! - **Keyboard Navigation**: Native focus and keyboard support
//!
//! ## CSS Classes
//!
//! - `.dropdown` - Base dropdown container
//! - `.dropdown-hover` - Enable hover-to-open behavior
//! - `.dropdown-open` - Force open state
//! - `.dropdown-content` - Content container with positioning

mod component;
mod style;

pub use component::*;
pub use style::*;
