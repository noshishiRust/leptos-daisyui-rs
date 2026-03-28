use crate::merge_classes;
use leptos::{
    html::{Div, Figure},
    prelude::*,
};

/// # Diff Component
///
/// A side-by-side comparison component that allows users to compare two items
/// with an interactive resizer to adjust the view proportions.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("diff diff-item-1 diff-item-2 diff-resizer");
/// ```
///
/// ## Node References
/// - `node_ref` - References the outer `<figure>` element ([HTMLElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement))
#[component]
pub fn Diff(
    /// Additional CSS classes to apply to the diff container
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the diff `<figure>` element
    #[prop(optional)]
    node_ref: NodeRef<Figure>,

    /// Child components: [`DiffItem1`], [`DiffItem2`], and [`DiffResizer`]
    children: Children,
) -> impl IntoView {
    view! {
        <figure node_ref=node_ref class=move || merge_classes!("diff", class)>
            {children()}
        </figure>
    }
}

/// # Diff Item 1 Component
///
/// The first (left) item in the diff comparison.
///
/// ## Node References
/// - `node_ref` - References the item `<div>` element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn DiffItem1(
    /// Additional CSS classes to apply to the first diff item
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the first item `<div>` element
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// Content for the first comparison item
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("diff-item-1", class)>
            {children()}
        </div>
    }
}

/// # Diff Item 2 Component
///
/// The second (right) item in the diff comparison.
///
/// ## Node References
/// - `node_ref` - References the item `<div>` element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn DiffItem2(
    /// Additional CSS classes to apply to the second diff item
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the second item `<div>` element
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// Content for the second comparison item
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("diff-item-2", class)>
            {children()}
        </div>
    }
}

/// # Diff Resizer Component
///
/// An interactive resizer element that allows users to adjust the proportions
/// between the two diff items by dragging.
///
/// ## Node References
/// - `node_ref` - References the resizer `<div>` element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn DiffResizer(
    /// Additional CSS classes to apply to the resizer
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the resizer `<div>` element
    #[prop(optional)]
    node_ref: NodeRef<Div>,
) -> impl IntoView {
    view! { <div node_ref=node_ref class=move || merge_classes!("diff-resizer", class)></div> }
}
