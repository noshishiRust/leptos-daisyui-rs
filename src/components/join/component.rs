use super::style::JoinDirection;
use crate::merge_classes;
use leptos::{html::Div, prelude::*};

#[component]
pub fn Join(
    #[prop(optional, into)] direction: Signal<JoinDirection>,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            class=move || {
                merge_classes!(
                    "join",
                direction.get().as_str(),
                class
                )
            }
        >
            {children()}
        </div>
    }
}
