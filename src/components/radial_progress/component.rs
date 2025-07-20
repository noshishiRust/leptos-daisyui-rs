use super::style::{RadialProgressColor, RadialProgressSize};
use crate::merge_classes;
use leptos::{html::Div, prelude::*};

#[component]
pub fn RadialProgress(
    #[prop(optional, into)] color: Signal<RadialProgressColor>,
    #[prop(optional, into)] size: Signal<RadialProgressSize>,
    #[prop(optional, into)] value: Signal<f64>,
    #[prop(optional, into)] thickness: Signal<String>,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let progress_style =
        move || format!("--value:{}; --thickness:{};", value.get(), thickness.get());

    view! {
        <div
            node_ref=node_ref
            class=move || {
                merge_classes!(
                    "radial-progress",
                    color.get().as_str(),
                    size.get().as_str(),
                    class
                )
            }
            style=progress_style
        >
            {children.map(|v| v())}
        </div>
    }
}
