use super::style::{SelectColor, SelectSize, SelectStyle};
use crate::merge_classes;
use leptos::{
    html::{Option_, Select as HtmlSelect},
    prelude::*,
};

#[component]
pub fn Select(
    #[prop(optional, into)] style: Signal<SelectStyle>,
    #[prop(optional, into)] color: Signal<SelectColor>,
    #[prop(optional, into)] size: Signal<SelectSize>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<HtmlSelect>,
    children: Children,
) -> impl IntoView {
    view! {
        <select
            node_ref=node_ref
            class=move || {
                merge_classes!(
                    "select",
                style.get().as_str(),
                color.get().as_str(),
                size.get().as_str(),
                class
                )
            }
            disabled=disabled
        >
            {children()}
        </select>
    }
}

#[component]
pub fn SelectOption(
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Option_>,
    children: Children,
) -> impl IntoView {
    view! {
        <option node_ref=node_ref class=class disabled=disabled>
            {children()}
        </option>
    }
}
