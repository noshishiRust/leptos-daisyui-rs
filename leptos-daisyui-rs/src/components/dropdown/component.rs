use super::style::*;
use crate::merge_classes;
use leptos::{
    html::{Details, Div, Summary, Ul},
    prelude::*,
};

/// # Dropdown Component
///
/// A dropdown container component using HTML `<div>` element.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("dropdown menu dropdown-start dropdown-center dropdown-end dropdown-top dropdown-bottom dropdown-left dropdown-right");
/// ```
///
/// ## Node References
/// - `node_ref` - References the top `<div>` element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn Dropdown(
    /// Dropdown alignment (start, center, end)
    #[prop(optional, into)]
    alignment: Signal<DropdownAlignment>,

    /// Dropdown placement (top, bottom, left, right)
    #[prop(optional, into)]
    placement: Signal<DropdownPlacement>,

    /// Enable hover-to-open behavior
    #[prop(optional, into)]
    hover: Signal<bool>,

    /// Force open state
    #[prop(optional, into)]
    open: Signal<bool>,

    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Reference to the div element
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// Dropdown content
    children: Children,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            tabindex="0"
            class=move || {
                merge_classes!(
                    "dropdown",
                    alignment.get().as_str(),
                    placement.get().as_str(),
                    class
                )
            }
            class:dropdown-hover=hover
            class:dropdown-open=open
        >
            {children()}
        </div>
    }
}

/// # DropdownDetails Component
///
/// A dropdown container component using HTML `<details>` element.
///
///
/// ## Node References
/// - `node_ref` - References the top `<details>` element ([HTMLDetailsElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDetailsElement))
#[component]
pub fn DropdownDetails(
    /// Dropdown alignment (start, center, end)
    #[prop(optional, into)]
    alignment: Signal<DropdownAlignment>,

    /// Dropdown placement (top, bottom, left, right)
    #[prop(optional, into)]
    placement: Signal<DropdownPlacement>,

    /// Enable hover-to-open behavior
    #[prop(optional, into)]
    hover: Signal<bool>,

    /// Force open state
    #[prop(optional, into)]
    open: Signal<bool>,

    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Reference to the details element
    #[prop(optional)]
    node_ref: NodeRef<Details>,

    /// Dropdown content
    children: Children,
) -> impl IntoView {
    view! {
        <details
            node_ref=node_ref
            class=move || {
                merge_classes!(
                    "dropdown",
                    alignment.get().as_str(),
                    placement.get().as_str(),
                    class
                )
            }
            class:dropdown-hover=hover
            class:dropdown-open=open
        >
            {children()}
        </details>
    }
}

/// # DropdownSummary Component
///
/// Creates the clickable element that toggles the dropdown state.
/// Uses the HTML `<summary>` element which provides native toggle behavior.
///
/// ## Node References
/// - `node_ref` - References the top `<details>` element ([summary](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/summary))
#[component]
pub fn DropdownSummary(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Reference to the summary element
    #[prop(optional)]
    node_ref: NodeRef<Summary>,

    /// Trigger content
    children: Children,
) -> impl IntoView {
    view! {
        <summary node_ref=node_ref class=class>
            {children()}
        </summary>
    }
}

/// # DropdownContent Component
///
///  Content container for dropdown menu items.
///
/// ## Node References
/// - `node_ref` - Rederences the top `<ul>` element ([HTMLUlElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLUlElement))
#[component]
pub fn DropdownContent(
    /// Whether this is a menu (adds specific styling)
    #[prop(optional, into)]
    is_menu: bool,

    /// Additional CSS classes (should include dropdown-content and styling)
    #[prop(optional, into)]
    class: &'static str,

    /// Reference to the ul element
    #[prop(optional)]
    node_ref: NodeRef<Ul>,

    /// Menu content
    children: Children,
) -> impl IntoView {
    let menu = if is_menu { "menu " } else { "" };

    view! {
        <ul node_ref=node_ref class=move || merge_classes!("dropdown-content", menu, class)>
            {children()}
        </ul>
    }
}
