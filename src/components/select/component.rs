use super::style::{SelectColor, SelectSize, SelectStyle};
use crate::merge_classes;
use leptos::{
    html::{Option_, Select as HtmlSelect},
    prelude::*,
};

#[component]
pub fn Select(
    /// The style variant of the select
    #[prop(optional, into)]
    style: Signal<SelectStyle>,

    /// The color variant of the select
    #[prop(optional, into)]
    color: Signal<SelectColor>,

    /// The size variant of the select
    #[prop(optional, into)]
    size: Signal<SelectSize>,

    /// Whether the select is disabled
    #[prop(optional, into)]
    disabled: Signal<bool>,

    /// Additional CSS classes to apply
    #[prop(optional, into)]
    class: &'static str,

    /// Reference to the underlying HTML select element
    #[prop(optional)]
    node_ref: NodeRef<HtmlSelect>,

    /// The child elements (typically SelectOption components)
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

#[component]
pub fn SelectOption(
    /// Whether the option is disabled and cannot be selected
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// Additional CSS classes to apply
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Reference to the underlying HTML option element
    #[prop(optional)]
    node_ref: NodeRef<Option_>,
    /// The content of the option
    children: Children,
) -> impl IntoView {
    view! {
        <option node_ref=node_ref class=class disabled=disabled>
            {children()}
        </option>
    }
}
