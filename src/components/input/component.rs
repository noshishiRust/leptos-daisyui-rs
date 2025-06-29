use super::style::{InputColor, InputSize, InputStyle};
use crate::merge_classes;
use leptos::{html::Input as HtmlInput, prelude::*};

#[component]
pub fn Input(
    #[prop(optional, into)] style: Signal<InputStyle>,
    #[prop(optional, into)] color: Signal<InputColor>,
    #[prop(optional, into)] size: Signal<InputSize>,
    #[prop(optional)] input_type: Option<&'static str>,
    #[prop(optional, into)] placeholder: &'static str,
    #[prop(optional, into)] value: Signal<String>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<HtmlInput>,
) -> impl IntoView {
    view! {
        <input
            node_ref=node_ref
            type=input_type.unwrap_or("text")
            class=merge_classes!(
                "input",
                style.get().as_str(),
                color.get().as_str(),
                size.get().as_str(),
                class
            )
            placeholder=placeholder
            value=value
        />
    }
}
