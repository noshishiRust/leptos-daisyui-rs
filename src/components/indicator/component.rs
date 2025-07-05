use super::style::{IndicatorHorizontalPlacement, IndicatorVerticalPlacement};
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
        <div node_ref=node_ref class=move || merge_classes!("indicator", class)>
            {children()}
        </div>
    }
}

#[component]
pub fn IndicatorItem(
    #[prop(optional, into)] vertical: Signal<IndicatorVerticalPlacement>,
    #[prop(optional, into)] horizontal: Signal<IndicatorHorizontalPlacement>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Span>,
    children: Children,
) -> impl IntoView {
    view! {
        <span
            node_ref=node_ref
            class=move || {
                merge_classes!(
                    "indicator-item",
                    vertical.get().as_str(),
                    horizontal.get().as_str(),
                    class
                )
            }
        >
            {children()}
        </span>
    }
}
