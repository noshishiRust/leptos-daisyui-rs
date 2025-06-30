use super::style::IndicatorPlacement;
use crate::merge_classes;
use leptos::{
    html::{Div, Span},
    prelude::*,
};

#[component]
pub fn Indicator(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=merge_classes!("indicator", class)>
            {children()}
        </div>
    }
}

#[component]
pub fn IndicatorItem(
    #[prop(optional, into)] placement: Signal<IndicatorPlacement>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Span>,
    children: Children,
) -> impl IntoView {
    view! {
        <span
            node_ref=node_ref
            class=merge_classes!(
                "indicator-item",
                placement.get().as_str(),
                class
            )
        >
            {children()}
        </span>
    }
}
