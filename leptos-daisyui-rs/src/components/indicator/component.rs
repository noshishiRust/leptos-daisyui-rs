use super::style::{IndicatorHorizontalPlacement, IndicatorVerticalPlacement};
use crate::merge_classes;
use leptos::{
    html::{Div, Span},
    prelude::*,
};

/// # Indicator Component
///
/// A reactive Leptos wrapper for daisyUI's indicator component that provides a container
/// for positioning indicator items like badges, notifications, or status markers.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("indicator indicator-item indicator-top indicator-middle indicator-bottom indicator-start indicator-center indicator-end");
/// ```
///
/// ## Node References
/// - `node_ref` - References the container div element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn Indicator(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference to the container element
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// Child elements to be contained within the indicator
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("indicator", class)>
            {children()}
        </div>
    }
}

/// # Indicator Item Component
///
/// A reactive Leptos wrapper for daisyUI's indicator item component that represents
/// individual indicator elements positioned within an Indicator container.
///
/// ## Node References
/// - `node_ref` - References the span element ([HTMLSpanElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSpanElement))
#[component]
pub fn IndicatorItem(
    /// Vertical placement of the indicator item
    #[prop(optional, into)]
    vertical: Signal<IndicatorVerticalPlacement>,

    /// Horizontal placement of the indicator item
    #[prop(optional, into)]
    horizontal: Signal<IndicatorHorizontalPlacement>,

    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference to the span element
    #[prop(optional)]
    node_ref: NodeRef<Span>,

    /// Child elements of the indicator item
    children: Children,
) -> impl IntoView {
    view! {
        <span
            node_ref=node_ref
            class=move || {
                merge_classes!(
                    "indicator-item",
                    vertical.get().as_str(),
                    horizontal.get().as_str(),
                    class
                )
            }
        >
            {children()}
        </span>
    }
}
