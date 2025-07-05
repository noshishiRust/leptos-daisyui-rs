use super::style::{AlertColor, AlertDirection, AlertStyle};
use crate::merge_classes;
use leptos::{html::Div, prelude::*};

/// Alert component that displays important messages or notifications.
///
/// Add `@source inline("alert alert-outline alert-dash alert-soft alert-info alert-success alert-warning alert-error alert-vertical alert-horizontal");`
/// to input.css
#[component]
pub fn Alert(
    #[prop(optional, into)] style: Signal<AlertStyle>,
    #[prop(optional, into)] color: Signal<AlertColor>,
    #[prop(optional, into)] direction: Signal<AlertDirection>,
    #[prop(optional)] node_ref: NodeRef<Div>,
    #[prop(optional, into)] class: &'static str,
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
