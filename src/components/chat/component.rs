use super::style::{ChatBubbleColor, ChatPlacement};
use crate::merge_classes;
use leptos::prelude::*;

#[component]
pub fn Chat(
    placement: Signal<ChatPlacement>,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=move || {
            merge_classes!("chat",
                placement.get().as_str(),
                class)
        }>{children()}</div>
    }
}

#[component]
pub fn ChatImage(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    children: Children,
) -> impl IntoView {
    view! { <div class=move || merge_classes!("chat-image", class)>{children()}</div> }
}

#[component]
pub fn ChatHeader(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    children: Children,
) -> impl IntoView {
    view! { <div class=move || merge_classes!("chat-header", class)>{children()}</div> }
}

#[component]
pub fn ChatBubble(
    #[prop(optional, into)] color: Signal<ChatBubbleColor>,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=move || {
            merge_classes!(
                "chat-bubble",
                color.get().as_str(),
                class
            )
        }>{children()}</div>
    }
}

#[component]
pub fn ChatFooter(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    children: Children,
) -> impl IntoView {
    view! { <div class=move || merge_classes!("chat-footer", class)>{children()}</div> }
}
