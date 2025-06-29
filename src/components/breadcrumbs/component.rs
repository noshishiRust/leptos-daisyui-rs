use crate::merge_classes;
use leptos::html::{Div, Li, Ul};
use leptos::prelude::*;

#[component]
pub fn Breadcrumbs(
    #[prop(optional, into)] ouuter_class: &'static str,
    #[prop(optional, into)] outer_node_ref: NodeRef<Div>,
    #[prop(optional, into)] inner_class: &'static str,
    #[prop(optional, into)] inner_node_ref: NodeRef<Ul>,
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=outer_node_ref class=merge_classes!("breadcrumbs", ouuter_class)>
            <ul node_ref=inner_node_ref class=inner_class>
                {children()}
            </ul>
        </div>
    }
}

#[component]
pub fn BreadcrumbItem(
    #[prop(optional, into)] href: MaybeProp<String>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional, into)] node_ref: NodeRef<Li>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <li node_ref=node_ref class=class>
            {if let Some(href) = href.get() {
                view! { <a href=href>{children.map(|v| v())}</a> }.into_any()
            } else {
                view! { {children.map(|v| v())} }.into_any()
            }}
        </li>
    }
}
