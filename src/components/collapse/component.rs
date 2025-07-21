use super::style::{CollapseForceModifier, CollapseModifier};
use crate::merge_classes;
use leptos::{
    html::{Div, Input},
    prelude::*,
};

/// A collapsible container that can expand and contract to show or hide content.
///
/// The `Collapse` component uses tabindex-based interaction, allowing users to click
/// on the collapse to toggle its state. For more control, use `CollapseCheckbox`.
///
/// /// ### Add to `input.css`
/// ```css
/// @source inline("collapse collapse-title collapse-content collapse-arrow collapse-plus collapse-open collapse-close");
/// ```
///
/// ## Node References
/// - `outer_node_ref` - References the outer `<div>` element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
/// - `inner_node_ref` - References the inner `<input>` element ([HTMLInputElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement))
#[component]
pub fn Collapse(
    /// Whether to focus open or not
    #[prop(optional, into)]
    focus_open: Signal<bool>,

    /// Reactive signal controlling whether the collapse is checked for open
    #[prop(optional, into)]
    checked: Signal<bool>,

    /// Visual style and behavior modifier for the collapse
    #[prop(optional, into)]
    modifier: Signal<CollapseModifier>,

    /// Reactive signal controlling whether the collapse is open/close
    #[prop(optional, into)]
    force: Signal<CollapseForceModifier>,

    /// Additional CSS classes to apply to the collapse container
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the outer container `<div>` element
    #[prop(optional)]
    outer_node_ref: NodeRef<Div>,

    /// Node reference for the internal radio `<input>` element
    /// If focus_open is false, mount.
    #[prop(optional)]
    inner_node_ref: NodeRef<Input>,

    /// Child elements, typically CollapseTitle and CollapseContent
    children: Children,
) -> impl IntoView {
    view! {
        <div
            node_ref=outer_node_ref
            tabindex=move || { if focus_open.get() { Some("0") } else { None } }
            class=move || {
                merge_classes!(
                    "collapse",
                    modifier.get().as_str(),
                    force.get().as_str(),
                    class
                )
            }
        >
            {move || {
                if focus_open.get() {
                    ().into_any()
                } else {
                    view! { <input node_ref=inner_node_ref type="checkbox" checked=checked /> }
                        .into_any()
                }
            }}

            {children()}
        </div>
    }
}

/// The clickable title section of a collapse component.
///
/// This component renders the header/title area that users click to toggle
/// the collapse state.
///
/// ## Node References
/// - `node_ref` - References the top `<div>` element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn CollapseTitle(
    /// Additional CSS classes to apply to the title element
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the title `<div>` element
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// Title content (text, icons, etc.)
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("collapse-title", class)>
            {children()}
        </div>
    }
}

/// The collapsible content section of a collapse component.
///
/// This component renders the content that is shown/hidden when the collapse
/// is toggled.
///
/// ## Node References
/// - `node_ref` - References the top `<div>` element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn CollapseContent(
    /// Additional CSS classes to apply to the content element
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the content `<div>` element
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// Content to show/hide when collapse is toggled
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("collapse-content", class)>
            {children()}
        </div>
    }
}
