use super::style::{AlertColor, AlertDirection, AlertStyle};
use crate::merge_classes;
use leptos::{html::Div, prelude::*};

/// # Alert Component
///
/// A reactive Leptos wrapper for daisyUI's alert component that displays important messages,
/// notifications, and contextual feedback to users.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("alert alert-outline alert-dash alert-soft alert-info alert-success alert-warning alert-error alert-vertical alert-horizontal");
/// ```
///
/// ## Node References
/// - `node_ref` - References the top `<div>` element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn Alert(
    /// Visual style of the alert
    #[prop(optional, into)]
    style: Signal<AlertStyle>,

    /// Semantic color of the alert
    #[prop(optional, into)]
    color: Signal<AlertColor>,

    /// Layout direction of alert content
    #[prop(optional, into)]
    direction: Signal<AlertDirection>,

    /// Node reference for the alert element
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Alert content (text, icons, buttons, or other elements)
    children: Children,
) -> impl IntoView {
    view! {
        <div
            role="alert"
            node_ref=node_ref
            class=move || {
                merge_classes!(
                    "alert",
                    style.get().as_str(),
                    color.get().as_str(),
                    direction.get().as_str(),
                    class
                )
            }
        >
            {children()}
        </div>
    }
}
