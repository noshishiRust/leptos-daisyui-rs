//! # Loading Component
//!
//! A visual loading indicator component that displays various animated loading states.
//! Perfect for showing loading states during data fetching, processing, or any asynchronous operations.
//!
//! ## Features
//!
//! - **Multiple animation types**: Spinner, dots, ring, ball, bars, infinity
//! - **Color variants**: Support for all theme colors
//! - **Size variants**: From extra small to large
//! - **Lightweight**: Pure CSS animations, no JavaScript required
//! - **Accessible**: Proper semantic markup for screen readers
//!
//! ## Examples
//!
//! ### Basic Loading Spinner
//! ```rust
//! use leptos::prelude::*;
//! use leptos_daisyui::Loading;
//!
//! #[component]
//! fn App() -> impl IntoView {
//!     view! {
//!         <Loading />
//!     }
//! }
//! ```
//!
//! ### Colored Loading Dots
//! ```rust
//! use leptos::prelude::*;
//! use leptos_daisyui::{Loading, LoadingType, LoadingColor};
//!
//! #[component]
//! fn App() -> impl IntoView {
//!     view! {
//!         <Loading
//!             loading_type=LoadingType::Dots
//!             color=LoadingColor::Primary
//!         />
//!     }
//! }
//! ```
//!
//! ### Large Loading Ring
//! ```rust
//! use leptos::prelude::*;
//! use leptos_daisyui::{Loading, LoadingType, LoadingSize, LoadingColor};
//!
//! #[component]
//! fn App() -> impl IntoView {
//!     view! {
//!         <Loading
//!             loading_type=LoadingType::Ring
//!             size=LoadingSize::Lg
//!             color=LoadingColor::Success
//!         />
//!     }
//! }
//! ```

mod component;
mod style;

pub use component::*;
pub use style::*;
