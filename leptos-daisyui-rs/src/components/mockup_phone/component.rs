use crate::merge_classes;
use leptos::{html::Div, prelude::*};

/// # Mockup Phone Component
///
/// A reactive Leptos wrapper for daisyUI's mockup phone component that provides
/// a styled mobile phone frame for displaying mobile interfaces.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("mockup-phone mockup-phone-camera mockup-phone-display camera display artboard");
/// ```
///
/// ## Node References
/// - `node_ref` - References the div element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn MockupPhone(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference to the div element
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// Content displayed within the phone screen
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || { merge_classes!("mockup-phone", class) }>
            {children()}
        </div>
    }
}

/// # Mockup Phone Camera Component
///
/// A reactive Leptos wrapper for daisyUI's mockup phone camera component that provides
/// a styled mobile phone frame with camera cutout.
///
/// ## Node References
/// - `node_ref` - References the div element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn MockupPhoneCamera(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference to the div element
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// Content displayed within the phone screen
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || { merge_classes!("mockup-phone-camera", class) }>
            {children()}
        </div>
    }
}

/// # Mockup Phone Display Component
///
/// A reactive Leptos wrapper for daisyUI's mockup phone display component that provides
/// a styled phone display area.
///
/// ## Node References
/// - `node_ref` - References the div element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn MockupPhoneDisplay(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference to the div element
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// Content displayed within the phone display
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || { merge_classes!("mockup-phone-display", class) }>
            {children()}
        </div>
    }
}
