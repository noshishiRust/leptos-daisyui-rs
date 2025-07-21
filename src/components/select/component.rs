use super::style::{SelectColor, SelectSize, SelectStyle};
use crate::merge_classes;
use leptos::{
    html::{Option_, Select as HtmlSelect},
    prelude::*,
};

/// # Select Component
///
/// A reactive Leptos wrapper for daisyUI's select component that provides dropdown
/// selection controls with multiple styling and size options.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("select select-ghost select-primary select-secondary select-accent select-info select-success select-warning select-error select-xs select-sm select-md select-lg select-xl");
/// ```
///
/// ## Node References
/// - `node_ref` - References the top select element ([HTMLSelectElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement))
#[component]
pub fn Select(
    /// Style variant of the select
    #[prop(optional, into)]
    style: Signal<SelectStyle>,

    /// Color scheme of the select
    #[prop(optional, into)]
    color: Signal<SelectColor>,

    /// Size of the select
    #[prop(optional, into)]
    size: Signal<SelectSize>,

    /// Whether the select is disabled
    #[prop(optional, into)]
    disabled: Signal<bool>,

    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference to the select element
    #[prop(optional)]
    node_ref: NodeRef<HtmlSelect>,

    /// Child elements (typically SelectOption components)
    children: Children,
) -> impl IntoView {
    view! {
        <select
            node_ref=node_ref
            class=move || {
                merge_classes!(
                    "select",
                style.get().as_str(),
                color.get().as_str(),
                size.get().as_str(),
                class
                )
            }
            disabled=disabled
        >
            {children()}
        </select>
    }
}

/// Option element for Select component.
///
/// ## Node References
/// - `node_ref` - References the top option element ([HTMLOptionElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement))
#[component]
pub fn SelectOption(
    /// Whether the option is disabled
    #[prop(optional, into)]
    disabled: Signal<bool>,

    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference to the option element
    #[prop(optional)]
    node_ref: NodeRef<Option_>,

    /// Content of the option
    children: Children,
) -> impl IntoView {
    view! {
        <option node_ref=node_ref class=class disabled=disabled>
            {children()}
        </option>
    }
}
