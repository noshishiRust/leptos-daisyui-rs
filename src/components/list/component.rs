use crate::merge_classes;
use leptos::{html::{Ul, Li}, prelude::*};

#[component]
pub fn List(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Ul>,
    children: Children,
) -> impl IntoView {
    view! {
        <ul node_ref=node_ref class=merge_classes!("list", class)>
            {children()}
        </ul>
    }
}

#[component]
pub fn ListRow(
    #[prop(optional, into)] col_wrap: Signal<bool>,
    #[prop(optional, into)] col_grow: Signal<bool>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Li>,
    children: Children,
) -> impl IntoView {
    view! {
        <li
            node_ref=node_ref
            class=merge_classes!("list-row", class)
            class:list-col-wrap=col_wrap
            class:list-col-grow=col_grow
        >
            {children()}
        </li>
    }
}