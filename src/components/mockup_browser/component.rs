use crate::merge_classes;
use leptos::{html::Div, prelude::*};

/// # Mockup Browser Component
///
/// A reactive Leptos wrapper for daisyUI's mockup browser component that provides
/// a styled web browser frame for displaying web interfaces.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("mockup-browser mockup-browser-toolbar");
/// ```
///
/// ## Node References
/// - `node_ref` - References the div element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn MockupBrowser(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference to the div element
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// Content displayed within the browser frame
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("mockup-browser", class)>
            {children()}
        </div>
    }
}

/// # Mockup Browser Toolbar Component
///
/// A reactive Leptos wrapper for daisyUI's mockup browser toolbar component that provides
/// a styled browser toolbar area.
///
/// ## Node References
/// - `node_ref` - References the div element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn MockupBrowserToolbar(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference to the div element
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// Content displayed within the browser toolbar
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("mockup-browser-toolbar", class)>
            {children()}
        </div>
    }
}
