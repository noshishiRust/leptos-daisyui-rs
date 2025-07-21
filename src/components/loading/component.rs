use super::style::{LoadingColor, LoadingSize, LoadingType};
use crate::merge_classes;
use leptos::{html::Span, prelude::*};

/// # Loading Component
///
/// A reactive Leptos wrapper for daisyUI's loading component that provides animated
/// loading indicators with customizable colors, types, and sizes.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("loading loading-spinner loading-dots loading-ring loading-ball loading-bars loading-infinity loading-xs loading-sm loading-md loading-lg loading-neutral loading-primary loading-secondary loading-accent loading-success loading-info loading-warning loading-error");
/// ```
///
/// ## Node References
/// - `node_ref` - References the span element ([HTMLSpanElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSpanElement))
#[component]
pub fn Loading(
    /// Color scheme of the loading indicator
    #[prop(optional, into)]
    color: Signal<LoadingColor>,

    /// Type of loading animation
    #[prop(optional, into)]
    loading_type: Signal<LoadingType>,

    /// Size of the loading indicator
    #[prop(optional, into)]
    size: Signal<LoadingSize>,

    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference to the span element
    #[prop(optional)]
    node_ref: NodeRef<Span>,
) -> impl IntoView {
    view! {
        <span
            node_ref=node_ref
            class=move || {
                merge_classes!(
                    "loading",
                color.get().as_str(),
                loading_type.get().as_str(),
                size.get().as_str(),
                class
                )
            }
        ></span>
    }
}
