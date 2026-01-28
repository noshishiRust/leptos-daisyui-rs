use crate::merge_classes;
use leptos::html::{Div, Li, Ul};
use leptos::prelude::*;

/// # Breadcrumbs Component
///
/// A reactive Leptos wrapper for daisyUI's breadcrumbs component that provides navigation
/// hierarchy display with automatic separator styling.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("breadcrumbs");
/// ```
///
/// ## Node References
/// - `outer_node_ref` - References the top `<div>` element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
/// - `inner_node_ref` - References the inner `<ul>` element ([HTMLUlElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLUlElement))
#[component]
pub fn Breadcrumbs(
    /// Additional CSS classes for the outer container
    #[prop(optional, into)]
    outer_class: &'static str,

    /// Node reference for the outer `<div>` element
    #[prop(optional)]
    outer_node_ref: NodeRef<Div>,

    /// Additional CSS classes for the inner `<ul>` element
    #[prop(optional, into)]
    inner_class: &'static str,

    /// Node reference for the inner `<ul>` element
    #[prop(optional)]
    inner_node_ref: NodeRef<Ul>,

    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=outer_node_ref class=move || merge_classes!("breadcrumbs", outer_class)>
            <ul node_ref=inner_node_ref class=inner_class>
                {children()}
            </ul>
        </div>
    }
}

/// # BreadcrumbItem Component
///
/// ## Node References
/// - `node_ref` - References the top `<li>` element ([HTMLLiElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLiElement))
#[component]
pub fn BreadcrumbItem(
    /// Optional link URL for the breadcrumb item
    #[prop(optional, into)]
    href: MaybeProp<String>,

    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the breadcrumb item `<li>` element
    #[prop(optional)]
    node_ref: NodeRef<Li>,

    /// Child components, typically text or other elements
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    view! {
        <li node_ref=node_ref class=class>
            {if let Some(href) = href.get() {
                view! { <a href=href>{children.map(|v| v())}</a> }.into_any()
            } else {
                view! { {children.map(|v| v())} }.into_any()
            }}
        </li>
    }
}
