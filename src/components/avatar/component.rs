use super::style::AvatarModifier;
use crate::merge_classes;
use leptos::{html::Div, prelude::*};

#[component]
pub fn Avatar(
    #[prop(optional, into)] modifier: Signal<AvatarModifier>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            aria-label="avatar"
            node_ref=node_ref
            class=merge_classes!(
                "avatar",
                modifier.get().as_str(),
                class
            )
        >
            <div>{children()}</div>
        </div>
    }
}

#[component]
pub fn AvatarGroup(
    #[prop(optional, into)] class: &'static str,
    children: Children,
) -> impl IntoView {
    view! { <div class=merge_classes!("avatar-group", class)>{children()}</div> }
}
