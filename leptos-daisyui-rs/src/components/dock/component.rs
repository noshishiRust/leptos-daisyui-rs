use super::style::DockSize;
use crate::merge_classes;
use leptos::{
    html::{Button, Div, Span},
    prelude::*,
};

/// # Dock Component
///
/// A bottom navigation bar component that provides quick access to frequently used
/// actions or navigation options. Sticks to the bottom of the screen.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("dock dock-active dock-label dock-xs dock-sm dock-md dock-lg dock-xl");
/// ```
///
/// ## Node References
/// - `node_ref` - References the dock `<div>` element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn Dock(
    /// Size of the dock
    #[prop(optional, into)]
    size: Signal<DockSize>,

    /// Additional CSS classes to apply to the dock container
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the dock `<div>` element
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// Child [`DockItem`] components
    children: Children,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            class=move || {
                merge_classes!("dock",
                size.get().as_str(),
                class)
            }
        >
            {children()}
        </div>
    }
}

/// # Dock Item Component
///
/// An individual navigation button within the dock. Can be marked as active
/// and supports click handling.
///
/// ## Node References
/// - `node_ref` - References the item `<button>` element ([HTMLButtonElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement))
#[component]
pub fn DockItem(
    /// Whether this dock item is currently active/selected
    #[prop(optional, into)]
    active: Signal<bool>,

    /// Additional CSS classes to apply to the dock item
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the item `<button>` element
    #[prop(optional)]
    node_ref: NodeRef<Button>,

    /// Content including icons and [`DockLabel`]
    children: Children,
) -> impl IntoView {
    view! {
        <button node_ref=node_ref class=class class:dock-active=active>
            {children()}
        </button>
    }
}

/// # Dock Label Component
///
/// A text label for dock items, typically displayed below the icon.
///
/// ## Node References
/// - `node_ref` - References the label `<span>` element ([HTMLSpanElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSpanElement))
#[component]
pub fn DockLabel(
    /// Additional CSS classes to apply to the label
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the label `<span>` element
    #[prop(optional)]
    node_ref: NodeRef<Span>,

    /// Label text
    children: Children,
) -> impl IntoView {
    view! {
        <span node_ref=node_ref class=move || merge_classes!("dock-label", class)>
            {children()}
        </span>
    }
}
