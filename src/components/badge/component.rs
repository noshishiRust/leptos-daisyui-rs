use super::style::{BadgeColor, BadgeSize, BadgeStyle};
use crate::merge_classes;
use leptos::{html::Div, prelude::*};

#[component]
pub fn Badge(
    #[prop(optional, into)] style: Signal<BadgeStyle>,
    #[prop(optional, into)] color: Signal<BadgeColor>,
    #[prop(optional, into)] size: Signal<BadgeSize>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            aria-label="badge"
            node_ref=node_ref
            class=move || {
                merge_classes!(
                    "badge",
                style.get().as_str(),
                color.get().as_str(),
                size.get().as_str(),
                class
                )
            }
        >
            {children()}
        </div>
    }
}
