use crate::merge_classes;
use leptos::{
    html::{Div, Figure},
    prelude::*,
};

#[component]
pub fn Diff(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Figure>,
    children: Children,
) -> impl IntoView {
    view! {
        <figure node_ref=node_ref class=move || merge_classes!("diff", class)>
            {children()}
        </figure>
    }
}

#[component]
pub fn DiffItem1(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("diff-item-1", class)>
            {children()}
        </div>
    }
}

#[component]
pub fn DiffItem2(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("diff-item-2", class)>
            {children()}
        </div>
    }
}

#[component]
pub fn DiffResizer(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
) -> impl IntoView {
    view! { <div node_ref=node_ref class=move || merge_classes!("diff-resizer", class)></div> }
}
