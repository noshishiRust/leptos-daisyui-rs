use super::style::{FileInputColor, FileInputSize, FileInputStyle};
use crate::merge_classes;
use leptos::{html::Input, prelude::*};

/// # File Input Component
///
/// A reactive Leptos wrapper for daisyUI's file input component that provides a styled
/// file upload interface with customizable appearance and behavior.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("file-input file-input-xs file-input-sm file-input-md file-input-lg file-input-xl file-input-ghost file-input-neutral file-input-primary file-input-secondary file-input-accent file-input-info file-input-success file-input-warning file-input-error");
/// ```
///
/// ## Node References
/// - `node_ref` - References the input element ([HTMLInputElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement))
#[component]
pub fn FileInput(
    /// Visual style of the file input
    #[prop(optional, into)]
    style: Signal<FileInputStyle>,

    /// Color scheme of the file input
    #[prop(optional, into)]
    color: Signal<FileInputColor>,

    /// Size of the file input
    #[prop(optional, into)]
    size: Signal<FileInputSize>,

    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference to the input element
    #[prop(optional)]
    node_ref: NodeRef<Input>,
) -> impl IntoView {
    view! {
        <input
            node_ref=node_ref
            type="file"
            class=move || {
                merge_classes!(
                    "file-input",
                    style.get().as_str(),
                    color.get().as_str(),
                    size.get().as_str(),
                    class
                )
            }
        />
    }
}
