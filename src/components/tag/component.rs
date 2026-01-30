use super::style::{TagColor, TagSize};
use leptos::{html, prelude::*};

/// A tag component for labels and badges.
///
/// # Example
/// ```rust,ignore
/// use leptos::prelude::*;
/// use leptos_daisyui_rs::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <Tag color=TagColor::Primary size=TagSize::Medium>
///             "New"
///         </Tag>
///         <Tag color=TagColor::Success closable=true on_close=move || {}>
///             "Completed"
///         </Tag>
///     }
/// }
/// ```
///
/// # CSS Classes
/// Add to your `input.css`:
/// ```css
/// @source inline("badge badge-neutral badge-primary badge-secondary badge-accent badge-success badge-warning badge-error badge-info");
/// @source inline("badge-sm badge-md badge-lg badge-outline");
/// @source inline("btn btn-xs btn-circle btn-ghost ml-1");
/// ```
#[component]
pub fn Tag(
    /// Color of the tag
    #[prop(optional, into)]
    color: Signal<TagColor>,
    /// Size of the tag
    #[prop(optional, into)]
    size: Signal<TagSize>,
    /// Whether to show as outline style
    #[prop(optional, into)]
    outline: Signal<bool>,
    /// Whether the tag can be closed
    #[prop(optional, into)]
    closable: Signal<bool>,
    /// Callback when close button is clicked
    #[prop(optional)]
    on_close: Option<Callback<()>>,
    /// Tag content
    children: Children,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Reference to the underlying DOM node
    #[prop(optional)]
    node_ref: NodeRef<html::Span>,
) -> impl IntoView {
    let computed_class = move || {
        let mut classes = vec!["badge", color.get().as_str(), size.get().as_str()];
        if outline.get() {
            classes.push("badge-outline");
        }
        if !class.is_empty() {
            classes.push(class);
        }
        classes.join(" ")
    };

    view! {
        <span
            node_ref=node_ref
            class=computed_class
        >
            {children()}
            <Show when=move || closable.get()>
                <button
                    class="btn btn-xs btn-circle btn-ghost ml-1"
                    on:click=move |_| {
                        if let Some(callback) = on_close {
                            callback.run(());
                        }
                    }
                >
                    "✕"
                </button>
            </Show>
        </span>
    }
}
