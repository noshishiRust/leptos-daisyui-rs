use crate::merge_classes;
use leptos::{html::Div, prelude::*};

#[component]
pub fn MockupWindow(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("mockup-window", class)>
            {children()}
        </div>
    }
}
