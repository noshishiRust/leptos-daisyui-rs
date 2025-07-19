use super::style::ToastPosition;
use crate::merge_classes;
use leptos::{html::Div, prelude::*};

/// A Toast component that displays notifications or messages.
///
/// This component is a wrapper `<div>` element,
/// you can spread [HTMLDivElement](https://developer.mozilla.org/ja/docs/Web/API/HTMLDivElement) attributes to it.
#[component]
pub fn Toast(
    #[prop(optional, into)] position: Signal<ToastPosition>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            class=move || {
                merge_classes!(
                    "toast",
                    position.get().as_str(),
                    class
                )
            }
        >
            {children()}
        </div>
    }
}
