use super::style::ProgressColor;
use crate::merge_classes;
use leptos::{html::Progress as HtmlProgress, prelude::*};

/// A progress bar component that displays the progress of a task or operation.
///
/// This component is a wrapper `<progress>` element,
/// you can spread [HTMLProgressElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLProgressElement) attributes to it.
#[component]
pub fn Progress(
    #[prop(optional, into)] color: Signal<ProgressColor>,
    #[prop(optional, into)] value: MaybeProp<f64>,
    #[prop(optional, into)] max: Signal<f64>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<HtmlProgress>,
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
            value=move || value.get()
            max=max
        />
    }
}
