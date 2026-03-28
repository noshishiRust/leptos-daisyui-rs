use super::style::{DividerColor, DividerDirection, DividerPlacement};
use crate::merge_classes;
use leptos::{html::Div, prelude::*};

/// # Divider Component
///
/// A visual separator component for dividing content sections.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("divider divider-start divider-end divider-horizontal divider-vertical divider-neutral divider-primary divider-secondary divider-accent divider-success divider-warning divider-info divider-error checkbox-xs checkbox-sm checkbox-md checkbox-lg checkbox-xl");
/// ```
///
/// ## Node References
/// - `node_ref` - References the top `<div>` element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn Divider(
    /// Color variant for the divider styling
    #[prop(optional, into)]
    color: Signal<DividerColor>,

    /// Direction of the divider (horizontal or vertical)
    #[prop(optional, into)]
    direction: Signal<DividerDirection>,

    /// Text placement within the divider
    #[prop(optional, into)]
    placement: Signal<DividerPlacement>,

    /// Additional CSS classes to apply
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the div element
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// Optional text content to display within the divider
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            class=move || {
                merge_classes!(
                    "divider",
                    color.get().as_str(),
                    direction.get().as_str(),
                    placement.get().as_str(),
                    class
                )
            }
        >
            {children.map(|c| c())}
        </div>
    }
}
