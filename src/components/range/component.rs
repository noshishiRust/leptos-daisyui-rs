use super::style::{RangeColor, RangeSize};
use crate::merge_classes;
use leptos::{html::Input, prelude::*};

#[component]
pub fn Range(
    /// The color variant of the range slider
    #[prop(optional, into)]
    color: Signal<RangeColor>,

    /// The size variant of the range slider
    #[prop(optional, into)]
    size: Signal<RangeSize>,

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
            type="range"
            class=move || {
                merge_classes!(
                    "range",
                    color.get().as_str(),
                    size.get().as_str(),
                    class
                )
            }
        />
    }
}
