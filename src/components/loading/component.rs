use super::style::{LoadingColor, LoadingSize, LoadingType};
use crate::merge_classes;
use leptos::{html::Span, prelude::*};

#[component]
pub fn Loading(
    #[prop(optional, into)] color: Signal<LoadingColor>,
    #[prop(optional, into)] loading_type: Signal<LoadingType>,
    #[prop(optional, into)] size: Signal<LoadingSize>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Span>,
) -> impl IntoView {
    view! {
        <span
            node_ref=node_ref
            class=merge_classes!(
                "loading",
                color.get().as_str(),
                loading_type.get().as_str(),
                size.get().as_str(),
                class
            )
        ></span>
    }
}
