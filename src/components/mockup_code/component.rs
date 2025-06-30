use crate::merge_classes;
use leptos::{html::Div, prelude::*};

#[component]
pub fn MockupCode(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=merge_classes!("mockup-code", class)>
            <pre>{children()}</pre>
        </div>
    }
}

#[component]
pub fn MockupCodeLine(
    #[prop(optional)] prefix: Option<&'static str>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=merge_classes!("", class) data-prefix=prefix.unwrap_or("$")>
            {children()}
        </div>
    }
}
