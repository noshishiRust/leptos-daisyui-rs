/// # Pagination Component
///
/// A reactive Leptos wrapper for daisyUI's pagination component that provides navigation
/// controls for paginated content using join layout.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("join join-xs join-sm join-md join-lg join-item btn btn-active");
/// ```
///
/// ## Node References
/// - `node_ref` - References the container div element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))

use super::style::PaginationSize;
use crate::merge_classes;
use leptos::{
    html::{Button, Div, Input},
    prelude::*,
};

#[component]
pub fn Pagination(
    /// Size of the pagination controls
    #[prop(optional, into)] size: Signal<PaginationSize>,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Node reference to the container element
    #[prop(optional)] node_ref: NodeRef<Div>,
    /// Child pagination controls
    children: Children,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            class=move || {
                merge_classes!("join",
                size.get().as_str(),
                class)
            }
        >
            {children()}
        </div>
    }
}

/// # Pagination Button Component
///
/// A reactive Leptos wrapper for pagination button controls with active state and click handling.
///
/// ## Node References
/// - `node_ref` - References the button element ([HTMLButtonElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement))

#[component]
pub fn PaginationButton(
    /// Whether the button is in active state
    #[prop(optional, into)] active: Signal<bool>,
    /// Whether the button is disabled
    #[prop(optional, into)] disabled: Signal<bool>,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Node reference to the button element
    #[prop(optional)] node_ref: NodeRef<Button>,
    /// Click event handler
    #[prop(optional)] on_click: Option<Box<dyn Fn()>>,
    /// Button content
    children: Children,
) -> impl IntoView {
    view! {
        <button
            node_ref=node_ref
            class=move || merge_classes!("join-item", "btn", class)
            class:btn-active=active
            prop:disabled=disabled
            on:click=move |_| {
                if let Some(handler) = &on_click {
                    handler();
                }
            }
        >
            {children()}
        </button>
    }
}

/// # Pagination Input Component
///
/// A reactive Leptos wrapper for pagination input controls that allow direct page navigation.
///
/// ## Node References
/// - `node_ref` - References the input element ([HTMLInputElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement))

#[component]
pub fn PaginationInput(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Node reference to the input element
    #[prop(optional)] node_ref: NodeRef<Input>,
    /// Current input value
    #[prop(optional, into)] value: Signal<String>,
    /// Input change event handler
    #[prop(optional)] on_input: Option<Box<dyn Fn(String)>>,
) -> impl IntoView {
    view! {
        <input
            node_ref=node_ref
            class=move || merge_classes!("join-item", "btn", class)
            type="text"
            prop:value=value
            on:input=move |ev| {
                if let Some(handler) = &on_input {
                    handler(event_target_value(&ev));
                }
            }
        />
    }
}
