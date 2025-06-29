use super::style::SwapRotate;
use crate::merge_classes;
use leptos::{
    html::{Div, Label},
    prelude::*,
};

#[component]
pub fn Swap(
    #[prop(optional, into)] rotate: Signal<SwapRotate>,
    #[prop(optional, into)] active: Signal<bool>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Label>,
    children: Children,
) -> impl IntoView {
    view! {
        <label
            node_ref=node_ref
            class=merge_classes!(
                "swap",
                rotate.get().as_str(),
                class
            )
        >
            <input type="checkbox" prop:checked=active />
            {children()}
        </label>
    }
}

#[component]
pub fn SwapOn(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=merge_classes!("swap-on", class)>
            {children()}
        </div>
    }
}

#[component]
pub fn SwapOff(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=merge_classes!("swap-off", class)>
            {children()}
        </div>
    }
}
