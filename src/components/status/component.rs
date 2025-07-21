/// # Status Component
///
/// A reactive Leptos wrapper for daisyUI's status component that provides small
/// status indicators with customizable colors and sizes.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("status status-neutral status-primary status-secondary status-accent status-info status-success status-warning status-error status-xs status-sm status-md status-lg status-xl");
/// ```
///
/// ## Node References
/// - `node_ref` - References the span element ([HTMLSpanElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSpanElement))

use super::style::{StatusColor, StatusSize};
use crate::merge_classes;
use leptos::{html::Span, prelude::*};

#[component]
pub fn Status(
    /// Color scheme of the status indicator
    #[prop(optional, into)] color: Signal<StatusColor>,
    /// Size of the status indicator
    #[prop(optional, into)] size: Signal<StatusSize>,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Node reference to the span element
    #[prop(optional)] node_ref: NodeRef<Span>,
) -> impl IntoView {
    view! {
        <span
            node_ref=node_ref
            class=move || {
                merge_classes!(
                    "status",
                color.get().as_str(),
                size.get().as_str(),
                class
                )
            }
        ></span>
    }
}
