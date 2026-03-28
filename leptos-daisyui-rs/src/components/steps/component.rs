use super::style::{StepColor, StepsDirection};
use crate::merge_classes;
use leptos::{
    html::{Li, Ul},
    prelude::*,
};

/// # Steps Component
///
/// A container for displaying sequential steps or progress indicators.
/// Supports horizontal and vertical layouts for multi-step processes.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("steps step steps-vertical steps-horizontal step-primary step-secondary step-accent step-neutral step-info step-success step-warning step-error");
/// ```
///
/// ## Node References
/// - `node_ref` - References the steps `<ul>` element ([HTMLUListElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLUListElement))
#[component]
pub fn Steps(
    /// Direction of the steps layout
    #[prop(optional, into)]
    direction: Signal<StepsDirection>,

    /// Additional CSS classes to apply to the steps container
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the steps `<ul>` element
    #[prop(optional)]
    node_ref: NodeRef<Ul>,

    /// Child [`Step`] components
    children: Children,
) -> impl IntoView {
    view! {
        <ul
            node_ref=node_ref
            class=move || {
                merge_classes!(
                    "steps",
                direction.get().as_str(),
                class
                )
            }
        >
            {children()}
        </ul>
    }
}

/// # Step Component
///
/// An individual step within a steps sequence. Supports color variants
/// and custom content indicators for step numbering or status symbols.
///
/// ## Node References
/// - `node_ref` - References the step `<li>` element ([HTMLLIElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLIElement))
#[component]
pub fn Step(
    /// Color variant indicating step state
    #[prop(optional, into)]
    color: Signal<StepColor>,

    /// Content for the step indicator (numbers, symbols, etc.)
    #[prop(optional, into)]
    data_content: Option<&'static str>,

    /// Additional CSS classes to apply to the step
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the step `<li>` element
    #[prop(optional)]
    node_ref: NodeRef<Li>,

    /// Step content and description
    children: Children,
) -> impl IntoView {
    view! {
        <li
            node_ref=node_ref
            class=move || {
                merge_classes!("step",
                color.get().as_str(),
                class)
            }
            data-content=data_content
        >
            {children()}
        </li>
    }
}
