use crate::merge_classes;
use leptos::{html::Span, prelude::*};

#[component]
pub fn Countdown(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Span>,
    children: Children,
) -> impl IntoView {
    view! {
        <span node_ref=node_ref class=merge_classes!("countdown", class)>
            {children()}
        </span>
    }
}

#[component]
pub fn CountdownValue(
    value: Signal<u8>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Span>,
    #[prop(optional, into)] aria_label: Signal<Option<String>>,
) -> impl IntoView {
    view! {
        <span
            node_ref=node_ref
            class=class
            style=move || format!("--value:{};", value.get())
            aria-live="polite"
            aria-label=move || {
                if let Some(label) = aria_label.get() { label } else { value.get().to_string() }
            }
        >
            {move || value.get()}
        </span>
    }
}