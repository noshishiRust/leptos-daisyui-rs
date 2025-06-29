use super::style::{DividerColor, DividerDirection, DividerPlacement};
use crate::merge_classes;
use leptos::{html::Div, prelude::*};

#[component]
pub fn Divider(
    #[prop(optional, into)] color: Signal<DividerColor>,
    #[prop(optional, into)] direction: Signal<DividerDirection>,
    #[prop(optional, into)] placement: Signal<DividerPlacement>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            class=merge_classes!(
                "divider",
                color.get().as_str(),
                direction.get().as_str(),
                placement.get().as_str(),
                class
            )
        >
            {children.map(|c| c())}
        </div>
    }
}