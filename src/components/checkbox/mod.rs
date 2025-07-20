//! # Checkbox Component
//!
//! A reactive wrapper around daisyUI's checkbox component for creating interactive checkboxes.
//!
//! ## Features
//!
//! - **Reactive Props**: All properties are reactive using Leptos signals
//! - **Color Variants**: Support for all daisyUI color themes (primary, secondary, accent, etc.)
//! - **Size Variants**: Five size options from extra small to extra large
//! - **Accessibility**: Proper ARIA attributes and keyboard navigation
//! - **Event Handling**: Change event handling with boolean value
//! - **Customizable**: Additional CSS classes can be added
//!
//! ## Basic Usage
//!
//! ```rust
//! use leptos::prelude::*;
//! use leptos_daisyui::Checkbox;
//!
//! #[component]
//! fn App() -> impl IntoView {
//!     let (checked, set_checked) = signal(false);
//!
//!     view! {
//!         <Checkbox
//!             checked=checked
//!             on_change=Box::new(move |value| set_checked.set(value))
//!         />
//!     }
//! }
//! ```
//!
//! ## Color Variants
//!
//! ```rust
//! use leptos::prelude::*;
//! use leptos_daisyui::{Checkbox, CheckboxColor};
//!
//! view! {
//!     <Checkbox color=CheckboxColor::Primary />
//!     <Checkbox color=CheckboxColor::Secondary />
//!     <Checkbox color=CheckboxColor::Accent />
//!     <Checkbox color=CheckboxColor::Success />
//!     <Checkbox color=CheckboxColor::Error />
//! }
//! ```
//!
//! ## Size Variants
//!
//! ```rust
//! use leptos::prelude::*;
//! use leptos_daisyui::{Checkbox, CheckboxSize};
//!
//! view! {
//!     <Checkbox size=CheckboxSize::Xs />
//!     <Checkbox size=CheckboxSize::Sm />
//!     <Checkbox size=CheckboxSize::Md />
//!     <Checkbox size=CheckboxSize::Lg />
//!     <Checkbox size=CheckboxSize::Xl />
//! }
//! ```
//!
//! ## With Custom Styling
//!
//! ```rust
//! use leptos::prelude::*;
//! use leptos_daisyui::{Checkbox, CheckboxColor, CheckboxSize};
//!
//! view! {
//!     <Checkbox
//!         color=CheckboxColor::Primary
//!         size=CheckboxSize::Lg
//!         class="m-4 shadow-lg"
//!     />
//! }
//! ```
//!
//! ## Controlled Component
//!
//! ```rust
//! use leptos::prelude::*;
//! use leptos_daisyui::Checkbox;
//!
//! #[component]
//! fn ControlledCheckbox() -> impl IntoView {
//!     let (checked, set_checked) = signal(false);
//!
//!     view! {
//!         <div>
//!             <Checkbox
//!                 checked=checked
//!                 on_change=Box::new(move |value| {
//!                     set_checked.set(value);
//!                     // Additional logic here
//!                 })
//!             />
//!             <p>"Checked: " {move || checked.get()}</p>
//!         </div>
//!     }
//! }
//! ```

mod component;
mod style;

pub use component::*;
pub use style::*;
