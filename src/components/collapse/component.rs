use super::style::CollapseModifier;
use crate::merge_classes;
use leptos::prelude::*;

#[component]
pub fn Collapse(
    #[prop(optional, into)] modifier: Signal<CollapseModifier>,
    #[prop(optional, into)] class: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            tabindex="0"
            class=move || {
                merge_classes!(
                    "collapse",
                modifier.get().as_str(),
                class
                )
            }
        >
            {children()}
        </div>
    }
}

#[component]
pub fn CollapseCheckbox(
    #[prop(optional, into)] modifier: Signal<CollapseModifier>,
    #[prop(optional, into)] checked: Signal<bool>,
    #[prop(optional, into)] class: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=move || {
            merge_classes!(
                "collapse",
                modifier.get().as_str(),
                class
            )
        }>
            <input type="checkbox" prop:checked=checked />
            {children()}
        </div>
    }
}

#[component]
pub fn CollapseTitle(
    #[prop(optional, into)] class: &'static str,
    children: Children,
) -> impl IntoView {
    view! { <div class=move || merge_classes!("collapse-title", class)>{children()}</div> }
}

#[component]
pub fn CollapseContent(
    #[prop(optional, into)] class: &'static str,
    children: Children,
) -> impl IntoView {
    view! { <div class=move || merge_classes!("collapse-content", class)>{children()}</div> }
}
