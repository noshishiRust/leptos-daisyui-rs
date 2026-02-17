use crate::merge_classes;
use leptos::{
    html::{Div, Nav},
    prelude::*,
};

/// # Navbar Component
///
/// A reactive Leptos wrapper for daisyUI's navbar component that provides
/// a responsive navigation bar with flexible layout sections.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("navbar navbar-start navbar-center navbar-end");
/// ```
///
/// ## Node References
/// - `node_ref` - References the nav element ([HTMLElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement))
#[component]
pub fn Navbar(
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

/// Left-aligned section of the navbar for logos, brand names, and primary navigation.
#[component]
pub fn NavbarStart(
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

/// Center-aligned section of the navbar for search bars and main navigation menus.
#[component]
pub fn NavbarCenter(
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

/// Right-aligned section of the navbar for user actions, authentication buttons, and settings.
#[component]
pub fn NavbarEnd(
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
