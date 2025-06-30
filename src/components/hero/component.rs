use crate::merge_classes;
use leptos::{html::Div, prelude::*};

#[component]
pub fn Hero(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=merge_classes!("hero", class)>
            {children()}
        </div>
    }
}

#[component]
pub fn HeroContent(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=merge_classes!("hero-content", class)>
            {children()}
        </div>
    }
}

#[component]
pub fn HeroOverlay(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
) -> impl IntoView {
    view! { <div node_ref=node_ref class=merge_classes!("hero-overlay", class)></div> }
}
