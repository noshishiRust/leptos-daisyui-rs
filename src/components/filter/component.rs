use crate::merge_classes;
use leptos::{
    html::{Div, Input},
    prelude::*,
};

#[component]
pub fn Filter(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("filter", class)>
            {children()}
        </div>
    }
}

#[component]
pub fn FilterForm(
    #[prop(optional, into)] class: &'static str,
    children: Children,
) -> impl IntoView {
    view! { <form class=move || merge_classes!("filter", class)>{children()}</form> }
}

#[component]
pub fn FilterReset(
    name: &'static str,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Input>,
) -> impl IntoView {
    view! {
        <input
            node_ref=node_ref
            class=move || merge_classes!("btn filter-reset", class)
            type="radio"
            name=name
            aria-label="×"
        />
    }
}
