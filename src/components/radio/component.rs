use super::style::{RadioColor, RadioSize};
use crate::merge_classes;
use leptos::{html::Input, prelude::*};

#[component]
pub fn Radio(
    /// The color variant of the radio button
    #[prop(optional, into)]
    color: Signal<RadioColor>,

    /// The size variant of the radio button
    #[prop(optional, into)]
    size: Signal<RadioSize>,

    /// Additional CSS classes to apply
    #[prop(optional, into)]
    class: &'static str,

    /// Reference to the underlying HTML input element
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
