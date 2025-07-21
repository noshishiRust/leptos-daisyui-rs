/// # Radial Progress Component
///
/// A reactive Leptos wrapper for daisyUI's radial progress component that provides
/// circular progress indicators with customizable colors, sizes, and values.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("radial-progress text-primary text-secondary text-accent text-success text-info text-warning text-error");
/// ```
///
/// ## Node References
/// - `node_ref` - References the div element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))

use super::style::{RadialProgressColor, RadialProgressSize};
use crate::merge_classes;
use leptos::{html::Div, prelude::*};

#[component]
pub fn RadialProgress(
    /// Color scheme of the radial progress
    #[prop(optional, into)] color: Signal<RadialProgressColor>,
    /// Size of the radial progress indicator
    #[prop(optional, into)] size: Signal<RadialProgressSize>,
    /// Progress value (0-100)
    #[prop(optional, into)] value: Signal<f64>,
    /// Thickness of the progress ring
    #[prop(optional, into)] thickness: Signal<String>,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Node reference to the div element
    #[prop(optional)] node_ref: NodeRef<Div>,
    /// Optional child elements displayed in the center
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let progress_style =
        move || format!("--value:{}; --thickness:{};", value.get(), thickness.get());

    view! {
        <div
            node_ref=node_ref
            class=move || {
                merge_classes!(
                    "radial-progress",
                    color.get().as_str(),
                    size.get().as_str(),
                    class
                )
            }
            style=progress_style
        >
            {children.map(|v| v())}
        </div>
    }
}
