use super::style::{RadioColor, RadioSize};
use crate::merge_classes;
use leptos::{html::Input, prelude::*};

/// # Radio Component
///
/// A reactive Leptos wrapper for daisyUI's radio component that provides radio button inputs
/// for single selection from a group of options.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("radio radio-primary radio-secondary radio-accent radio-success radio-warning radio-info radio-error radio-xs radio-sm radio-md radio-lg radio-xl");
/// ```
///
/// ## Node References
/// - `node_ref` - References the input element ([HTMLInputElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement))
#[component]
pub fn Radio(
    /// Color scheme of the radio button
    #[prop(optional, into)]
    color: Signal<RadioColor>,

    /// Size of the radio button
    #[prop(optional, into)]
    size: Signal<RadioSize>,

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
            type="radio"
            class=move || {
                merge_classes!(
                    "radio",
                    color.get().as_str(),
                    size.get().as_str(),
                    class
                )
            }
        />
    }
}
