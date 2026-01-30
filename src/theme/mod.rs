//! Advanced runtime theming system
//!
//! This module provides a comprehensive theming system for leptos-daisyui-rs that allows
//! runtime customization of daisyUI themes including colors, typography, borders, and
//! component-specific overrides.
//!
//! ## Features
//! - Base theme selection from daisyUI's built-in themes
//! - Runtime color customization with Oklahoma LCH color space
//! - Typography customization (fonts, scales, weights)
//! - Border and spacing customization
//! - Component-specific style overrides
//! - Theme persistence with localStorage
//! - JSON import/export for theme sharing
//!
//! ## Example
//! ```rust,no_run
//! use leptos::prelude::*;
//! use leptos_daisyui_rs::theme::*;
//!
//! #[component]
//! fn App() -> impl IntoView {
//!     let theme = ThemeConfiguration::new("dark")
//!         .with_metadata(ThemeMetadata::new("My Custom Theme"));
//!
//!     view! {
//!         <div>
//!             "App with custom theme"
//!         </div>
//!     }
//! }
//! ```

pub mod css_injection;
pub mod provider;
pub mod types;

pub use css_injection::*;
pub use provider::*;
pub use types::*;
