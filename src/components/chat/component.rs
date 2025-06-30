use super::style::{ChatBubbleColor, ChatPlacement};
use crate::merge_classes;
use leptos::prelude::*;

#[component]
pub fn Chat(
    placement: Signal<ChatPlacement>,
    #[prop(optional, into)] class: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=merge_classes!(
            "chat",
                placement.get().as_str(),
                class
        )>{children()}</div>
    }
}

#[component]
pub fn ChatImage(#[prop(optional, into)] class: &'static str, children: Children) -> impl IntoView {
    view! { <div class=merge_classes!("chat-image", class)>{children()}</div> }
}

#[component]
pub fn ChatHeader(
    #[prop(optional, into)] class: &'static str,
    children: Children,
) -> impl IntoView {
    view! { <div class=merge_classes!("chat-header", class)>{children()}</div> }
}

#[component]
pub fn ChatBubble(
    #[prop(optional, into)] color: Signal<ChatBubbleColor>,
    #[prop(optional, into)] class: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=merge_classes!(
            "chat-bubble",
                color.get().as_str(),
                class
        )>{children()}</div>
    }
}

#[component]
pub fn ChatFooter(
    #[prop(optional, into)] class: &'static str,
    children: Children,
) -> impl IntoView {
    view! { <div class=merge_classes!("chat-footer", class)>{children()}</div> }
}
