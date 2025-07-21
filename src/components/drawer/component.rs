use super::style::DrawerPlacement;
use crate::merge_classes;
use leptos::{
    html::{Div, Input},
    prelude::*,
};

/// # Drawer Component
///
/// A grid layout that can show/hide a sidebar on the left or right side of the page.
/// The drawer provides a slide-out navigation or content panel.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("drawer drawer-toggle drawer-content drawer-side drawer-overlay drawer-end drawer-open");
/// ```
///
/// ## Node References
/// - `node_ref` - References the drawer `<div>` element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn Drawer(
    /// Placement of the drawer (left by default, or right with DrawerPlacement::End)
    #[prop(optional, into)] 
    placement: Signal<DrawerPlacement>,

    /// Whether the drawer is open
    #[prop(optional, into)] 
    open: Signal<bool>,

    /// Additional CSS classes to apply to the drawer container
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the drawer `<div>` element
    #[prop(optional)] 
    node_ref: NodeRef<Div>,

    /// Child components: [`DrawerToggle`], [`DrawerContent`], [`DrawerSide`]
    children: Children,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            class=move || {
                merge_classes!(
                    "drawer",
                    placement.get().as_str(),
                    class
                )
            }
            class:drawer-open=open
        >
            {children()}
        </div>
    }
}

/// # Drawer Toggle Component
///
/// A hidden checkbox that controls the drawer open/close state. Use labels
/// with matching `for` attributes to toggle the drawer.
///
/// ## Node References
/// - `node_ref` - References the toggle `<input>` element ([HTMLInputElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement))
#[component]
pub fn DrawerToggle(
    /// Unique ID for the checkbox, used by labels to control drawer state
    id: &'static str,

    /// Whether the drawer is currently open
    #[prop(optional, into)] 
    checked: Signal<bool>,

    /// Node reference for the toggle `<input>` element
    #[prop(optional)] 
    node_ref: NodeRef<Input>,
) -> impl IntoView {
    view! {
        <input node_ref=node_ref id=id type="checkbox" class="drawer-toggle" prop:checked=checked />
    }
}

/// # Drawer Content Component
///
/// The main content area of the page when using a drawer layout.
/// All page content including navbar and footer should be inside this component.
///
/// ## Node References
/// - `node_ref` - References the content `<div>` element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn DrawerContent(
    /// Additional CSS classes to apply to the content area
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the content `<div>` element
    #[prop(optional)] 
    node_ref: NodeRef<Div>,

    /// Main page content (navbar, pages, footer, etc.)
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("drawer-content", class)>
            {children()}
        </div>
    }
}

/// # Drawer Side Component
///
/// The sidebar content area that slides in/out. Typically contains
/// navigation menus or additional content panels.
///
/// ## Node References
/// - `node_ref` - References the sidebar `<div>` element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn DrawerSide(
    /// Additional CSS classes to apply to the sidebar
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the sidebar `<div>` element
    #[prop(optional)] 
    node_ref: NodeRef<Div>,

    /// Sidebar content (usually menu components)
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("drawer-side", class)>
            {children()}
        </div>
    }
}

/// # Drawer Overlay Component
///
/// An optional overlay that appears over the content when the drawer is open,
/// typically used for mobile layouts to darken the background.
///
/// ## Node References
/// - `node_ref` - References the overlay `<div>` element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn DrawerOverlay(
    /// Additional CSS classes to apply to the overlay
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the overlay `<div>` element
    #[prop(optional)] 
    node_ref: NodeRef<Div>,
) -> impl IntoView {
    view! { <div node_ref=node_ref class=move || merge_classes!("drawer-overlay", class)></div> }
}
