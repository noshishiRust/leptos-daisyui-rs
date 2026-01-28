use crate::merge_classes;
use leptos::{html::Div, prelude::*};

/// # Hover 3D Component
///
/// A reactive Leptos wrapper for daisyUI's hover-3d component that adds a 3D tilt effect
/// to content based on mouse position. The component uses 8 hover zones to create smooth
/// and responsive 3D rotation as users move their mouse.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("hover-3d");
/// ```
///
/// ## Important Notes
/// - **Only use non-interactive content** inside the hover-3d wrapper
/// - If you want to make the entire card clickable, wrap the whole hover-3d component in a link
/// - **Do NOT put buttons or links inside** the component as the hover zones will prevent interaction
/// - The component requires 8 empty div elements to function (automatically created)
///
/// ## Use Cases
/// - Credit card displays
/// - Image galleries
/// - Product showcases
/// - Portfolio items
///
/// ## Node References
/// - `node_ref` - References the wrapper `<div>` element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn Hover3d(
    /// Additional CSS classes for the wrapper
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the wrapper div element
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// Content to display with 3D hover effect (must be non-interactive)
    children: Children,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("hover-3d", class)>
            // Eight empty divs for hover zones (required by daisyUI)
            <div></div>
            <div></div>
            <div></div>
            <div></div>
            <div></div>
            <div></div>
            <div></div>
            <div></div>

            {children()}
        </div>
    }
}
