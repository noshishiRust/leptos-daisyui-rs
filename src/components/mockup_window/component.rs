use crate::merge_classes;
use leptos::{html::Div, prelude::*};

#[component]
pub fn MockupWindow(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=merge_classes!("mockup-window", class)>
            <div class="mockup-window-toolbar">
                <div class="mockup-window-button"></div>
                <div class="mockup-window-button"></div>
                <div class="mockup-window-button"></div>
            </div>
            <div class="mockup-window-content">{children()}</div>
        </div>
    }
}