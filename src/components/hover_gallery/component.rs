use crate::merge_classes;
use leptos::{ev, html::Figure, prelude::*};
use wasm_bindgen::JsCast;
use web_sys;

/// # Hover Gallery Component
///
/// A reactive Leptos wrapper for daisyUI's hover-gallery component. A container of images where
/// the first image is visible by default and other images show up when hovering horizontally.
/// Supports up to 10 images.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("hover-gallery");
/// ```
///
/// ## How It Works
/// The component creates invisible columns for each non-first image. Hovering over these columns
/// horizontally reveals the corresponding image. This is particularly useful for:
/// - E-commerce product displays
/// - Portfolios
/// - Image galleries requiring quick visual comparisons
///
/// ## Node References
/// - `node_ref` - References the `<figure>` element ([HTMLElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement))
#[component]
pub fn HoverGallery(
    /// Additional CSS classes for the gallery container
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the figure element
    #[prop(optional)]
    node_ref: NodeRef<Figure>,

    /// Image elements (supports up to 10 images)
    children: Children,
) -> impl IntoView {
    let internal_ref = NodeRef::<Figure>::new();
    let combined_ref = if node_ref.get().is_some() {
        node_ref
    } else {
        internal_ref
    };

    // Handle mousemove for hover gallery
    let handle_mouse_move = move |e: ev::MouseEvent| {
        if let Some(element) = combined_ref.get() {
            // Cast to Element to access getBoundingClientRect and querySelectorAll
            let elem = element.unchecked_ref::<web_sys::Element>();
            let rect = elem.get_bounding_client_rect();
            let x = e.client_x() as f64 - rect.left();
            let width = rect.width();

            // Get all img elements
            if let Ok(images) = elem.query_selector_all("img") {
                let image_count = images.length();
                if image_count <= 1 {
                    return;
                }

                // Calculate which image should be visible based on horizontal position
                let segment_width = width / (image_count as f64);
                let active_index = (x / segment_width).floor() as u32;
                let active_index = active_index.min(image_count - 1);

                // Show only the active image
                for i in 0..image_count {
                    if let Some(img) = images.item(i)
                        && let Some(html_elem) = img.dyn_ref::<web_sys::HtmlElement>()
                    {
                        if i == active_index {
                            let _ = html_elem.style().set_property("opacity", "1");
                            let _ = html_elem.style().set_property("z-index", "1");
                        } else {
                            let _ = html_elem.style().set_property("opacity", "0");
                            let _ = html_elem.style().set_property("z-index", "0");
                        }
                    }
                }
            }
        }
    };

    // Handle mouseleave to reset to first image
    let handle_mouse_leave = move |_e: ev::MouseEvent| {
        if let Some(element) = combined_ref.get() {
            let elem = element.unchecked_ref::<web_sys::Element>();
            // Reset to show only first image on mouse leave
            if let Ok(images) = elem.query_selector_all("img") {
                for i in 0..images.length() {
                    if let Some(img) = images.item(i)
                        && let Some(html_elem) = img.dyn_ref::<web_sys::HtmlElement>()
                    {
                        if i == 0 {
                            let _ = html_elem.style().set_property("opacity", "1");
                            let _ = html_elem.style().set_property("z-index", "1");
                        } else {
                            let _ = html_elem.style().set_property("opacity", "0");
                            let _ = html_elem.style().set_property("z-index", "0");
                        }
                    }
                }
            }
        }
    };

    view! {
        <figure
            node_ref=combined_ref
            class=move || merge_classes!("hover-gallery", class)
            on:mousemove=handle_mouse_move
            on:mouseleave=handle_mouse_leave
        >
            {children()}
        </figure>
    }
}
