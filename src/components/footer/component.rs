use super::style::{FooterDirection, FooterPlacement};
use crate::merge_classes;
use leptos::{
    html::{Div, Footer as HtmlFooter},
    prelude::*,
};

#[component]
pub fn Footer(
    #[prop(optional, into)] placement: Signal<FooterPlacement>,
    #[prop(optional, into)] direction: Signal<FooterDirection>,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    #[prop(optional)] node_ref: NodeRef<HtmlFooter>,
    children: Children,
) -> impl IntoView {
    view! {
        <footer
            node_ref=node_ref
            class=move || {
                merge_classes!(
                    "footer",
                placement.get().as_str(),
                direction.get().as_str(),
                class
                )
            }
        >
            {children()}
        </footer>
    }
}

#[component]
pub fn FooterTitle(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("footer-title", class)>
            {children()}
        </div>
    }
}
