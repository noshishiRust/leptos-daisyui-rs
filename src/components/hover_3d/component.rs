use crate::merge_classes;
use leptos::{ev, html::Div, prelude::*};
use wasm_bindgen::JsCast;
use web_sys;

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
    let internal_ref = NodeRef::<Div>::new();
    let combined_ref = if node_ref.get_untracked().is_some() {
        node_ref
    } else {
        internal_ref
    };

    // Handle mousemove for 3D tilt effect
    let handle_mouse_move = move |e: ev::MouseEvent| {
        if let Some(element) = combined_ref.get_untracked() {
            // Use as_ref to get the Element reference
            let elem = element.unchecked_ref::<web_sys::Element>();
            let rect = elem.get_bounding_client_rect();
            let x = e.client_x() as f64 - rect.left();
            let y = e.client_y() as f64 - rect.top();
            let center_x = rect.width() / 2.0;
            let center_y = rect.height() / 2.0;

            // Calculate rotation angles (-15 to +15 degrees)
            let rotate_y = ((x - center_x) / center_x) * 15.0;
            let rotate_x = ((center_y - y) / center_y) * 15.0;

            // Apply transform to content children (skip first 8 hover zone divs)
            let children = element.children();
            for i in 8..children.length() {
                if let Some(child) = children.item(i)
                    && let Some(html_elem) = child.dyn_ref::<web_sys::HtmlElement>()
                {
                    let _ = html_elem.style().set_property(
                        "transform",
                        &format!("rotateX({}deg) rotateY({}deg)", rotate_x, rotate_y),
                    );
                }
            }
        }
    };

    // Handle mouseleave to reset transform
    let handle_mouse_leave = move |_e: ev::MouseEvent| {
        if let Some(element) = combined_ref.get_untracked() {
            let children = element.children();
            for i in 8..children.length() {
                if let Some(child) = children.item(i)
                    && let Some(html_elem) = child.dyn_ref::<web_sys::HtmlElement>()
                {
                    let _ = html_elem
                        .style()
                        .set_property("transform", "rotateX(0deg) rotateY(0deg)");
                }
            }
        }
    };

    view! {
        <div
            node_ref=combined_ref
            class=move || merge_classes!("hover-3d", class)
            on:mousemove=handle_mouse_move
            on:mouseleave=handle_mouse_leave
        >
            // Eight empty divs for hover zones (required for CSS)
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
