use super::style::{TooltipColor, TooltipPosition};
use crate::merge_classes;
use leptos::{html::Div, prelude::*};

/// # Tooltip Component
///
/// A reactive Leptos wrapper for daisyUI's tooltip component that displays contextual
/// messages on hover. Supports positioning, colors, and both simple text and custom content.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("tooltip tooltip-top tooltip-bottom tooltip-left tooltip-right tooltip-neutral tooltip-primary tooltip-secondary tooltip-accent tooltip-info tooltip-success tooltip-warning tooltip-error tooltip-open");
/// ```
///
/// ## Node References
/// - `node_ref` - References the tooltip container `<div>` element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn Tooltip(
    /// Tooltip text content (simple string tooltip)
    #[prop(optional, into)]
    tip: Signal<String>,

    /// Position of the tooltip relative to the trigger element
    #[prop(optional, into)]
    position: Signal<TooltipPosition>,

    /// Color variant of the tooltip
    #[prop(optional, into)]
    color: Signal<TooltipColor>,

    /// Force tooltip to always be visible
    #[prop(optional, into)]
    open: Signal<bool>,

    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the tooltip container element
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// Trigger element and optional custom tooltip content
    children: Children,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            class=move || {
                merge_classes!(
                    "tooltip",
                    position.get().as_str(),
                    color.get().as_str(),
                    class
                )
            }
            class:tooltip-open=open
            data-tip=move || tip.get()
        >
            {children()}
        </div>
    }
}
