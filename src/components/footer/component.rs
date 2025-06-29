use super::style::{FooterPlacement, FooterDirection};
use crate::merge_classes;
use leptos::{html::{Footer as HtmlFooter, Div}, prelude::*};

#[component]
pub fn Footer(
    #[prop(optional, into)] placement: Signal<FooterPlacement>,
    #[prop(optional, into)] direction: Signal<FooterDirection>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<HtmlFooter>,
    children: Children,
) -> impl IntoView {
    view! {
        <footer
            node_ref=node_ref
            class=merge_classes!(
                "footer",
                placement.get().as_str(),
                direction.get().as_str(),
                class
            )
        >
            {children()}
        </footer>
    }
}

#[component]
pub fn FooterTitle(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=merge_classes!("footer-title", class)>
            {children()}
        </div>
    }
}