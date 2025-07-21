use super::style::AccordionModifier;
use crate::merge_classes;
use leptos::{
    html::{Div, Input},
    prelude::*,
};

/// # Accordion Component
///
/// A reactive Leptos wrapper for daisyUI's accordion/collapse component that allows for collapsible content sections.
/// When multiple accordions share the same radio input name, only one can be open at a time, creating traditional
/// accordion behavior.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("collapse collapse-title collapse-content collapse-arrow collapse-plus collapse-open collapse-close");
/// ```
///
/// ## Node References
/// - `outer_node_ref` - References the outer `<div>` element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
/// - `inner_node_ref` - References the inner `<input>` element ([HTMLInputElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement))
#[component]
pub fn Accordion(
    /// Radio input name for grouping accordion sections
    ///
    /// When multiple accordions share the same name, only one can be open at a time.
    /// Each group should have a unique name to avoid conflicts with other accordion groups.
    #[prop(optional)]
    name: Option<&'static str>,

    /// Reactive signal controlling whether the accordion is open/checked
    #[prop(optional, into)]
    checked: Signal<bool>,

    /// Visual modifier for the accordion appearance
    #[prop(optional, into)]
    modifier: Signal<AccordionModifier>,

    /// Additional CSS classes to apply to the accordion container
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the outer container `<div>` element
    #[prop(optional)]
    outer_node_ref: NodeRef<Div>,

    /// Node reference for the internal radio `<input>` element
    #[prop(optional)]
    inner_node_ref: NodeRef<Input>,

    /// Child components, typically [`AccordionTitle`] and [`AccordionContent`]
    children: Children,
) -> impl IntoView {
    view! {
        <div
            node_ref=outer_node_ref
            class=move || {
                merge_classes!(
                    "collapse",
                    modifier.get().as_str(),
                    class
                )
            }
        >
            <input node_ref=inner_node_ref type="radio" name=name prop:checked=checked />
            {children()}
        </div>
    }
}

/// # Accordion Title Component
///
/// A clickable title/header section for accordion content. This component provides the interactive
/// element that users click to expand or collapse the accordion section.
///
/// ## Node References
/// - `node_ref` - References the top `<div>` element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn AccordionTitle(
    /// Additional CSS classes to apply to the title element
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the title `<div>` element
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// Title content (text, icons, or other elements)
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("collapse-title", class)>
            {children()}
        </div>
    }
}

/// # Accordion Content Component
///
/// The collapsible content section of an accordion. This component contains the content that
/// is shown or hidden when the accordion is expanded or collapsed.
///
/// ## Node References
/// - `node_ref` - References the top `<div>` element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn AccordionContent(
    /// Additional CSS classes to apply to the content element
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the content `<div>` element
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// Content to display when the accordion is expanded
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("collapse-content", class)>
            {children()}
        </div>
    }
}
