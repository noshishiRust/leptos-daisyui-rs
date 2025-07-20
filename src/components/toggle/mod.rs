//! # Toggle Component
//!
//! A toggle switch component that provides a visual alternative to traditional checkboxes for binary state control.
//!
//! ## Features
//!
//! - **Color variants**: Multiple color options including primary, secondary, accent, and semantic colors
//! - **Size variants**: Five different sizes from extra small to extra large
//! - **Reactive props**: All properties support reactive updates via Leptos signals
//! - **Binary state**: Represents on/off, enabled/disabled, or true/false states
//! - **Accessibility**: Proper checkbox semantics with toggle visual styling
//! - **Form integration**: Full support for HTML form attributes and validation
//!
//! ## Usage
//!
//! ```rust
//! use leptos::prelude::*;
//! use leptos_daisyui::toggle::{Toggle, ToggleColor, ToggleSize};
//!
//! #[component]
//! fn Example() -> impl IntoView {
//!     let (enabled, set_enabled) = signal(false);
//!
//!     view! {
//!         <div class="form-control">
//!             <label class="label cursor-pointer">
//!                 <span class="label-text">"Enable notifications"</span>
//!                 <Toggle
//!                     color=ToggleColor::Primary
//!                     size=ToggleSize::Md
//!                     checked=enabled
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
//! - Base: `toggle`
//! - Colors: `toggle-primary`, `toggle-secondary`, `toggle-accent`, `toggle-success`, `toggle-warning`, `toggle-info`, `toggle-error`
//! - Sizes: `toggle-xs`, `toggle-sm`, `toggle-md`, `toggle-lg`, `toggle-xl`
//!
//! ## Accessibility
//!
//! The Toggle component follows ARIA guidelines:
//!
//! - Uses semantic `<input type="checkbox">` element with toggle styling
//! - Supports keyboard navigation (Space to toggle, Tab to focus)
//! - Proper focus management and visual indicators
//! - Screen reader compatible with state announcements
//! - Form validation support with native HTML attributes

mod component;
mod style;

pub use component::*;
pub use style::*;
