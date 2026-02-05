use super::style::{ChatBubbleColor, ChatPlacement};
use crate::merge_classes;
use leptos::html::Div;
use leptos::prelude::*;

/// # Chat Component
///
/// A container for chat messages that displays conversation elements with proper
/// placement and styling. Supports left (start) and right (end) alignment.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("chat chat-start chat-end chat-image chat-header chat-bubble chat-footer chat-bubble-neutral chat-bubble-primary chat-bubble-secondary chat-bubble-accent chat-bubble-info chat-bubble-success chat-bubble-warning chat-bubble-error");
/// ```
///
/// ## Node References
/// - `node_ref` - References the top `<div>` element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn Chat(
    /// Placement of the chat bubble (start for left, end for right)
    #[prop(optional, into)]
    placement: Signal<ChatPlacement>,

    /// Additional CSS classes to apply to the chat container
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the chat container element
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// Child components: [`ChatImage`], [`ChatHeader`], [`ChatBubble`], [`ChatFooter`]
    children: Children,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            class=move || {
                merge_classes!(
                    "chat",
                    placement.get().as_str(),
                    class
                )
            }
        >
            {children()}
        </div>
    }
}

/// # Chat Image Component
///
/// Container for user avatar or profile image in a chat message.
///
/// ## Node References
/// - `node_ref` - References the top `<div>` element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn ChatImage(
    /// Additional CSS classes to apply to the image container
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the chat image container element
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// Avatar or image content
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("chat-image", class)>
            {children()}
        </div>
    }
}

/// # Chat Header Component
///
/// Header section for chat metadata like username, timestamp, or status.
///
/// ## Node References
/// - `node_ref` - References the top `<div>` element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn ChatHeader(
    /// Additional CSS classes to apply to the header
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the chat header container element
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// Header content (username, time, etc.)
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("chat-header", class)>
            {children()}
        </div>
    }
}

/// # Chat Bubble Component
///
/// The main message bubble containing the chat text or content.
/// Supports various color themes for different message types.
///
/// ## Node References
/// - `node_ref` - References the top `<div>` element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn ChatBubble(
    /// Color theme for the message bubble
    #[prop(optional, into)]
    color: Signal<ChatBubbleColor>,

    /// Additional CSS classes to apply to the bubble
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the chat bubble container element
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// Message content
    children: Children,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            class=move || {
                merge_classes!(
                    "chat-bubble",
                    color.get().as_str(),
                    class
                )
            }
        >
            {children()}
        </div>
    }
}

/// # Chat Footer Component
///
/// Footer section for additional message information like delivery status,
/// reactions, or timestamps.
///
/// ## Node References
/// - `node_ref` - References the top `<div>` element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn ChatFooter(
    /// Additional CSS classes to apply to the footer
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the chat footer container element
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// Footer content (status, time, reactions, etc.)
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("chat-footer", class)>
            {children()}
        </div>
    }
}
