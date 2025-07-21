use crate::merge_classes;
use leptos::{
    html::{Li, Ul},
    prelude::*,
};

/// # List Component
///
/// A vertical layout component for displaying information in rows with
/// flexible column arrangements.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("list list-row list-col-wrap list-col-grow");
/// ```
///
/// ## Node References
/// - `node_ref` - References the `<ul>` element ([HTMLUListElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLUListElement))
#[component]
pub fn List(
    /// Additional CSS classes to apply to the list container
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the `<ul>` element
    #[prop(optional)] 
    node_ref: NodeRef<Ul>,

    /// Child [`ListRow`] components
    children: Children,
) -> impl IntoView {
    view! {
        <ul node_ref=node_ref class=move || merge_classes!("list", class)>
            {children()}
        </ul>
    }
}

/// # List Row Component
///
/// An individual row within a list that can contain multiple columns
/// with flexible layout options.
///
/// ## Node References
/// - `node_ref` - References the `<li>` element ([HTMLLIElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLIElement))
#[component]
pub fn ListRow(
    /// Force content to wrap to the next line
    #[prop(optional, into)] 
    col_wrap: Signal<bool>,

    /// Make this column fill remaining space instead of the default second column
    #[prop(optional, into)] 
    col_grow: Signal<bool>,

    /// Additional CSS classes to apply to the row
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the `<li>` element
    #[prop(optional)] 
    node_ref: NodeRef<Li>,

    /// Row content (multiple columns supported)
    children: Children,
) -> impl IntoView {
    view! {
        <li
            node_ref=node_ref
            class=move || merge_classes!("list-row", class)
            class:list-col-wrap=col_wrap
            class:list-col-grow=col_grow
        >
            {children()}
        </li>
    }
}
