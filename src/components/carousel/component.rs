use super::style::{CarouselDirection, CarouselModifier};
use crate::merge_classes;
use leptos::prelude::*;

#[component]
pub fn Carousel(
    #[prop(optional, into)] modifier: Signal<CarouselModifier>,
    #[prop(optional, into)] direction: Signal<CarouselDirection>,
    #[prop(optional, into)] class: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=move || {
            merge_classes!(
                "carousel",
                modifier.get().as_str(),
                direction.get().as_str(),
                class
            )
        }>{children()}</div>
    }
}

#[component]
pub fn CarouselItem(
    #[prop(optional, into)] class: &'static str,
    children: Children,
) -> impl IntoView {
    view! { <div class=move || merge_classes!("carousel-item", class)>{children()}</div> }
}
