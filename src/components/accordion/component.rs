use super::style::AccordionModifier;
use crate::merge_classes;
use leptos::{html::Input, prelude::*};

#[component]
pub fn Accordion(
    #[prop(optional)] name: Option<&'static str>,
    #[prop(optional, into)] checked: Signal<bool>,
    #[prop(optional, into)] modifier: Signal<AccordionModifier>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Input>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=merge_classes!(
            "collapse",
            modifier.get().as_str(),
            class
        )>
            <input node_ref=node_ref type="radio" name=name prop:checked=checked />
            {children()}
        </div>
    }
}

#[component]
pub fn AccordionTitle(
    #[prop(optional, into)] class: &'static str,
    children: Children,
) -> impl IntoView {
    view! { <div class=merge_classes!("collapse-title", class)>{children()}</div> }
}

#[component]
pub fn AccordionContent(
    #[prop(optional, into)] class: &'static str,
    children: Children,
) -> impl IntoView {
    view! { <div class=merge_classes!("collapse-content", class)>{children()}</div> }
}
