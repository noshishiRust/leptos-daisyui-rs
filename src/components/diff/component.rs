use crate::merge_classes;
use leptos::{html::{Figure, Div}, prelude::*};

#[component]
pub fn Diff(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Figure>,
    children: Children,
) -> impl IntoView {
    view! {
        <figure node_ref=node_ref class=merge_classes!("diff", class)>
            {children()}
        </figure>
    }
}

#[component]
pub fn DiffItem1(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=merge_classes!("diff-item-1", class)>
            {children()}
        </div>
    }
}

#[component]
pub fn DiffItem2(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=merge_classes!("diff-item-2", class)>
            {children()}
        </div>
    }
}

#[component]
pub fn DiffResizer(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
) -> impl IntoView {
    view! { <div node_ref=node_ref class=merge_classes!("diff-resizer", class)></div> }
}