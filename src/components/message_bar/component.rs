use super::style::MessageBarType;
use leptos::{html, prelude::*};

/// A message bar component for displaying notifications and alerts.
///
/// # Example
/// ```rust,ignore
/// use leptos::prelude::*;
/// use leptos_daisyui_rs::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     let (show_message, set_show_message) = signal(true);
///
///     view! {
///         <Show when=move || show_message.get()>
///             <MessageBar
///                 message_type=MessageBarType::Success
///                 on_close=move || set_show_message.set(false)
///             >
///                 "Operation completed successfully!"
///             </MessageBar>
///         </Show>
///     }
/// }
/// ```
///
/// # CSS Classes
/// Add to your `input.css`:
/// ```css
/// @source inline("flex items-center gap-3 p-4 rounded-lg");
/// @source inline("bg-info bg-success bg-warning bg-error");
/// @source inline("text-info-content text-success-content text-warning-content text-error-content");
/// @source inline("btn btn-sm btn-circle btn-ghost");
/// ```
#[component]
pub fn MessageBar(
    /// Type of message
    #[prop(optional, into)]
    message_type: Signal<MessageBarType>,
    /// Whether to show a close button
    #[prop(optional, into)]
    closable: Signal<bool>,
    /// Callback when close button is clicked
    #[prop(optional)]
    on_close: Option<Callback<()>>,
    /// Message content
    children: Children,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Reference to the underlying DOM node
    #[prop(optional)]
    node_ref: NodeRef<html::Div>,
) -> impl IntoView {
    let computed_class = move || {
        let mut classes = vec![
            "flex",
            "items-center",
            "gap-3",
            "p-4",
            "rounded-lg",
            message_type.get().bg_class(),
            message_type.get().text_class(),
        ];
        if !class.is_empty() {
            classes.push(class);
        }
        classes.join(" ")
    };

    view! {
        <div
            node_ref=node_ref
            class=computed_class
        >
            <span class="text-lg font-bold">
                {move || message_type.get().icon()}
            </span>
            <div class="flex-1">
                {children()}
            </div>
            <Show when=move || closable.get()>
                <button
                    class="btn btn-sm btn-circle btn-ghost"
                    on:click=move |_| {
                        if let Some(callback) = on_close {
                            callback.run(());
                        }
                    }
                >
                    "✕"
                </button>
            </Show>
        </div>
    }
}
