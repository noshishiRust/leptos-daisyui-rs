//! # Tab Component Module
//!
//! Tab navigation components for organizing content into switchable panels.
//! Provides multiple tab styles and radio-based tab groups for accessibility.
//!
//! ## Components
//!
//! - [`Tabs`] - Root tab container with size and variant configuration
//! - [`Tab`] - Individual tab item with click handling and active state
//! - [`TabRadio`] - Radio-based tab for form integration and accessibility
//!
//! ## Features
//!
//! - **Multiple Styles**: Default, bordered, lifted, and boxed variants
//! - **Size Options**: Four size variants from extra small to large
//! - **State Management**: Active state tracking and click handling
//! - **Accessibility**: Radio-based tabs for proper keyboard navigation
//! - **Form Integration**: Radio tabs can be part of forms
//!
//! ## Tab Variants
//!
//! - **Default**: Clean minimal tabs
//! - **Bordered**: Tabs with bottom borders
//! - **Lifted**: Tabs that appear lifted from content
//! - **Boxed**: Tabs with full boxing and background
//!
//! ## Usage
//!
//! ```rust
//! use leptos::prelude::*;
//! use leptos_daisyui::{Tabs, Tab, TabVariant};
//!
//! #[component]
//! fn TabNavigation() -> impl IntoView {
//!     let active_tab = RwSignal::new(0);
//!
//!     view! {
//!         <Tabs variant=TabVariant::Bordered>
//!             <Tab
//!                 active=Signal::derive(move || active_tab.get() == 0)
//!                 on_click=Box::new(move || active_tab.set(0))
//!             >
//!                 "Home"
//!             </Tab>
//!             <Tab
//!                 active=Signal::derive(move || active_tab.get() == 1)
//!                 on_click=Box::new(move || active_tab.set(1))
//!             >
//!                 "About"
//!             </Tab>
//!             <Tab
//!                 active=Signal::derive(move || active_tab.get() == 2)
//!                 on_click=Box::new(move || active_tab.set(2))
//!             >
//!                 "Contact"
//!             </Tab>
//!         </Tabs>
//!     }
//! }
//! ```
//!
//! ## CSS Classes
//!
//! - `.tabs` - Base tab container
//! - `.tabs-{size}` - Size variants (xs, sm, md, lg)
//! - `.tabs-{variant}` - Style variants (bordered, lifted, boxed)
//! - `.tab` - Individual tab styling
//! - `.tab-active` - Active tab state
//! - `.tab-disabled` - Disabled tab state

mod component;
mod style;

pub use component::*;
pub use style::*;
