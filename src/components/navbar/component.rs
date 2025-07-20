//! Navbar component implementations for application navigation.

use crate::merge_classes;
use leptos::{
    html::{Div, Nav},
    prelude::*,
};

/// A responsive navigation bar container.
///
/// Creates the main navigation container for application headers. Uses semantic `<nav>` element
/// and provides a flexible three-section layout system for organizing navigation content.
///
/// # Props
///
/// - `class` - Additional CSS classes to apply
/// - `node_ref` - Reference to the underlying `<nav>` element
/// - `children` - Navigation content (typically NavbarStart, NavbarCenter, NavbarEnd)
///
/// # Accessibility
///
/// - Uses semantic `<nav>` element for proper navigation landmark
/// - Inherits focus management from child components
/// - Screen reader friendly structure
///
/// # Examples
///
/// ## Basic Navbar
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::{Navbar, NavbarStart, NavbarEnd};
///
/// view! {
///     <Navbar>
///         <NavbarStart>
///             <a class="btn btn-ghost text-xl">"Brand"</a>
///         </NavbarStart>
///         <NavbarEnd>
///             <button class="btn">"Login"</button>
///         </NavbarEnd>
///     </Navbar>
/// }
/// ```
///
/// ## Full Layout
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::{Navbar, NavbarStart, NavbarCenter, NavbarEnd};
///
/// view! {
///     <Navbar class="bg-base-100 shadow-lg">
///         <NavbarStart>
///             <div class="dropdown">
///                 <label class="btn btn-ghost lg:hidden">"☰"</label>
///             </div>
///             <a class="btn btn-ghost text-xl">"MyApp"</a>
///         </NavbarStart>
///         
///         <NavbarCenter class="hidden lg:flex">
///             <ul class="menu menu-horizontal px-1">
///                 <li><a>"Home"</a></li>
///                 <li><a>"About"</a></li>
///                 <li><a>"Contact"</a></li>
///             </ul>
///         </NavbarCenter>
///         
///         <NavbarEnd>
///             <button class="btn btn-ghost btn-circle">
///                 <svg class="w-5 h-5">/* search icon */</svg>
///             </button>
///             <button class="btn btn-primary">"Get Started"</button>
///         </NavbarEnd>
///     </Navbar>
/// }
/// ```
///
/// ## Custom Styling
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::Navbar;
///
/// view! {
///     <Navbar class="bg-primary text-primary-content">
///         // Primary colored navbar with contrasting text
///     </Navbar>
/// }
/// ```
#[component]
pub fn Navbar(
    /// Additional CSS classes
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Reference to the nav element
    #[prop(optional)]
    node_ref: NodeRef<Nav>,
    /// Navigation content
    children: Children,
) -> impl IntoView {
    view! {
        <nav node_ref=node_ref class=move || merge_classes!("navbar", class)>
            {children()}
        </nav>
    }
}

/// Left-aligned section of the navbar.
///
/// Contains navigation elements that should be positioned on the left side of the navbar.
/// Commonly used for logos, brand names, menu toggles, and primary navigation items.
///
/// # Props
///
/// - `class` - Additional CSS classes to apply
/// - `node_ref` - Reference to the underlying `<div>` element
/// - `children` - Left-aligned content
///
/// # Layout Behavior
///
/// - Items are left-aligned and flex to the start of the navbar
/// - Takes up only as much space as needed
/// - Responsive: Can be hidden/shown based on screen size
///
/// # Examples
///
/// ## Logo and Brand
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::NavbarStart;
///
/// view! {
///     <NavbarStart>
///         <a class="btn btn-ghost text-xl" href="/">
///             <img src="logo.svg" alt="Logo" class="w-8 h-8"/>
///             "MyApp"
///         </a>
///     </NavbarStart>
/// }
/// ```
///
/// ## Mobile Menu Toggle
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::NavbarStart;
///
/// view! {
///     <NavbarStart>
///         <div class="dropdown">
///             <label class="btn btn-ghost lg:hidden">
///                 <svg class="w-5 h-5">/* hamburger icon */</svg>
///             </label>
///             <ul class="menu menu-sm dropdown-content">
///                 <li><a>"Home"</a></li>
///                 <li><a>"About"</a></li>
///             </ul>
///         </div>
///         <a class="btn btn-ghost text-xl">"Brand"</a>
///     </NavbarStart>
/// }
/// ```
///
/// ## Navigation Menu
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::NavbarStart;
///
/// view! {
///     <NavbarStart class="flex-1">
///         <a class="btn btn-ghost text-xl">"Logo"</a>
///         <ul class="menu menu-horizontal hidden lg:flex">
///             <li><a>"Products"</a></li>
///             <li><a>"Solutions"</a></li>
///             <li><a>"Resources"</a></li>
///         </ul>
///     </NavbarStart>
/// }
/// ```
#[component]
pub fn NavbarStart(
    /// Additional CSS classes
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Reference to the div element
    #[prop(optional)]
    node_ref: NodeRef<Div>,
    /// Left-aligned content
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("navbar-start", class)>
            {children()}
        </div>
    }
}

