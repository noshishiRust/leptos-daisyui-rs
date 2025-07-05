use super::style::{ButtonColor, ButtonShape, ButtonSize, ButtonStyle};
use crate::merge_classes;
use leptos::{
    html::{Button as HTMLButton, A},
    prelude::*,
};

#[component]
pub fn Button(
    #[prop(optional, into)] loading: Signal<bool>,
    #[prop(optional, into)] color: Signal<ButtonColor>,
    #[prop(optional, into)] style: Signal<ButtonStyle>,
    #[prop(optional, into)] size: Signal<ButtonSize>,
    #[prop(optional, into)] shape: Signal<ButtonShape>,
    #[prop(optional, into)] active: Signal<bool>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] node_ref: NodeRef<HTMLButton>,
    #[prop(optional, into)] class: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <button
            disabled=disabled
            node_ref=node_ref
            class=move || {
                merge_classes!(
                    "btn",
                    color.get().as_str(),
                    style.get().as_str(),
                    size.get().as_str(),
                    shape.get().as_str(),
                    class
                )
            }
            class:btn-active=active
            class:btn-disabled=disabled
            class:loading=loading
        >
            {children()}
        </button>
    }
}

#[component]
pub fn LinkButton(
    #[prop(optional)] href: &'static str,
    #[prop(optional, into)] color: Signal<ButtonColor>,
    #[prop(optional, into)] style: Signal<ButtonStyle>,
    #[prop(optional, into)] size: Signal<ButtonSize>,
    #[prop(optional, into)] shape: Signal<ButtonShape>,
    #[prop(optional, into)] node_ref: NodeRef<A>,
    #[prop(optional, into)] class: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <a
            href=href
            node_ref=node_ref
            class=move || {
                merge_classes!(
                    "btn",
                color.get().as_str(),
                style.get().as_str(),
                size.get().as_str(),
                shape.get().as_str(),
                class
                )
            }
        >
            {children()}
        </a>
    }
}
