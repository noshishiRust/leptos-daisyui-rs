//! # Range Component
//!
//! A range slider input component that provides a visual interface for selecting numeric values within a specified range.
//!
//! ## Features
//!
//! - **Color variants**: Multiple color options including primary, secondary, accent, and semantic colors
//! - **Size variants**: Five different sizes from extra small to large
//! - **Reactive props**: All properties support reactive updates via Leptos signals
//! - **Numeric precision**: Support for floating-point values with configurable step sizes
//! - **Event handling**: Support for input events with numeric value callbacks
//! - **Accessibility**: Proper slider semantics and keyboard navigation
//! - **Customizable bounds**: Configurable minimum and maximum values
//!
//! ## Usage
//!
//! ```rust
//! use leptos::prelude::*;
//! use leptos_daisyui::range::{Range, RangeColor, RangeSize};
//!
//! #[component]
//! fn Example() -> impl IntoView {
//!     let (value, set_value) = signal(50.0);
//!
//!     view! {
//!         <div class="form-control">
//!             <label class="label">
//!                 <span class="label-text">"Volume: " {move || value.get()}</span>
//!             </label>
//!             <Range
//!                 color=RangeColor::Primary
//!                 size=RangeSize::Md
//!                 value=value
//!                 min=0.0
//!                 max=100.0
//!                 step=1.0
//!                 on_input=Box::new(move |val| set_value.set(val))
//!             />
//!         </div>
//!     }
//! }
//! ```
//!
//! ## CSS Classes
//!
//! This component uses the following daisyUI CSS classes:
//!
//! - Base: `range`
//! - Colors: `range-primary`, `range-secondary`, `range-accent`, `range-success`, `range-warning`, `range-info`, `range-error`
//! - Sizes: `range-xs`, `range-sm`, `range-md`, `range-lg`
//!
//! ## Accessibility
//!
//! The Range component follows ARIA guidelines:
//!
//! - Uses semantic `<input type="range">` element
//! - Supports keyboard navigation (arrow keys for value adjustment)
//! - Proper focus management and visual indicators
//! - Screen reader compatible with value announcements
//! - ARIA attributes for minimum, maximum, and current values

mod component;
mod style;

pub use component::*;
pub use style::*;
