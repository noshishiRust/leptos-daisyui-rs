use crate::merge_classes;
use leptos::{html::Div, prelude::*};

#[component]
pub fn Hero(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("hero", class)>
            {children()}
        </div>
    }
}

#[component]
pub fn HeroContent(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("hero-content", class)>
            {children()}
        </div>
    }
}

#[component]
pub fn HeroOverlay(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
) -> impl IntoView {
    view! { <div node_ref=node_ref class=move || merge_classes!("hero-overlay", class)></div> }
}
