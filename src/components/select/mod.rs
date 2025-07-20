//! # Select Component
//!
//! A dropdown selection component that provides a styled interface for choosing from multiple options.
//!
//! ## Features
//!
//! - **Style variants**: Default and ghost styling options
//! - **Color variants**: Multiple color options including primary, secondary, accent, and semantic colors
//! - **Size variants**: Five different sizes from extra small to extra large
//! - **Reactive props**: All properties support reactive updates via Leptos signals
//! - **Option support**: Companion `SelectOption` component for creating option elements
//! - **Accessibility**: Proper select semantics and keyboard navigation
//! - **Form integration**: Full support for HTML form attributes and validation
//!
//! ## Usage
//!
//! ```rust
//! use leptos::prelude::*;
//! use leptos_daisyui::select::{Select, SelectOption, SelectColor, SelectSize, SelectStyle};
//!
//! #[component]
//! fn Example() -> impl IntoView {
//!     view! {
//!         <div class="form-control w-full max-w-xs">
//!             <label class="label">
//!                 <span class="label-text">"Choose an option"</span>
//!             </label>
//!             <Select
//!                 style=SelectStyle::Default
//!                 color=SelectColor::Primary
//!                 size=SelectSize::Md
//!             >
//!                 <SelectOption>"Option 1"</SelectOption>
//!                 <SelectOption>"Option 2"</SelectOption>
//!                 <SelectOption disabled=true>"Disabled Option"</SelectOption>
//!             </Select>
//!         </div>
//!     }
//! }
//! ```
//!
//! ## CSS Classes
//!
//! This component uses the following daisyUI CSS classes:
//!
//! - Base: `select`
//! - Styles: `select-ghost`
//! - Colors: `select-primary`, `select-secondary`, `select-accent`, `select-info`, `select-success`, `select-warning`, `select-error`
//! - Sizes: `select-xs`, `select-sm`, `select-md`, `select-lg`, `select-xl`
//!
//! ## Accessibility
//!
//! The Select component follows ARIA guidelines:
//!
//! - Uses semantic `<select>` element
//! - Supports keyboard navigation (arrow keys, typing to search)
//! - Proper focus management and visual indicators
//! - Screen reader compatible with option announcements
//! - Form validation support with native HTML attributes

mod component;
mod style;

pub use component::*;
pub use style::*;
