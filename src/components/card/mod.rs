//! # Card Component Module
//!
//! This module provides Leptos components for creating daisyUI cards, which are used to group
//! and display content in a structured format. Cards are one of the most commonly used UI
//! patterns for organizing information.
//!
//! ## Components
//!
//! - [`Card`] - Main card container with configurable style, size, and layout options
//! - [`CardBody`] - Content area of the card with default padding
//! - [`CardTitle`] - Styled heading for the card content
//! - [`CardActions`] - Container for action buttons, typically placed at the bottom
//!
//! ## Style Options
//!
//! Cards support various visual styles and size configurations:
//!
//! - **Styles**: Default, Border, Dash
//! - **Sizes**: XS, SM, MD, LG, XL
//! - **Layout**: Side layout, Image full coverage
//!
//! ## Usage Example
//!
//! ```rust
//! use leptos::prelude::*;
//! use leptos_daisyui::card::*;
//!
//! #[component]
//! fn Example() -> impl IntoView {
//!     view! {
//!         <Card style=CardStyle::Border size=CardSize::Lg>
//!             <CardBody>
//!                 <CardTitle>"Card Title"</CardTitle>
//!                 <p>"Card content goes here"</p>
//!                 <CardActions>
//!                     <button class="btn btn-primary">"Action"</button>
//!                 </CardActions>
//!             </CardBody>
//!         </Card>
//!     }
//! }
//! ```
//!
//! ## daisyUI Reference
//!
//! This module implements the daisyUI card component system:
//! - Base: `card`, `card-body`, `card-title`, `card-actions`
//! - Styles: `card-border`, `card-dash`
//! - Modifiers: `card-side`, `image-full`
//! - Sizes: `card-xs`, `card-sm`, `card-md`, `card-lg`, `card-xl`
//!
//! For more information, see the [daisyUI card documentation](https://daisyui.com/components/card/).

mod component;
mod style;

pub use component::*;
pub use style::*;
