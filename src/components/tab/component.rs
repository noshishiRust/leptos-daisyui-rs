use super::style::{TabSize, TabVariant};
use crate::merge_classes;
use leptos::{
    html::{A, Div, Input},
    prelude::*,
};

#[component]
pub fn Tabs(
    #[prop(optional, into)] size: Signal<TabSize>,
    #[prop(optional, into)] variant: Signal<TabVariant>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            class=move || {
                merge_classes!(
                    "tabs",
                size.get().as_str(),
                variant.get().as_str(),
                class
                )
            }
        >
            {children()}
        </div>
    }
}

#[component]
pub fn Tab(
    #[prop(optional, into)] active: Signal<bool>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<A>,
    #[prop(optional)] on_click: Option<Box<dyn Fn()>>,
    children: Children,
) -> impl IntoView {
    view! {
        <a
            node_ref=node_ref
            class=move || merge_classes!("tab", class)
            class:tab-active=active
            class:tab-disabled=disabled
            on:click=move |_| {
                if let Some(handler) = &on_click {
                    handler();
                }
            }
        >
            {children()}
        </a>
    }
}

#[component]
pub fn TabRadio(
    #[prop(optional)] name: Option<&'static str>,
    #[prop(optional, into)] checked: Signal<bool>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Input>,
    #[prop(optional)] on_change: Option<Box<dyn Fn(bool)>>,
    children: Children,
) -> impl IntoView {
    view! {
        <input
            node_ref=node_ref
            type="radio"
            name=name
            class=move || merge_classes!("tab", class)
            prop:checked=checked
            on:change=move |ev| {
                if let Some(handler) = &on_change {
                    handler(event_target_checked(&ev));
                }
            }
        />
        {children()}
    }
}
