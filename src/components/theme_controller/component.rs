use crate::merge_classes;
use leptos::{html::Input, prelude::*};

#[component]
pub fn ThemeController(
    #[prop(optional, into)] theme_name: &'static str,
    #[prop(optional, into)] checked: Signal<bool>,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Input>,
) -> impl IntoView {
    view! {
        <input
            node_ref=node_ref
            type="checkbox"
            value=theme_name
            class=move || merge_classes!("theme-controller", class)
            prop:checked=checked
        />
    }
}
