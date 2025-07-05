use crate::merge_classes;
use leptos::{html::Div, prelude::*};

#[component]
pub fn Stack(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("stack", class)>
            {children()}
        </div>
    }
}
