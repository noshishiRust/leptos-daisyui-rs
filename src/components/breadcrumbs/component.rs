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
/// ## Usage Example
///
/// ```rust
/// use leptos::*;
/// use leptos_daisyui::breadcrumbs::*;
///
/// #[component]
/// fn BreadcrumbsDemo() -> impl IntoView {
///     view! {
///         <Breadcrumbs outer_class="text-sm">
///             <BreadcrumbItem href="/".to_string()>
///                 "Home"
///             </BreadcrumbItem>
///             <BreadcrumbItem href="/documents".to_string()>
///                 "Documents"
///             </BreadcrumbItem>
///             <BreadcrumbItem>
///                 "Current Page"
///             </BreadcrumbItem>
///         </Breadcrumbs>
///     }
/// }
/// ```
///
/// ## Styling Notes
///
/// - Separators are automatically added between items via CSS pseudo-elements
/// - Works well with Tailwind utilities like `text-sm` for responsive sizing
/// - Can be combined with color utilities for theming
/// - The last breadcrumb item should typically not be a clickable link
///
/// For more information, see: https://daisyui.com/components/breadcrumbs/
#[component]
pub fn Breadcrumbs(
    /// Additional CSS classes for the outer container
    #[prop(optional, into)]
    outer_class: &'static str,

    /// Node reference for the outer `<div>` element
    #[prop(optional, into)]
    outer_node_ref: NodeRef<Div>,

    /// Additional CSS classes for the inner `<ul>` element
    #[prop(optional, into)]
    inner_class: &'static str,

    /// Node reference for the inner `<ul>` element
    #[prop(optional, into)]
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
/// A reactive Leptos wrapper for individual breadcrumb items within a breadcrumbs navigation.
/// Renders as a list item that can contain either a clickable link or plain text.
#[component]
pub fn BreadcrumbItem(
    /// Optional link URL for the breadcrumb item
    #[prop(optional, into)]
    href: MaybeProp<String>,

    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the breadcrumb item `<li>` element
    #[prop(optional, into)]
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
