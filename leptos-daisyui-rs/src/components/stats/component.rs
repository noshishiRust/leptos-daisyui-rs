use crate::merge_classes;
use leptos::{html::Div, prelude::*};

/// # Stats Component
///
/// A reactive Leptos wrapper for daisyUI's stats component that provides a container
/// for displaying statistical data in organized, visually appealing card layouts.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("stats stats-horizontal stats-vertical stat stat-title stat-value stat-desc stat-figure stat-actions");
/// ```
///
/// ## Node References
/// - `node_ref` - References the div element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn Stats(
    /// Toggle vertical layout
    #[prop(optional, into)]
    vertical: Signal<bool>,

    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference to the div element
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

/// Individual statistic item within a Stats container.
#[component]
pub fn Stat(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference to the div element
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

/// Title/label component for a statistic.
#[component]
pub fn StatTitle(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Node reference to the div element
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

/// Primary value display component for a statistic.
#[component]
pub fn StatValue(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference to the div element
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

/// Description component providing additional context for a statistic.
#[component]
pub fn StatDesc(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference to the div element
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

/// Figure component for displaying icons, images, or visual elements in a statistic.
#[component]
pub fn StatFigure(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference to the div element
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

/// Actions container for interactive elements within a statistic.
#[component]
pub fn StatActions(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference to the div element
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
