use crate::merge_classes;
use leptos::{
    html::{Div, Pre},
    prelude::*,
};

#[component]
pub fn MockupCode(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("mockup-code", class)>
            {children()}
        </div>
    }
}

#[component]
pub fn MockupCodeLine(
    #[prop(optional)] prefix: Option<&'static str>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Pre>,
    children: Children,
) -> impl IntoView {
    view! {
        <pre node_ref=node_ref class=class data-prefix=prefix.unwrap_or("$")>
            {children()}
        </pre>
    }
}
