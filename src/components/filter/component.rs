use crate::merge_classes;
use leptos::{
    html::{Div, Input},
    prelude::*,
};

/// # Filter Component
///
/// A group of radio buttons that work as filters. When one option is selected,
/// it shows a reset button and allows filtering content.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("filter filter-reset");
/// ```
///
/// ## Node References
/// - `node_ref` - References the container `<div>` element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn Filter(
    /// Additional CSS classes to apply to the filter container
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the filter `<div>` element
    #[prop(optional)] 
    node_ref: NodeRef<Div>,

    /// Child elements including filter radio buttons and [`FilterReset`]
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("filter", class)>
            {children()}
        </div>
    }
}

/// # Filter Form Component
///
/// A form wrapper for filter components that provides better semantic structure
/// and built-in reset functionality.
#[component]
pub fn FilterForm(
    /// Additional CSS classes to apply to the form
    #[prop(optional, into)]
    class: &'static str,

    /// Child elements including filter radio buttons
    children: Children,
) -> impl IntoView {
    view! { <form class=move || merge_classes!("filter", class)>{children()}</form> }
}

/// # Filter Reset Component
///
/// A reset button that clears the current filter selection and shows all content.
///
/// ## Node References
/// - `node_ref` - References the reset `<input>` element ([HTMLInputElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement))
#[component]
pub fn FilterReset(
    /// Radio group name that matches the filter buttons
    name: &'static str,

    /// Additional CSS classes to apply to the reset button
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the reset `<input>` element
    #[prop(optional)] 
    node_ref: NodeRef<Input>,
) -> impl IntoView {
    view! {
        <input
            node_ref=node_ref
            class=move || merge_classes!("btn filter-reset", class)
            type="radio"
            name=name
            aria-label="×"
        />
    }
}
