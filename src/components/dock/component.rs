use super::style::DockSize;
use crate::merge_classes;
use leptos::{
    html::{Button, Div, Span},
    prelude::*,
};

#[component]
pub fn Dock(
    #[prop(optional, into)] size: Signal<DockSize>,
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
                merge_classes!("dock",
                size.get().as_str(),
                class)
            }
        >
            {children()}
        </div>
    }
}

#[component]
pub fn DockItem(
    #[prop(optional, into)] active: Signal<bool>,
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
            class=class
            class:dock-active=active
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
pub fn DockLabel(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Span>,
    children: Children,
) -> impl IntoView {
    view! {
        <span node_ref=node_ref class=move || merge_classes!("dock-label", class)>
            {children()}
        </span>
    }
}
