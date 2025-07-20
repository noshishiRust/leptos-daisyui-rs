//! Collapse component implementations.

use super::style::CollapseModifier;
use crate::merge_classes;
use leptos::prelude::*;

/// A collapsible container that can expand and contract to show or hide content.
///
/// The `Collapse` component uses tabindex-based interaction, allowing users to click
/// on the collapse to toggle its state. For more control, use `CollapseCheckbox`.
///
/// ## Props
///
/// - `modifier`: Visual style and behavior modifier (`CollapseModifier`)
/// - `class`: Additional CSS classes to apply
/// - `children`: Child elements, typically `CollapseTitle` and `CollapseContent`
///
/// ## Accessibility
///
/// - Uses `tabindex="0"` for keyboard navigation
/// - Click to toggle expand/collapse state
/// - Proper focus management for keyboard users
///
/// ## Examples
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::{Collapse, CollapseTitle, CollapseContent, CollapseModifier};
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <Collapse modifier=CollapseModifier::Arrow>
///             <CollapseTitle>"Frequently Asked Question"</CollapseTitle>
///             <CollapseContent>
///                 <p>"This is the answer to the question."</p>
///             </CollapseContent>
///         </Collapse>
///     }
/// }
/// ```
#[component]
pub fn Collapse(
    /// Visual style and behavior modifier for the collapse
    #[prop(optional, into)]
    modifier: Signal<CollapseModifier>,
    /// Additional CSS classes to apply to the collapse container
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Child elements, typically CollapseTitle and CollapseContent
    children: Children,
) -> impl IntoView {
    view! {
        <div
            tabindex="0"
            class=move || {
                merge_classes!(
                    "collapse",
                modifier.get().as_str(),
                class
                )
            }
        >
            {children()}
        </div>
    }
}

/// A checkbox-controlled collapsible container for programmatic state management.
///
/// Unlike the basic `Collapse`, this variant includes a checkbox input that
/// can be controlled externally for programmatic state management.
///
/// ## Props
///
/// - `modifier`: Visual style and behavior modifier (`CollapseModifier`)
/// - `checked`: Checkbox state for controlling collapse open/closed state
/// - `class`: Additional CSS classes to apply
/// - `children`: Child elements, typically `CollapseTitle` and `CollapseContent`
///
/// ## Examples
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::{CollapseCheckbox, CollapseTitle, CollapseContent, CollapseModifier};
///
/// #[component]
/// fn App() -> impl IntoView {
///     let (expanded, set_expanded) = signal(false);
///     
///     view! {
///         <div>
///             <button on:click=move |_| set_expanded.update(|x| *x = !*x)>
///                 "Toggle Externally"
///             </button>
///             <CollapseCheckbox
///                 modifier=CollapseModifier::Plus
///                 checked=expanded
///             >
///                 <CollapseTitle>"Controlled Section"</CollapseTitle>
///                 <CollapseContent>"This can be controlled externally"</CollapseContent>
///             </CollapseCheckbox>
///         </div>
///     }
/// }
/// ```
#[component]
pub fn CollapseCheckbox(
    /// Visual style and behavior modifier for the collapse
    #[prop(optional, into)]
    modifier: Signal<CollapseModifier>,
    /// Checkbox state controlling whether the collapse is open or closed
    #[prop(optional, into)]
    checked: Signal<bool>,
    /// Additional CSS classes to apply to the collapse container
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Child elements, typically CollapseTitle and CollapseContent
    children: Children,
) -> impl IntoView {
    view! {
        <div class=move || {
            merge_classes!(
                "collapse",
                modifier.get().as_str(),
                class
            )
        }>
            <input type="checkbox" prop:checked=checked />
            {children()}
        </div>
    }
}

/// The clickable title section of a collapse component.
///
/// This component renders the header/title area that users click to toggle
/// the collapse state. It should be used within a `Collapse` or `CollapseCheckbox`.
///
/// ## Props
///
/// - `class`: Additional CSS classes to apply
/// - `children`: Title content (text, icons, etc.)
///
/// ## Examples
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::CollapseTitle;
///
/// view! {
///     <CollapseTitle>
///         <span class="font-bold">"Question: "</span>
///         "How does this work?"
///     </CollapseTitle>
/// }
/// ```
#[component]
pub fn CollapseTitle(
    /// Additional CSS classes to apply to the title element
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Title content (text, icons, etc.)
    children: Children,
) -> impl IntoView {
    view! { <div class=move || merge_classes!("collapse-title", class)>{children()}</div> }
}

/// The collapsible content section of a collapse component.
///
/// This component renders the content that is shown/hidden when the collapse
/// is toggled. It should be used within a `Collapse` or `CollapseCheckbox`.
///
/// ## Props
///
/// - `class`: Additional CSS classes to apply
/// - `children`: Content to show/hide (text, components, etc.)
///
/// ## Examples
///
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::CollapseContent;
///
/// view! {
///     <CollapseContent>
///         <p>"This content will be hidden when collapsed."</p>
///         <p>"It can contain any components or HTML."</p>
///     </CollapseContent>
/// }
/// ```
#[component]
pub fn CollapseContent(
    /// Additional CSS classes to apply to the content element
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,
    /// Content to show/hide when collapse is toggled
    children: Children,
) -> impl IntoView {
    view! { <div class=move || merge_classes!("collapse-content", class)>{children()}</div> }
}
