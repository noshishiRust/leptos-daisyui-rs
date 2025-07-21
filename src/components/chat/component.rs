use super::style::{ChatBubbleColor, ChatPlacement};
use crate::merge_classes;
use leptos::prelude::*;

/// # Chat Component
///
/// A container for chat messages that displays conversation elements with proper
/// placement and styling. Supports left (start) and right (end) alignment.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("chat chat-start chat-end chat-image chat-header chat-bubble chat-footer");
/// ```
///
/// ## Node References
/// - No node_ref available - uses div element
#[component]
pub fn Chat(
    /// Placement of the chat bubble (start for left, end for right)
    #[prop(optional, into)]
    placement: Signal<ChatPlacement>,

    /// Additional CSS classes to apply to the chat container
    #[prop(optional, into)]
    class: &'static str,

    /// Child components: [`ChatImage`], [`ChatHeader`], [`ChatBubble`], [`ChatFooter`]
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

/// # Chat Image Component
///
/// Container for user avatar or profile image in a chat message.
///
/// ## Node References
/// - No node_ref available - uses div element
#[component]
pub fn ChatImage(
    /// Additional CSS classes to apply to the image container
    #[prop(optional, into)]
    class: &'static str,

    /// Avatar or image content
    children: Children,
) -> impl IntoView {
    view! { <div class=move || merge_classes!("chat-image", class)>{children()}</div> }
}

/// # Chat Header Component
///
/// Header section for chat metadata like username, timestamp, or status.
///
/// ## Node References
/// - No node_ref available - uses div element
#[component]
pub fn ChatHeader(
    /// Additional CSS classes to apply to the header
    #[prop(optional, into)]
    class: &'static str,

    /// Header content (username, time, etc.)
    children: Children,
) -> impl IntoView {
    view! { <div class=move || merge_classes!("chat-header", class)>{children()}</div> }
}

/// # Chat Bubble Component
///
/// The main message bubble containing the chat text or content.
/// Supports various color themes for different message types.
///
/// ## Node References
/// - No node_ref available - uses div element
#[component]
pub fn ChatBubble(
    /// Color theme for the message bubble
    #[prop(optional, into)]
    color: Signal<ChatBubbleColor>,

    /// Additional CSS classes to apply to the bubble
    #[prop(optional, into)]
    class: &'static str,

    /// Message content
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

/// # Chat Footer Component
///
/// Footer section for additional message information like delivery status,
/// reactions, or timestamps.
///
/// ## Node References
/// - No node_ref available - uses div element
#[component]
pub fn ChatFooter(
    /// Additional CSS classes to apply to the footer
    #[prop(optional, into)]
    class: &'static str,

    /// Footer content (status, time, reactions, etc.)
    children: Children,
) -> impl IntoView {
    view! { <div class=move || merge_classes!("chat-footer", class)>{children()}</div> }
}
