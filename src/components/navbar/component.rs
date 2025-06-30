use crate::merge_classes;
use leptos::{
    html::{Div, Nav},
    prelude::*,
};

#[component]
pub fn Navbar(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Nav>,
    children: Children,
) -> impl IntoView {
    view! {
        <nav node_ref=node_ref class=merge_classes!("navbar", class)>
            {children()}
        </nav>
    }
}

#[component]
pub fn NavbarStart(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=merge_classes!("navbar-start", class)>
            {children()}
        </div>
    }
}

#[component]
pub fn NavbarCenter(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=merge_classes!("navbar-center", class)>
            {children()}
        </div>
    }
}

#[component]
pub fn NavbarEnd(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=merge_classes!("navbar-end", class)>
            {children()}
        </div>
    }
}
