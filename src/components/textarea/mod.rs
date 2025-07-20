//! # Textarea Component
//!
//! A multi-line text input component that provides a styled interface for entering longer text content.
//!
//! ## Features
//!
//! - **Style variants**: Default and ghost styling options
//! - **Color variants**: Multiple color options including primary, secondary, accent, and semantic colors
//! - **Size variants**: Five different sizes from extra small to extra large
//! - **Reactive props**: All properties support reactive updates via Leptos signals
//! - **Auto-resize**: Native browser textarea behavior with optional resizing
//! - **Accessibility**: Proper textarea semantics and keyboard navigation
//! - **Form integration**: Full support for HTML form attributes and validation
//!
//! ## Usage
//!
//! ```rust
//! use leptos::prelude::*;
//! use leptos_daisyui::textarea::{Textarea, TextareaColor, TextareaSize, TextareaStyle};
//!
//! #[component]
//! fn Example() -> impl IntoView {
//!     view! {
//!         <div class="form-control">
//!             <label class="label">
//!                 <span class="label-text">"Your message"</span>
//!             </label>
//!             <Textarea
//!                 color=TextareaColor::Primary
//!                 size=TextareaSize::Md
//!                 class="h-24"
//!             />
//!             <label class="label">
//!                 <span class="label-text-alt">"Alt label"</span>
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
//! - Base: `textarea`
//! - Styles: `textarea-ghost`
//! - Colors: `textarea-primary`, `textarea-secondary`, `textarea-accent`, `textarea-info`, `textarea-success`, `textarea-warning`, `textarea-error`
//! - Sizes: `textarea-xs`, `textarea-sm`, `textarea-md`, `textarea-lg`, `textarea-xl`
//!
//! ## Accessibility
//!
//! The Textarea component follows ARIA guidelines:
//!
//! - Uses semantic `<textarea>` element
//! - Supports keyboard navigation and text selection
//! - Proper focus management and visual indicators
//! - Screen reader compatible with content announcements
//! - Form validation support with native HTML attributes

mod component;
mod style;

pub use component::*;
pub use style::*;
