use super::style::{StepColor, StepsDirection};
use crate::merge_classes;
use leptos::{
    html::{Li, Ul},
    prelude::*,
};

#[component]
pub fn Steps(
    #[prop(optional, into)] direction: Signal<StepsDirection>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Ul>,
    children: Children,
) -> impl IntoView {
    view! {
        <ul
            node_ref=node_ref
            class=merge_classes!(
                "steps",
                direction.get().as_str(),
                class
            )
        >
            {children()}
        </ul>
    }
}

#[component]
pub fn Step(
    #[prop(optional, into)] color: Signal<StepColor>,
    #[prop(optional, into)] data_content: Option<&'static str>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Li>,
    children: Children,
) -> impl IntoView {
    view! {
        <li
            node_ref=node_ref
            class=merge_classes!(
                "step",
                color.get().as_str(),
                class
            )
            data-content=data_content
        >
            {children()}
        </li>
    }
}
