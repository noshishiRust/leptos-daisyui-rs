use super::style::{TextColor, TextSize, TextWeight};
use crate::merge_classes;
use leptos::{html::Span, prelude::*};

/// # Text Component
///
/// Typography component with size, weight, color variants and text truncation support.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("text-xs text-sm text-base text-lg text-xl text-2xl text-3xl font-normal font-medium font-semibold font-bold truncate");
/// ```
#[component]
pub fn Text(
    #[prop(optional, into)] size: Signal<TextSize>,
    #[prop(optional, into)] weight: Signal<TextWeight>,
    #[prop(optional, into)] color: Signal<TextColor>,
    #[prop(optional, into)] truncate: Signal<bool>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Span>,
    children: Children,
) -> impl IntoView {
    view! {
        <span
            node_ref=node_ref
            class=move || merge_classes!(
                size.get().as_str(),
                weight.get().as_str(),
                color.get().as_str(),
                if truncate.get() { "truncate" } else { "" },
                class
            )
        >
            {children()}
        </span>
    }
}
