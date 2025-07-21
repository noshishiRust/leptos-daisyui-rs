use super::style::{InputColor, InputSize, InputStyle};
use crate::merge_classes;
use leptos::{html::Input as HtmlInput, prelude::*};

/// # Input Component
///
/// A reactive Leptos wrapper for daisyUI's input component that provides a comprehensive
/// set of styling options for text input fields and form elements.
///
/// ## CSS Classes Used
///
/// ### Add to `input.css`
/// ```css
/// @source inline("input input-neutral input-primary input-secondary input-accent input-info input-success input-warning input-error input-ghost input-xs input-sm input-md input-lg input-xl input-disabled");
/// ```
///
/// ## Node References
/// - `node_ref` - References the `<input>` element ([HTMLInputElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement))
#[component]
pub fn Input(
    /// Input style variant
    #[prop(optional, into)]
    style: Signal<InputStyle>,

    /// Input color variant
    #[prop(optional, into)]
    color: Signal<InputColor>,

    /// Input size variant
    #[prop(optional, into)]
    size: Signal<InputSize>,

    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the input element

    #[prop(optional)]
    node_ref: NodeRef<HtmlInput>,
) -> impl IntoView {
    view! {
        <input
            node_ref=node_ref
            class=move || {
                merge_classes!(
                    "input",
                    style.get().as_str(),
                    color.get().as_str(),
                    size.get().as_str(),
                    class
                )
            }
        />
    }
}
