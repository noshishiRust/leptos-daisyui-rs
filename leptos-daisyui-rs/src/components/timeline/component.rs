use super::style::{TimelineDirection, TimelineItemPosition};
use crate::merge_classes;
use leptos::{
    html::{Div, Li, Ul},
    prelude::*,
};

/// # Timeline Component
///
/// A reactive Leptos wrapper for daisyUI's timeline component that provides structured
/// layout for displaying sequential events or progress steps.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("timeline timeline-vertical timeline-horizontal timeline-snap-icon timeline-compact timeline-start timeline-middle timeline-end timeline-box");
/// ```
///
/// ## Node References
/// - `node_ref` - References the ul element ([HTMLUListElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLUListElement))
#[component]
pub fn Timeline(
    /// Direction of the timeline layout
    #[prop(optional, into)]
    direction: Signal<TimelineDirection>,

    /// Whether to snap icons to timeline
    #[prop(optional, into)]
    snap_icon: Signal<bool>,

    /// Whether to use compact spacing
    #[prop(optional, into)]
    compact: Signal<bool>,

    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference to the ul element
    #[prop(optional)]
    node_ref: NodeRef<Ul>,

    /// Child timeline items
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

/// # Timeline Item Component
///
/// A reactive Leptos wrapper for individual timeline items with connector lines.
///
/// ## Node References
/// - `node_ref` - References the li element ([HTMLLIElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLIElement))
#[component]
pub fn TimelineItem(
    /// Position in timeline affecting connector lines
    #[prop(into)]
    position: Signal<TimelineItemPosition>,

    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// CSS classes for the starting connector line
    #[prop(optional, into)]
    start_class: &'static str,

    /// CSS classes for the ending connector line
    #[prop(optional, into)]
    end_class: &'static str,

    /// Node reference to the li element
    #[prop(optional)]
    node_ref: NodeRef<Li>,

    /// Timeline item content
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

/// # Timeline Item Start Component
///
/// A reactive Leptos wrapper for timeline item start content positioning.
///
/// ## Node References
/// - `node_ref` - References the div element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn TimelineItemStart(
    /// Whether to apply box styling
    #[prop(optional, into)]
    boxed: Signal<bool>,

    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference to the div element
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// Start content
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

/// # Timeline Item Middle Component
///
/// A reactive Leptos wrapper for timeline item middle content positioning,
/// typically used for icons or indicators.
///
/// ## Node References
/// - `node_ref` - References the div element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn TimelineItemMiddle(
    /// Whether to apply box styling
    #[prop(optional, into)]
    boxed: Signal<bool>,

    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference to the div element
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// Middle content
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

/// # Timeline Item End Component
///
/// A reactive Leptos wrapper for timeline item end content positioning.
///
/// ## Node References
/// - `node_ref` - References the div element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn TimelineItemEnd(
    /// Whether to apply box styling
    #[prop(optional, into)]
    boxed: Signal<bool>,

    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference to the div element
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// End content
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
