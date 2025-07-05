use crate::merge_classes;
use leptos::{
    html::{Dialog, Div, Form},
    prelude::*,
};

#[component]
pub fn Modal(
    #[prop(optional, into)] open: Signal<bool>,
    #[prop(optional, into)] backdrop: Signal<bool>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Dialog>,
    children: Children,
) -> impl IntoView {
    Effect::new(move || {
        let Some(node) = node_ref.get() else { return };

        if open.get() {
            let _ = node.show_modal();
        } else {
            let _ = node.close();
        }
    });

    view! {
        <dialog
            aria_modal=move || open.get()
            aria-label="Modal"
            node_ref=node_ref
            class=move || merge_classes!("modal", class)
            class:modal-open=open
        >
            {children()}
            {move || {
                if backdrop.get() { view! { <ModalBackdrop /> }.into_any() } else { ().into_any() }
            }}
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
        <div node_ref=node_ref class=move || merge_classes!("modal-box", class)>
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
        <div node_ref=node_ref class=move || merge_classes!("modal-action", class)>
            {children()}
        </div>
    }
}

#[component]
pub fn ModalBackdrop(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Form>,
) -> impl IntoView {
    view! {
        <form
            node_ref=node_ref
            method="dialog"
            class=move || merge_classes!("modal-backdrop", class)
        >
            <button>close</button>
        </form>
    }
}
