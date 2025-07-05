use super::style::{CardSize, CardStyle};
use crate::merge_classes;
use leptos::prelude::*;

#[component]
pub fn Card(
    #[prop(optional, into)] style: Signal<CardStyle>,
    #[prop(optional, into)] size: Signal<CardSize>,
    #[prop(optional, into)] side: Signal<bool>,
    #[prop(optional, into)] image_full: Signal<bool>,
    #[prop(optional, into)] class: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class=move || {
                merge_classes!(
                    "card",
                style.get().as_str(),
                size.get().as_str(),
                class
                )
            }
            class:card-side=side
            class:image-full=image_full
        >
            {children()}
        </div>
    }
}

#[component]
pub fn CardBody(#[prop(optional, into)] class: &'static str, children: Children) -> impl IntoView {
    view! { <div class=move || merge_classes!("card-body", class)>{children()}</div> }
}

#[component]
pub fn CardTitle(#[prop(optional, into)] class: &'static str, children: Children) -> impl IntoView {
    view! { <h2 class=move || merge_classes!("card-title", class)>{children()}</h2> }
}

#[component]
pub fn CardActions(
    #[prop(optional, into)] class: &'static str,
    children: Children,
) -> impl IntoView {
    view! { <div class=move || merge_classes!("card-actions", class)>{children()}</div> }
}
