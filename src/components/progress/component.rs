use super::style::ProgressColor;
use crate::merge_classes;
use leptos::{html::Progress as HtmlProgress, prelude::*};

#[component]
pub fn Progress(
    #[prop(optional, into)] color: Signal<ProgressColor>,
    #[prop(optional, into)] value: Signal<f64>,
    #[prop(optional, into)] max: Signal<f64>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<HtmlProgress>,
) -> impl IntoView {
    view! {
        <progress
            node_ref=node_ref
            class=merge_classes!(
                "progress",
                color.get().as_str(),
                class
            )
            value=value
            max=max
        ></progress>
    }
}