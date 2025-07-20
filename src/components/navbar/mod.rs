//! # Navbar Component Module
//!
//! Responsive navigation bar components for application headers and navigation.
//! Provides a flexible layout system for organizing navigation elements.
//!
//! ## Components
//!
//! - [`Navbar`] - Root navigation bar container
//! - [`NavbarStart`] - Left-aligned navigation section
//! - [`NavbarCenter`] - Center-aligned navigation section  
//! - [`NavbarEnd`] - Right-aligned navigation section
//!
//! ## Features
//!
//! - **Flexible Layout**: Three-section layout system (start, center, end)
//! - **Responsive Design**: Adapts to different screen sizes
//! - **Semantic HTML**: Uses proper `<nav>` element for accessibility
//! - **Custom Styling**: Support for additional CSS classes
//!
//! ## Layout Structure
//!
//! ```text
//! ┌─────────────────────────────────────────────────┐
//! │ [NavbarStart]    [NavbarCenter]    [NavbarEnd]  │
//! │ Logo/Menu        Title/Search      User/Actions │
//! └─────────────────────────────────────────────────┘
//! ```
//!
//! ## Usage
//!
//! ```rust
//! use leptos::prelude::*;
//! use leptos_daisyui::{Navbar, NavbarStart, NavbarCenter, NavbarEnd};
//!
//! #[component]
//! fn AppHeader() -> impl IntoView {
//!     view! {
//!         <Navbar>
//!             <NavbarStart>
//!                 <a class="btn btn-ghost text-xl">"MyApp"</a>
//!             </NavbarStart>
//!             <NavbarCenter>
//!                 <input class="input input-bordered" placeholder="Search..."/>
//!             </NavbarCenter>
//!             <NavbarEnd>
//!                 <button class="btn btn-primary">"Login"</button>
//!             </NavbarEnd>
//!         </Navbar>
//!     }
//! }
//! ```
//!
//! ## CSS Classes
//!
//! - `.navbar` - Base navbar container with flexbox layout
//! - `.navbar-start` - Left section with left-aligned content
//! - `.navbar-center` - Center section with centered content
//! - `.navbar-end` - Right section with right-aligned content

mod component;

pub use component::*;
