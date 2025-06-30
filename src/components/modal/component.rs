use crate::merge_classes;
use leptos::{
    html::{Dialog, Div, Form},
    prelude::*,
};

#[component]
pub fn Modal(
    #[prop(optional, into)] open: Signal<bool>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Dialog>,
    children: Children,
) -> impl IntoView {
    view! {
        <dialog node_ref=node_ref class=merge_classes!("modal", class) class:modal-open=open>
            {children()}
        </dialog>
    }
}

#[component]
pub fn ModalBox(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=merge_classes!("modal-box", class)>
            {children()}
        </div>
    }
}

#[component]
pub fn ModalAction(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=merge_classes!("modal-action", class)>
            {children()}
        </div>
    }
}

#[component]
pub fn ModalBackdrop(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Form>,
    #[prop(optional)] on_close: Option<Box<dyn Fn()>>,
    children: Children,
) -> impl IntoView {
    view! {
        <form
            node_ref=node_ref
            method="dialog"
            class=merge_classes!("modal-backdrop", class)
            on:submit=move |_| {
                if let Some(handler) = &on_close {
                    handler();
                }
            }
        >
            {children()}
        </form>
    }
}
