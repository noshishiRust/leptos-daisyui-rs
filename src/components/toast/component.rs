use super::style::ToastPosition;
use crate::merge_classes;
use leptos::{html::Div, prelude::*};

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
            class=merge_classes!(
                "toast",
                position.get().as_str(),
                class
            )
        >
            {children()}
        </div>
    }
}

#[component]
pub fn ToastItem(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=merge_classes!("alert", class)>
            {children()}
        </div>
    }
}
