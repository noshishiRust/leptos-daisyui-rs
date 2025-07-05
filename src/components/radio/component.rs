use super::style::{RadioColor, RadioSize};
use crate::merge_classes;
use leptos::{html::Input, prelude::*};

#[component]
pub fn Radio(
    #[prop(optional, into)] color: Signal<RadioColor>,
    #[prop(optional, into)] size: Signal<RadioSize>,
    #[prop(optional, into)] checked: Signal<bool>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional)] name: Option<&'static str>,
    #[prop(optional)] value: Option<&'static str>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Input>,
    #[prop(optional)] on_change: Option<Box<dyn Fn(bool)>>,
) -> impl IntoView {
    view! {
        <input
            node_ref=node_ref
            type="radio"
            name=name
            value=value
            class=move || {
                merge_classes!(
                    "radio",
                color.get().as_str(),
                size.get().as_str(),
                class
                )
            }
            prop:checked=checked
            prop:disabled=disabled
            on:change=move |ev| {
                if let Some(handler) = &on_change {
                    handler(event_target_checked(&ev));
                }
            }
        />
    }
}
