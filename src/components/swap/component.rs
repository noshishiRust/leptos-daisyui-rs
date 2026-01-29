use super::style::SwapRotate;
use crate::merge_classes;
use leptos::{
    html::{Div, Label},
    prelude::*,
};

/// # Swap Component
///
/// Toggles between two elements using a checkbox. When active, shows the "on" content;
/// when inactive, shows the "off" content.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("swap swap-on swap-off swap-indeterminate swap-active swap-rotate swap-flip");
/// ```
///
/// ## Node References
/// - `node_ref` - References the swap `<label>` element ([HTMLLabelElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLabelElement))
#[component]
pub fn Swap(
    /// Animation style for the swap transition
    #[prop(optional, into)]
    rotate: Signal<SwapRotate>,

    /// Whether the swap is currently active (showing "on" content)
    #[prop(optional, into)]
    active: Signal<bool>,

    /// Whether the swap is currently indeterminate (showing "indeterminate" content)
    #[prop(optional, into)]
    indeterminate: Signal<bool>,

    /// Additional CSS classes to apply to the swap container
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the swap `<label>` element
    #[prop(optional)]
    node_ref: NodeRef<Label>,

    /// Child components: [`SwapOn`] and [`SwapOff`]
    children: Children,
) -> impl IntoView {
    view! {
        <label
            node_ref=node_ref
            class=move || {
                merge_classes!("swap",
                rotate.get().as_str(),
                class)
            }
            class:swap-active=move || active.get()
        >
            <input
                type="checkbox"
                prop:indeterminate=indeterminate
                style="display: none;"
            />
            {children()}
        </label>
    }
}

/// # Swap On Component
///
/// Content displayed when the swap is in the active/on state.
///
/// ## Node References
/// - `node_ref` - References the on-state `<div>` element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn SwapOn(
    /// Additional CSS classes to apply to the on-state content
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the on-state `<div>` element
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// Content to show when swap is active
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("swap-on", class)>
            {children()}
        </div>
    }
}

/// # Swap Off Component
///
/// Content displayed when the swap is in the inactive/off state.
///
/// ## Node References
/// - `node_ref` - References the off-state `<div>` element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn SwapOff(
    /// Additional CSS classes to apply to the off-state content
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the off-state `<div>` element
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// Content to show when swap is inactive
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("swap-off", class)>
            {children()}
        </div>
    }
}

/// # Swap Indeterminate Component
///
/// The child element that should be visible when checkbox is indeterminate
///
/// ## Node References
/// - `node_ref` - References the top `<div>` element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn SwapIndeterminate(
    /// Additional CSS classes to apply to the indeterminate-state content
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the indeterminate-state `<div>` element
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// Content to show when swap is indeterminate
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("swap-indeterminate", class)>
            {children()}
        </div>
    }
}
