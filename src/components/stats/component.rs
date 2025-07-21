use crate::merge_classes;
use leptos::{html::Div, prelude::*};

#[component]
pub fn Stats(
    /// Toggle vertical layout (default: false for horizontal layout)
    #[prop(optional, into)]
    vertical: Signal<bool>,

    /// Additional CSS classes to apply
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the container div
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// Child components (typically Stat components)
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
    /// Additional CSS classes to apply
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the container div
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// Child components (StatTitle, StatValue, StatDesc, etc.)
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
    /// Additional CSS classes to apply
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Node reference for the container div
    #[prop(optional)]
    node_ref: NodeRef<Div>,
    /// Title content
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
    /// Additional CSS classes to apply
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the container div
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// The primary value content
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
    /// Additional CSS classes to apply

    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the container div
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// Description content
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
    /// Additional CSS classes to apply
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the container div
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// Visual content (icons, images, etc.)
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
    /// Additional CSS classes to apply
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the container div
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// Interactive content (buttons, links, etc.)
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("stat-actions", class)>
            {children()}
        </div>
    }
}