/// Center-aligned section of the navbar.
///
/// Contains navigation elements that should be centered within the navbar.
/// Ideal for search bars, main navigation menus, or prominent branding elements.
///
/// # Props
///
/// - `class` - Additional CSS classes to apply
/// - `node_ref` - Reference to the underlying `<div>` element
/// - `children` - Center-aligned content
///
/// # Layout Behavior
///
/// - Items are centered within the available navbar space
/// - Grows to fill available space between start and end sections
/// - May be hidden on smaller screens for responsive design
///
/// # Examples
///
/// ## Search Bar
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::NavbarCenter;
///
/// view! {
///     <NavbarCenter>
///         <div class="form-control">
///             <input
///                 type="text"
///                 placeholder="Search products..."
///                 class="input input-bordered w-24 md:w-auto"
///             />
///         </div>
///     </NavbarCenter>
/// }
/// ```
///
/// ## Main Navigation
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::NavbarCenter;
///
/// view! {
///     <NavbarCenter class="hidden lg:flex">
///         <ul class="menu menu-horizontal px-1">
///             <li><a class="btn btn-ghost">"Home"</a></li>
///             <li><a class="btn btn-ghost">"Products"</a></li>
///             <li><a class="btn btn-ghost">"About"</a></li>
///             <li><a class="btn btn-ghost">"Contact"</a></li>
///         </ul>
///     </NavbarCenter>
/// }
/// ```
///
/// ## Centered Branding
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::NavbarCenter;
///
/// view! {
///     <NavbarCenter>
///         <a class="btn btn-ghost text-2xl font-bold">
///             "BRAND"
///         </a>
///     </NavbarCenter>
/// }
/// ```
///
/// ## Responsive Design
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::NavbarCenter;
///
/// view! {
///     <NavbarCenter class="hidden md:flex">
///         // Only show on medium screens and up
///         <div class="join">
///             <input class="input input-bordered join-item" placeholder="Search"/>
///             <button class="btn join-item">"🔍"</button>
///         </div>
///     </NavbarCenter>
/// }
/// ```
#[component]
pub fn NavbarCenter(
    /// Additional CSS classes
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Reference to the div element
    #[prop(optional)]
    node_ref: NodeRef<Div>,
    /// Center-aligned content
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("navbar-center", class)>
            {children()}
        </div>
    }
}

/// Right-aligned section of the navbar.
///
/// Contains navigation elements that should be positioned on the right side of the navbar.
/// Typically used for user account controls, action buttons, settings, and secondary navigation.
///
/// # Props
///
/// - `class` - Additional CSS classes to apply
/// - `node_ref` - Reference to the underlying `<div>` element
/// - `children` - Right-aligned content
///
/// # Layout Behavior
///
/// - Items are right-aligned and flex to the end of the navbar
/// - Takes up only as much space as needed
/// - Maintains position regardless of center content
///
/// # Examples
///
/// ## User Actions
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::NavbarEnd;
///
/// view! {
///     <NavbarEnd>
///         <button class="btn btn-ghost btn-circle">
///             <svg class="w-5 h-5">/* notification icon */</svg>
///         </button>
///         <div class="dropdown dropdown-end">
///             <label class="btn btn-ghost btn-circle avatar">
///                 <div class="w-10 rounded-full">
///                     <img src="avatar.jpg" alt="User"/>
///                 </div>
///             </label>
///             <ul class="dropdown-content menu">
///                 <li><a>"Profile"</a></li>
///                 <li><a>"Settings"</a></li>
///                 <li><a>"Logout"</a></li>
///             </ul>
///         </div>
///     </NavbarEnd>
/// }
/// ```
///
/// ## Authentication Buttons
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::NavbarEnd;
///
/// view! {
///     <NavbarEnd>
///         <a class="btn btn-ghost" href="/login">"Login"</a>
///         <a class="btn btn-primary" href="/signup">"Sign Up"</a>
///     </NavbarEnd>
/// }
/// ```
///
/// ## Icon Actions
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::NavbarEnd;
///
/// view! {
///     <NavbarEnd>
///         <button class="btn btn-ghost btn-circle">
///             <svg class="w-5 h-5">/* search icon */</svg>
///         </button>
///         <button class="btn btn-ghost btn-circle">
///             <svg class="w-5 h-5">/* theme toggle icon */</svg>
///         </button>
///         <button class="btn btn-ghost btn-circle">
///             <svg class="w-5 h-5">/* settings icon */</svg>
///         </button>
///     </NavbarEnd>
/// }
/// ```
///
/// ## Shopping Cart
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::NavbarEnd;
///
/// view! {
///     <NavbarEnd>
///         <div class="form-control hidden md:block">
///             <input class="input input-bordered" placeholder="Search"/>
///         </div>
///         <button class="btn btn-ghost btn-circle">
///             <div class="indicator">
///                 <svg class="w-5 h-5">/* cart icon */</svg>
///                 <span class="badge badge-sm indicator-item">"3"</span>
///             </div>
///         </button>
///     </NavbarEnd>
/// }
/// ```
#[component]
pub fn NavbarEnd(
    /// Additional CSS classes
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Reference to the div element
    #[prop(optional)]
    node_ref: NodeRef<Div>,
    /// Right-aligned content
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("navbar-end", class)>
            {children()}
        </div>
    }
}
