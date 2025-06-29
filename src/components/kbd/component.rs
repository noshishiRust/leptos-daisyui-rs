use super::style::KbdSize;
use crate::merge_classes;
use leptos::{html::Kbd as HtmlKbd, prelude::*};

#[component]
pub fn Kbd(
    #[prop(optional, into)] size: Signal<KbdSize>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<HtmlKbd>,
    children: Children,
) -> impl IntoView {
    view! {
        <kbd
            node_ref=node_ref
            class=merge_classes!("kbd",
                size.get().as_str(),
                class)
        >
            {children()}
        </kbd>
    }
}