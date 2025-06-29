use super::style::{StatusColor, StatusSize};
use crate::merge_classes;
use leptos::{html::Span, prelude::*};

#[component]
pub fn Status(
    #[prop(optional, into)] color: Signal<StatusColor>,
    #[prop(optional, into)] size: Signal<StatusSize>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Span>,
) -> impl IntoView {
    view! {
        <span
            node_ref=node_ref
            class=merge_classes!(
                "status",
                color.get().as_str(),
                size.get().as_str(),
                class
            )
        ></span>
    }
}
