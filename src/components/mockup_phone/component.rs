use super::style::MockupPhoneColor;
use crate::merge_classes;
use leptos::{html::Div, prelude::*};

#[component]
pub fn MockupPhone(
    #[prop(optional, into)] color: Signal<MockupPhoneColor>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            class=merge_classes!(
                "mockup-phone",
                color.get().as_str(),
                class
            )
        >
            <div class="camera"></div>
            <div class="display">
                <div class="artboard">{children()}</div>
            </div>
        </div>
    }
}
