//! # daisyUI Input Components
//!
//! The input module provides type-safe Leptos wrappers for daisyUI's input components.
//! Inputs allow users to enter and edit text data in forms and user interfaces.
//!
//! ## daisyUI Input Component
//!
//! Text inputs are fundamental form elements that can be styled with various colors, sizes, and styles.
//! The daisyUI input system provides a clean and consistent styling approach for text input fields.
//!
//! ### Related daisyUI Classes
//! - **Component**: `input` - The main input class
//! - **Colors**: `input-neutral`, `input-primary`, `input-secondary`, `input-accent`, `input-info`, `input-success`, `input-warning`, `input-error`
//! - **Styles**: `input-ghost`
//! - **Sizes**: `input-xs`, `input-sm`, `input-md`, `input-lg`, `input-xl`
//! - **States**: `input-disabled`
//!
//! ### Components Provided
//! - `Input` - Standard text input element (`<input>`)
//!
//! For more information, see: https://daisyui.com/components/input/

mod component;
mod style;

pub use component::*;
pub use style::*;
