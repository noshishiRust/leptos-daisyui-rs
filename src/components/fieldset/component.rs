use crate::merge_classes;
use leptos::{
    html::{Fieldset, Legend, P},
    prelude::*,
};

/// # Fieldset Component
///
/// A container for grouping related form elements with a legend title
/// and optional description label.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("fieldset fieldset-legend label");
/// ```
///
/// ## Node References
/// - `node_ref` - References the `<fieldset>` element ([HTMLFieldSetElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement))
#[component]
pub fn FieldSet(
    /// Additional CSS classes to apply to the fieldset
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the `<fieldset>` element
    #[prop(optional)] 
    node_ref: NodeRef<Fieldset>,

    /// Child elements including [`FieldsetLegend`], form controls, and [`FieldsetLabel`]
    children: Children,
) -> impl IntoView {
    view! {
        <fieldset node_ref=node_ref class=move || merge_classes!("fieldset", class)>
            {children()}
        </fieldset>
    }
}

/// # Fieldset Legend Component
///
/// The title/heading for a fieldset that describes the group of form elements.
///
/// ## Node References
/// - `node_ref` - References the `<legend>` element ([HTMLLegendElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLegendElement))
#[component]
pub fn FieldsetLegend(
    /// Additional CSS classes to apply to the legend
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the `<legend>` element
    #[prop(optional)] 
    node_ref: NodeRef<Legend>,

    /// Legend text or content
    children: Children,
) -> impl IntoView {
    view! {
        <legend node_ref=node_ref class=move || merge_classes!("fieldset-legend", class)>
            {children()}
        </legend>
    }
}

/// # Fieldset Label Component
///
/// A descriptive label or help text for the fieldset, typically placed
/// after the form elements.
///
/// ## Node References
/// - `node_ref` - References the `<p>` element ([HTMLParagraphElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParagraphElement))
#[component]
pub fn FieldsetLabel(
    /// Additional CSS classes to apply to the label
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the `<p>` element
    #[prop(optional)] 
    node_ref: NodeRef<P>,

    /// Descriptive text or help content
    children: Children,
) -> impl IntoView {
    view! {
        <p node_ref=node_ref class=move || merge_classes!("label", class)>
            {children()}
        </p>
    }
}
