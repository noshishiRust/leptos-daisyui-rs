use super::style::{ToggleColor, ToggleSize};
use crate::merge_classes;
use leptos::{html::Input, prelude::*};

#[component]
pub fn Toggle(
    #[prop(optional, into)] color: Signal<ToggleColor>,
    #[prop(optional, into)] size: Signal<ToggleSize>,
    #[prop(optional, into)] checked: Signal<bool>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Input>,
    #[prop(optional)] on_change: Option<Callback<bool>>,
) -> impl IntoView {
    view! {
        <input
            node_ref=node_ref
            type="checkbox"
            class=merge_classes!(
                "toggle",
                color.get().as_str(),
                size.get().as_str(),
                class
            )
            prop:checked=checked
            prop:disabled=disabled
            on:change=move |ev| {
                if let Some(handler) = &on_change {
                    handler.run(event_target_checked(&ev));
                }
            }
        />
    }
}
