use crate::merge_classes;
use leptos::{html::Div, prelude::*, tachys::html::class::class as class_fn};

/// # Validator Wrapper Component
///
/// This component itself does not have a container, only a validator class for its child components.
/// As such, it is intended to be used with form elements such as `Input` and `Select`.
#[component]
pub fn Validator(
    /// Form element clildren (suach as input, select, textarea etc...)
    children: Children,
) -> impl IntoView {
    children().add_any_attr(class_fn(("validator", true)));
}

/// # Validator Hint Component
///
/// A hint or error message that displays validation feedback for form fields.
/// This is only visible when the validator's child component has a validity of false.
///
/// ## Node References
/// - `node_ref` - References the hint `<div>` element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn ValidatorHint(
    /// Additional CSS classes to apply to the hint
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the hint `<div>` element
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// Hint or error message content
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("validator-text", class)>
            {children()}
        </div>
    }
}
