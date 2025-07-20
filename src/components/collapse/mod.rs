//! # Collapse Component
//!
//! A collapsible container that can expand and contract to show or hide content.
//! Perfect for FAQs, accordions, and other expandable content sections.
//!
//! ## Features
//!
//! - **Multiple variants**: Arrow, plus, or no indicator
//! - **State control**: Open, closed, or toggle-based
//! - **Checkbox integration**: Built-in checkbox for state management
//! - **Accessibility**: Proper ARIA attributes and keyboard navigation
//! - **Composable parts**: Separate title and content components
//!
//! ## Examples
//!
//! ### Basic Collapse
//! ```rust
//! use leptos::prelude::*;
//! use leptos_daisyui::Collapse;
//!
//! #[component]
//! fn App() -> impl IntoView {
//!     view! {
//!         <Collapse>
//!             <CollapseTitle>"Click to expand"</CollapseTitle>
//!             <CollapseContent>"Hidden content here"</CollapseContent>
//!         </Collapse>
//!     }
//! }
//! ```
//!
//! ### With Arrow Indicator
//! ```rust
//! use leptos::prelude::*;
//! use leptos_daisyui::{Collapse, CollapseModifier};
//!
//! #[component]
//! fn App() -> impl IntoView {
//!     view! {
//!         <Collapse modifier=CollapseModifier::Arrow>
//!             <CollapseTitle>"Expandable section"</CollapseTitle>
//!             <CollapseContent>
//!                 <p>"This content is hidden by default."</p>
//!             </CollapseContent>
//!         </Collapse>
//!     }
//! }
//! ```
//!
//! ### Checkbox-controlled Collapse
//! ```rust
//! use leptos::prelude::*;
//! use leptos_daisyui::{CollapseCheckbox, CollapseModifier};
//!
//! #[component]
//! fn App() -> impl IntoView {
//!     let (checked, set_checked) = signal(false);
//!     
//!     view! {
//!         <CollapseCheckbox
//!             modifier=CollapseModifier::Plus
//!             checked=checked
//!         >
//!             <CollapseTitle>"Toggle me"</CollapseTitle>
//!             <CollapseContent>"Controlled content"</CollapseContent>
//!         </CollapseCheckbox>
//!     }
//! }
//! ```

mod component;
mod style;

pub use component::*;
pub use style::*;
