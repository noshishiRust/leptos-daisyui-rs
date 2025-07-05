use super::style::RatingSize;
use crate::merge_classes;
use leptos::{
    html::{Div, Input},
    prelude::*,
};

#[component]
pub fn Rating(
    #[prop(optional, into)] size: Signal<RatingSize>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            class=move || {
                merge_classes!("rating",
                size.get().as_str(),
                class)
            }
        >
            {children()}
        </div>
    }
}

#[component]
pub fn RatingItem(
    #[prop(optional, into)] checked: Signal<bool>,
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
            class=move || merge_classes!("mask", "mask-star-2", "bg-orange-400", class)
            prop:checked=checked
            on:change=move |ev| {
                if let Some(handler) = &on_change {
                    handler(event_target_checked(&ev));
                }
            }
        />
    }
}

#[component]
pub fn RatingHidden(
    #[prop(optional)] name: Option<&'static str>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Input>,
) -> impl IntoView {
    view! {
        <input
            node_ref=node_ref
            type="radio"
            name=name
            class=move || merge_classes!("rating-hidden", class)
        />
    }
}
