use super::style::PaginationSize;
use crate::merge_classes;
use leptos::{
    html::{Button, Div, Input},
    prelude::*,
};

#[component]
pub fn Pagination(
    #[prop(optional, into)] size: Signal<PaginationSize>,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            class=move || {
                merge_classes!("join",
                size.get().as_str(),
                class)
            }
        >
            {children()}
        </div>
    }
}

#[component]
pub fn PaginationButton(
    #[prop(optional, into)] active: Signal<bool>,
    #[prop(optional, into)] disabled: Signal<bool>,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Button>,
    #[prop(optional)] on_click: Option<Box<dyn Fn()>>,
    children: Children,
) -> impl IntoView {
    view! {
        <button
            node_ref=node_ref
            class=move || merge_classes!("join-item", "btn", class)
            class:btn-active=active
            prop:disabled=disabled
            on:click=move |_| {
                if let Some(handler) = &on_click {
                    handler();
                }
            }
        >
            {children()}
        </button>
    }
}

#[component]
pub fn PaginationInput(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Input>,
    #[prop(optional, into)] value: Signal<String>,
    #[prop(optional)] on_input: Option<Box<dyn Fn(String)>>,
) -> impl IntoView {
    view! {
        <input
            node_ref=node_ref
            class=move || merge_classes!("join-item", "btn", class)
            type="text"
            prop:value=value
            on:input=move |ev| {
                if let Some(handler) = &on_input {
                    handler(event_target_value(&ev));
                }
            }
        />
    }
}
