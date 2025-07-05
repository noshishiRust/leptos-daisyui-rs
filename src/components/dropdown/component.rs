use crate::merge_classes;
use leptos::{
    html::{Details, Summary, Ul},
    prelude::*,
};

#[component]
pub fn Dropdown(
    #[prop(optional, into)] hover: Signal<bool>,
    #[prop(optional, into)] open: Signal<bool>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Details>,
    children: Children,
) -> impl IntoView {
    view! {
        <details
            node_ref=node_ref
            class=move || merge_classes!("dropdown", class)
            class:dropdown-hover=hover
            class:dropdown-open=open
        >
            {children()}
        </details>
    }
}

#[component]
pub fn DropdownTrigger(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Summary>,
    children: Children,
) -> impl IntoView {
    view! {
        <summary node_ref=node_ref class=class>
            {children()}
        </summary>
    }
}

#[component]
pub fn DropdownContent(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Ul>,
    children: Children,
) -> impl IntoView {
    view! {
        <ul node_ref=node_ref class=move || merge_classes!("dropdown-content", class)>
            {children()}
        </ul>
    }
}
