use crate::merge_classes;
use leptos::{html::Input, prelude::*};

#[component]
pub fn ThemeController(
    #[prop(optional, into)] theme_name: &'static str,
    #[prop(optional, into)] checked: Signal<bool>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Input>,
    #[prop(optional)] on_change: Option<Box<dyn Fn(bool)>>,
) -> impl IntoView {
    view! {
        <input
            node_ref=node_ref
            type="checkbox"
            value=theme_name
            class=merge_classes!("theme-controller", class)
            prop:checked=checked
            on:change=move |ev| {
                if let Some(handler) = &on_change {
                    handler(event_target_checked(&ev));
                }
            }
        />
    }
}
