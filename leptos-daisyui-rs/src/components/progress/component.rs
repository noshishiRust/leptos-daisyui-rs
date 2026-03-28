use super::style::ProgressColor;
use crate::merge_classes;
use leptos::{html::Progress as HtmlProgress, prelude::*};

/// # Progress Component
///
/// A reactive Leptos wrapper for daisyUI's progress component that displays the completion
/// progress of tasks or operations with visual progress bars.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("progress progress-primary progress-secondary progress-accent progress-info progress-success progress-warning progress-error");
/// ```
///
/// ## Node References
/// - `node_ref` - References the top `<progress>` element ([HTMLProgressElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLProgressElement))
#[component]
pub fn Progress(
    /// Progress bar color variant
    #[prop(optional, into)]
    color: Signal<ProgressColor>,

    /// Node reference for the progress element
    #[prop(optional)]
    node_ref: NodeRef<HtmlProgress>,

    /// Additional CSS classes to apply
    #[prop(optional, into)]
    class: &'static str,
) -> impl IntoView {
    view! {
        <progress
            node_ref=node_ref
            class=move || {
                merge_classes!(
                    "progress",
                    color.get().as_str(),
                    class
                )
            }
        />
    }
}
