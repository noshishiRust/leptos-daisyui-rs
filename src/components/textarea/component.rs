use super::style::{TextareaColor, TextareaSize};
use crate::merge_classes;
use leptos::{html::Textarea as HtmlTextarea, prelude::*};

/// # Textarea Component
///
/// A multi-line text input component for entering longer text content.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("textarea textarea-ghost textarea-primary textarea-secondary textarea-accent textarea-info textarea-success textarea-warning textarea-error textarea-xs textarea-sm textarea-md textarea-lg textarea-xl"
///
/// ```
///
/// ## Node References
/// - `node_ref` - References the table element ([HTMLTextareaElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextareaElement))
#[component]
pub fn Textarea(
    /// The color variant of the textarea
    #[prop(optional, into)]
    color: Signal<TextareaColor>,

    /// The size variant of the textarea
    #[prop(optional, into)]
    size: Signal<TextareaSize>,

    /// Additional CSS classes to apply
    #[prop(optional, into)]
    class: &'static str,

    /// Reference to the underlying HTML textarea element
    #[prop(optional)]
    node_ref: NodeRef<HtmlTextarea>,
) -> impl IntoView {
    view! {
        <textarea
            node_ref=node_ref
            class=move || {
                merge_classes!(
                    "textarea",
                color.get().as_str(),
                size.get().as_str(),
                class
                )
            }
        />
    }
}
