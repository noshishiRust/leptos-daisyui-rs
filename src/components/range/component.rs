use super::style::{RangeColor, RangeSize};
use crate::merge_classes;
use leptos::{html::Input, prelude::*};

#[component]
pub fn Range(
    #[prop(optional, into)] color: Signal<RangeColor>,
    #[prop(optional, into)] size: Signal<RangeSize>,
    #[prop(optional, into)] value: Signal<f64>,
    #[prop(optional, into)] min: Signal<f64>,
    #[prop(optional, into)] max: Signal<f64>,
    #[prop(optional, into)] step: Signal<f64>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Input>,
    #[prop(optional)] on_input: Option<Box<dyn Fn(f64)>>,
) -> impl IntoView {
    view! {
        <input
            node_ref=node_ref
            type="range"
            class=merge_classes!(
                "range",
                color.get().as_str(),
                size.get().as_str(),
                class
            )
            value=value
            min=min
            max=max
            step=step
            on:input=move |ev| {
                if let Some(handler) = &on_input {
                    if let Ok(val) = event_target_value(&ev).parse::<f64>() {
                        handler(val);
                    }
                }
            }
        />
    }
}
