use super::style::TimelineDirection;
use crate::merge_classes;
use leptos::{
    html::{Div, Li, Ul},
    prelude::*,
};

#[component]
pub fn Timeline(
    #[prop(optional, into)] direction: Signal<TimelineDirection>,
    #[prop(optional, into)] snap_icon: Signal<bool>,
    #[prop(optional, into)] compact: Signal<bool>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Ul>,
    children: Children,
) -> impl IntoView {
    view! {
        <ul
            node_ref=node_ref
            class=merge_classes!(
                "timeline",
                direction.get().as_str(),
                class
            )
            class:timeline-snap-icon=snap_icon
            class:timeline-compact=compact
        >
            {children()}
        </ul>
    }
}

#[component]
pub fn TimelineItem(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Li>,
    children: Children,
) -> impl IntoView {
    view! {
        <li node_ref=node_ref class=merge_classes!("", class)>
            {children()}
        </li>
    }
}

#[component]
pub fn TimelineStart(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=merge_classes!("timeline-start", class)>
            {children()}
        </div>
    }
}

#[component]
pub fn TimelineMiddle(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=merge_classes!("timeline-middle", class)>
            {children()}
        </div>
    }
}

#[component]
pub fn TimelineEnd(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=merge_classes!("timeline-end", class)>
            {children()}
        </div>
    }
}

#[component]
pub fn TimelineBox(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=merge_classes!("timeline-box", class)>
            {children()}
        </div>
    }
}
