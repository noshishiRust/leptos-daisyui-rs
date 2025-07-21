/// # Tabs Component
///
/// A reactive Leptos wrapper for daisyUI's tabs component that provides
/// navigation controls for organizing content into switchable panels.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("tabs tab tab-active tab-disabled tabs-box tabs-border tabs-lift tabs-top tabs-bottom");
/// ```
///
/// ## Node References
/// - `node_ref` - References the div element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))

use super::style::{TabSize, TabVariant};
use crate::merge_classes;
use leptos::{
    html::{A, Div, Input},
    prelude::*,
};
#[component]
pub fn Tabs(
    /// Size variant for tab dimensions
    #[prop(optional, into)]
    size: Signal<TabSize>,
    /// Visual style variant
    #[prop(optional, into)]
    variant: Signal<TabVariant>,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Node reference to the div element
    #[prop(optional)]
    node_ref: NodeRef<Div>,
    /// Tab content
    children: Children,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            class=move || {
                merge_classes!(
                    "tabs",
                size.get().as_str(),
                variant.get().as_str(),
                class
                )
            }
        >
            {children()}
        </div>
    }
}

/// # Tab Component
///
/// A reactive Leptos wrapper for individual tab items with click handling
/// and active state.
///
/// ## Node References
/// - `node_ref` - References the anchor element ([HTMLAnchorElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAnchorElement))
#[component]
pub fn Tab(
    /// Whether this tab is currently active
    #[prop(optional, into)]
    active: Signal<bool>,
    /// Whether this tab is disabled
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Node reference to the anchor element
    #[prop(optional)]
    node_ref: NodeRef<A>,
    /// Optional click handler
    #[prop(optional)]
    on_click: Option<Box<dyn Fn()>>,
    /// Tab label content
    children: Children,
) -> impl IntoView {
    view! {
        <a
            node_ref=node_ref
            class=move || merge_classes!("tab", class)
            class:tab-active=active
            class:tab-disabled=disabled
            on:click=move |_| {
                if let Some(handler) = &on_click {
                    handler();
                }
            }
        >
            {children()}
        </a>
    }
}

/// # Tab Radio Component
///
/// A reactive Leptos wrapper for radio input-based tabs providing form
/// integration and accessibility.
///
/// ## Node References
/// - `node_ref` - References the input element ([HTMLInputElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement))
#[component]
pub fn TabRadio(
    /// Radio group name for mutual exclusion
    #[prop(optional)]
    name: Option<&'static str>,
    /// Whether this radio tab is selected
    #[prop(optional, into)]
    checked: Signal<bool>,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Node reference to the input element
    #[prop(optional)]
    node_ref: NodeRef<Input>,
    /// Optional change handler
    #[prop(optional)]
    on_change: Option<Box<dyn Fn(bool)>>,
    /// Tab label content
    children: Children,
) -> impl IntoView {
    view! {
        <input
            node_ref=node_ref
            type="radio"
            name=name
            class=move || merge_classes!("tab", class)
            prop:checked=checked
            on:change=move |ev| {
                if let Some(handler) = &on_change {
                    handler(event_target_checked(&ev));
                }
            }
        />
        {children()}
    }
}
