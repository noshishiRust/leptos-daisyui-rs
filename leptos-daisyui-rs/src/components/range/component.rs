use super::style::{RangeColor, RangeSize};
use crate::merge_classes;
use leptos::{html::Input, prelude::*};

/// # Range Component
///
/// A reactive Leptos wrapper for daisyUI's range component that provides interactive
/// slider controls for selecting numeric values within a defined range.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("range range-primary range-secondary range-accent range-success range-warning range-info range-error range-xs range-sm range-md range-lg range-xl");
/// ```
///
/// ## Node References
/// - `node_ref` - References the input element ([HTMLInputElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement))
#[component]
pub fn Range(
    /// Color scheme of the range slider
    #[prop(optional, into)]
    color: Signal<RangeColor>,

    /// Size of the range slider
    #[prop(optional, into)]
    size: Signal<RangeSize>,

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
