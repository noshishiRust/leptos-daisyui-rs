//! # Radio Component
//!
//! A radio button input component that provides a single-choice selection interface.
//!
//! ## Features
//!
//! - **Color variants**: Multiple color options including primary, secondary, accent, and semantic colors
//! - **Size variants**: Five different sizes from extra small to large
//! - **Reactive props**: All properties support reactive updates via Leptos signals
//! - **Event handling**: Support for change events with boolean state callbacks
//! - **Accessibility**: Proper radio button semantics and keyboard navigation
//! - **Form integration**: Full support for HTML form attributes and validation
//!
//! ## Usage
//!
//! ```rust
//! use leptos::prelude::*;
//! use leptos_daisyui::radio::{Radio, RadioColor, RadioSize};
//!
//! #[component]
//! fn Example() -> impl IntoView {
//!     let (selected, set_selected) = signal(String::new());
//!
//!     view! {
//!         <div class="form-control">
//!             <label class="label cursor-pointer">
//!                 <span class="label-text">"Option 1"</span>
//!                 <Radio
//!                     name="radio-group"
//!                     value="option1"
//!                     color=RadioColor::Primary
//!                     size=RadioSize::Md
//!                     checked=move || selected.get() == "option1"
//!                     on_change=Box::new(move |checked| {
//!                         if checked {
//!                             set_selected.set("option1".to_string());
//!                         }
//!                     })
//!                 />
//!             </label>
//!         </div>
//!     }
//! }
//! ```
//!
//! ## CSS Classes
//!
//! This component uses the following daisyUI CSS classes:
//!
//! - Base: `radio`
//! - Colors: `radio-primary`, `radio-secondary`, `radio-accent`, `radio-success`, `radio-warning`, `radio-info`, `radio-error`
//! - Sizes: `radio-xs`, `radio-sm`, `radio-md`, `radio-lg`
//!
//! ## Accessibility
//!
//! The Radio component follows ARIA guidelines:
//!
//! - Uses semantic `<input type="radio">` element
//! - Supports keyboard navigation (arrow keys for group navigation)
//! - Proper focus management and visual indicators
//! - Screen reader compatible with appropriate labels
//! - Form validation support with native HTML attributes

mod component;
mod style;

pub use component::*;
pub use style::*;
