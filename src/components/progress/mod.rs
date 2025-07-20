//! # daisyUI Progress Components
//!
//! The progress module provides type-safe Leptos wrappers for daisyUI's progress components.
//! Progress indicators show the completion progress of a task or loading state.
//!
//! ## daisyUI Progress Component
//!
//! Progress bars are visual indicators that show the progress of a task or operation.
//! They display a partially filled bar that represents how much of the task has been completed.
//! daisyUI progress bars support different colors for various semantic meanings.
//!
//! ### Related daisyUI Classes
//! - **Component**: `progress` - The main progress bar class
//! - **Colors**: `progress-primary`, `progress-secondary`, `progress-accent`, `progress-info`, `progress-success`, `progress-warning`, `progress-error`
//!
//! ### Components Provided
//! - `Progress` - Standard progress bar element (`<progress>`)
//!
//! For more information, see: https://daisyui.com/components/progress/

mod component;
mod style;

pub use component::*;
pub use style::*;
