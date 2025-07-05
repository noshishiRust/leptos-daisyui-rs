use super::style::{TextareaColor, TextareaSize};
use crate::merge_classes;
use leptos::{html::Textarea as HtmlTextarea, prelude::*};

#[component]
pub fn Textarea(
    #[prop(optional, into)] color: Signal<TextareaColor>,
    #[prop(optional, into)] size: Signal<TextareaSize>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<HtmlTextarea>,
) -> impl IntoView {
    view! {
        <textarea
            node_ref=node_ref
            class=move || {
                merge_classes!(
                    "textarea",
                color.get().as_str(),
                size.get().as_str(),
                class
                )
            }
        />
    }
}
