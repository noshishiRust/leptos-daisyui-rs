use super::style::*;
use crate::merge_classes;
use leptos::{
    html::{Details, Div, Summary, Ul},
    prelude::*,
};

/// A dropdown container component using HTML `<details>` element.
///
/// Provides a collapsible dropdown menu with configurable trigger and content areas.
/// Built on the native `<details>` element for accessibility and keyboard support.
///
/// # Props
///
/// - `hover` - Enable hover-to-open behavior (default: false)
/// - `open` - Force open state (overrides normal toggle behavior)
/// - `class` - Additional CSS classes to apply
/// - `node_ref` - Reference to the underlying `<details>` element
/// - `children` - Dropdown content (typically DropdownTrigger and DropdownContent)
///
/// # Behavior
///
/// - Click trigger to toggle open/closed state
/// - Optional hover-to-open with `hover=true`
/// - Automatic positioning and overflow handling
/// - Native keyboard support (Space/Enter to toggle)
/// - Closes when clicking outside (native behavior)
///
/// # Examples
///
/// ## Basic Dropdown
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::{Dropdown, DropdownTrigger, DropdownContent};
///
/// view! {
///     <Dropdown>
///         <DropdownTrigger>
///             <button class="btn">"Options"</button>
///         </DropdownTrigger>
///         <DropdownContent class="menu p-2 shadow bg-base-100 rounded-box w-52">
///             <li><a>"Edit"</a></li>
///             <li><a>"Delete"</a></li>
///             <li><a>"Share"</a></li>
///         </DropdownContent>
///     </Dropdown>
/// }
/// ```
///
/// ## Node Reference
///
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

/// Trigger element for dropdown activation.
///
/// Creates the clickable element that toggles the dropdown state.
/// Uses the HTML `<summary>` element which provides native toggle behavior.
///
/// # Props
///
/// - `class` - Additional CSS classes to apply
/// - `node_ref` - Reference to the underlying `<summary>` element
/// - `children` - Trigger content (button, text, icon, etc.)
///
/// # Behavior
///
/// - Click to toggle dropdown open/closed
/// - Keyboard accessible (Space/Enter)
/// - Focus management
/// - Screen reader support
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

/// Content container for dropdown menu items.
///
/// Provides the dropdown menu content area with proper positioning and styling.
/// Typically contains menu items, links, or other interactive elements.
///
/// # Props
///
/// - `class` - Additional CSS classes to apply
/// - `node_ref` - Reference to the underlying `<ul>` element
/// - `children` - Menu content (list items, links, etc.)
///
/// # Styling
///
/// - Automatically positioned relative to trigger
/// - Hidden by default, shown when dropdown is open
/// - z-index handling for proper layering
/// - Responsive positioning to avoid viewport overflow
///
/// # Examples
///
/// ## Simple Menu
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::DropdownContent;
///
/// view! {
///     <DropdownContent class="menu p-2 shadow bg-base-100 rounded-box w-52">
///         <li><a>"Item 1"</a></li>
///         <li><a>"Item 2"</a></li>
///         <li><a>"Item 3"</a></li>
///     </DropdownContent>
/// }
/// ```
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

/// A dropdown container component using HTML `<div>` element.
///
/// Provides a collapsible dropdown menu with configurable trigger and content areas.
/// Built on the native `<div>` element for accessibility and keyboard support.
///
/// # Behavior
///
/// - Click trigger to toggle open/closed state
/// - Optional hover-to-open with `hover=true`
/// - Automatic positioning and overflow handling
/// - Native keyboard support (Space/Enter to toggle)
/// - Closes when clicking outside (native behavior)
///
/// ## Node Reference
///
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
