use super::style::{CheckboxColor, CheckboxSize};
use crate::merge_classes;
use leptos::prelude::*;

#[component]
pub fn Checkbox(
    #[prop(optional, into)] color: Signal<CheckboxColor>,
    #[prop(optional, into)] size: Signal<CheckboxSize>,
    #[prop(optional, into)] checked: Signal<bool>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] on_change: Option<Box<dyn Fn(bool)>>,
) -> impl IntoView {
    view! {
        <input
            type="checkbox"
            class=merge_classes!(
                "checkbox",
                color.get().as_str(),
                size.get().as_str(),
                class
            )
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
