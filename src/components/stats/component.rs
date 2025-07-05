use crate::merge_classes;
use leptos::{html::Div, prelude::*};

#[component]
pub fn Stats(
    #[prop(optional, into)] vertical: Signal<bool>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            class=move || merge_classes!("stats", class)
            class:stats-vertical=vertical
        >
            {children()}
        </div>
    }
}

#[component]
pub fn Stat(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("stat", class)>
            {children()}
        </div>
    }
}

#[component]
pub fn StatTitle(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("stat-title", class)>
            {children()}
        </div>
    }
}

#[component]
pub fn StatValue(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("stat-value", class)>
            {children()}
        </div>
    }
}

#[component]
pub fn StatDesc(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("stat-desc", class)>
            {children()}
        </div>
    }
}

#[component]
pub fn StatFigure(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("stat-figure", class)>
            {children()}
        </div>
    }
}

#[component]
pub fn StatActions(
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Div>,
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("stat-actions", class)>
            {children()}
        </div>
    }
}
