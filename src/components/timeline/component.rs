use super::style::{TimelineDirection, TimelineItemPosition};
use crate::merge_classes;
use leptos::{
    html::{Div, Li, Ul},
    prelude::*,
};

/// A Timeline component that displays a series of events or milestones.
///
/// This component is a wrapper `<ul>` element,
/// you can spread [HTMLUListElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLUListElement) attributes to it.
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
            class=move || {
                merge_classes!(
                    "timeline",
                    direction.get().as_str(),
                    class
                )
            }
            class:timeline-snap-icon=snap_icon
            class:timeline-compact=compact
        >
            {children()}
        </ul>
    }
}

#[component]
pub fn TimelineItem(
    #[prop(into)] position: Signal<TimelineItemPosition>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional, into)] start_class: &'static str,
    #[prop(optional, into)] end_class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Li>,
    children: Children,
) -> impl IntoView {
    view! {
        <li node_ref=node_ref class=class>
            {move || {
                let position = position.get();
                if position.is_end() || position.is_between() {
                    view! { <hr class=start_class /> }.into_any()
                } else {
                    ().into_any()
                }
            }}

            {children()}

            {move || {
                let position = position.get();
                if position.is_start() || position.is_between() {
                    view! { <hr class=end_class /> }.into_any()
                } else {
                    ().into_any()
                }
            }}
        </li>
    }
}

#[component]
pub fn TimelineItemStart(
    #[prop(optional, into)] boxed: Signal<bool>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            class=move || merge_classes!("timeline-start", class)
            class:timeline-box=boxed
        >
            {children()}
        </div>
    }
}

#[component]
pub fn TimelineItemMiddle(
    #[prop(optional, into)] boxed: Signal<bool>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            class=move || merge_classes!("timeline-middle", class)
            class:timeline-box=boxed
        >
            {children()}
        </div>
    }
}

#[component]
pub fn TimelineItemEnd(
    #[prop(optional, into)] boxed: Signal<bool>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            class=move || merge_classes!("timeline-end", class)
            class:timeline-box=boxed
        >
            {children()}
        </div>
    }
}
