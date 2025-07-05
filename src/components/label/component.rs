use crate::merge_classes;
use leptos::{
    html::{Label as HtmlLabel, Span},
    prelude::*,
};

#[component]
pub fn Label(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<HtmlLabel>,
    children: Children,
) -> impl IntoView {
    view! {
        <label node_ref=node_ref class=move || merge_classes!("label", class)>
            {children()}
        </label>
    }
}

#[component]
pub fn LabelText(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Span>,
    children: Children,
) -> impl IntoView {
    view! {
        <span node_ref=node_ref class=move || merge_classes!("label-text", class)>
            {children()}
        </span>
    }
}

#[component]
pub fn LabelTextAlt(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Span>,
    children: Children,
) -> impl IntoView {
    view! {
        <span node_ref=node_ref class=move || merge_classes!("label-text-alt", class)>
            {children()}
        </span>
    }
}
