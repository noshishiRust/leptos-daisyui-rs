use crate::components::{InputColor, InputSize, InputStyle, SelectColor, SelectSize, SelectStyle};
use crate::merge_classes;
use leptos::{
    html::{Div, Input, Select},
    prelude::*,
};

#[component]
pub fn ValidatorInput(
    #[prop(optional, into)] style: Signal<InputStyle>,
    #[prop(optional, into)] color: Signal<InputColor>,
    #[prop(optional, into)] size: Signal<InputSize>,
    #[prop(optional)] input_type: Option<&'static str>,
    #[prop(optional, into)] placeholder: &'static str,
    #[prop(optional, into)] value: Signal<String>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Input>,
) -> impl IntoView {
    view! {
        <input
            node_ref=node_ref
            type=input_type.unwrap_or("text")
            class=move || {
                merge_classes!(
                    "input",
                "validator",
                style.get().as_str(),
                color.get().as_str(),
                size.get().as_str(),
                class
                )
            }
            placeholder=placeholder
            value=value
        />
    }
}

#[component]
pub fn ValidatorSelect(
    #[prop(optional, into)] style: Signal<SelectStyle>,
    #[prop(optional, into)] color: Signal<SelectColor>,
    #[prop(optional, into)] size: Signal<SelectSize>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Select>,
    children: Children,
) -> impl IntoView {
    view! {
        <select
            node_ref=node_ref
            class=move || {
                merge_classes!(
                    "select",
                "validator",
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
pub fn ValidatorHint(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("validator-text", class)>
            {children()}
        </div>
    }
}
