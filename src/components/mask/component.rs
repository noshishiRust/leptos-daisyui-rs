use super::style::MaskType;
use crate::merge_classes;
use leptos::{html::Div, prelude::*};

#[component]
pub fn Mask(
    #[prop(optional, into)] mask_type: Signal<MaskType>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            class=move || {
                merge_classes!(
                    "mask",
                mask_type.get().as_str(),
                class
                )
            }
        >
            {children()}
        </div>
    }
}
