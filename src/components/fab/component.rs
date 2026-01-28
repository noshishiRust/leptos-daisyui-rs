use crate::merge_classes;
use leptos::{html::Div, prelude::*};

/// # FAB (Floating Action Button) Component
///
/// A reactive Leptos wrapper for daisyUI's FAB component that provides floating action buttons
/// with speed dial functionality. Supports vertical stacking or flower (quarter-circle) layout.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("fab fab-close fab-main-action fab-flower");
/// ```
///
/// ## Important Notes
/// - Uses a focusable `<div>` with `tabindex="0"` and `role="button"` for browser compatibility
/// - Safari has a CSS bug preventing `<button>` elements from being focused
/// - Speed dial buttons display on click/focus of the main trigger
///
/// ## Node References
/// - `node_ref` - References the FAB container `<div>` element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn Fab(
    /// Use flower layout (quarter-circle) instead of vertical stacking
    #[prop(optional, into)]
    flower: Signal<bool>,

    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the FAB container element
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// Trigger button and speed dial buttons (use FabMainAction, FabClose, and regular buttons)
    children: Children,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            class=move || {
                merge_classes!(
                    "fab",
                    class
                )
            }
            class:fab-flower=flower
            tabindex="0"
            role="button"
        >
            {children()}
        </div>
    }
}

/// # FAB Main Action Component
///
/// A main action button that displays when the FAB is expanded, replacing the default trigger.
/// This is an optional component that can be used instead of FabClose.
#[component]
pub fn FabMainAction(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Button content
    children: Children,
) -> impl IntoView {
    view! {
        <div class=move || merge_classes!("fab-main-action", class)>
            {children()}
        </div>
    }
}

/// # FAB Close Component
///
/// A close button that displays when the FAB is expanded, replacing the default trigger.
/// This is an optional component that can be used instead of FabMainAction.
#[component]
pub fn FabClose(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Button content
    children: Children,
) -> impl IntoView {
    view! {
        <div class=move || merge_classes!("fab-close", class)>
            {children()}
        </div>
    }
}
