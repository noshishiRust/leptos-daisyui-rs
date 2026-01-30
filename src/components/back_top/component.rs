use crate::merge_classes;
use leptos::{html::Button, prelude::*};

/// # BackTop Component
///
/// A floating "back to top" button that appears when scrolling down the page.
/// Smoothly scrolls the page back to the top when clicked.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("btn btn-circle fixed bottom-8 right-8 opacity-0 transition-opacity");
/// ```
///
/// ## Node References
/// - `node_ref` - References the button element ([HTMLButtonElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement))
#[component]
pub fn BackTop(
    /// Scroll threshold to show button (pixels from top)
    #[prop(optional, into)]
    threshold: Signal<f64>,

    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the button
    #[prop(optional)]
    node_ref: NodeRef<Button>,

    /// Optional custom content (defaults to up arrow)
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let (is_visible, set_is_visible) = signal(false);

    let threshold = Signal::derive(move || {
        let t = threshold.get();
        if t == 0.0 {
            300.0
        } else {
            t
        }
    });

    // Check scroll position on mount
    Effect::new(move |_| {
        #[cfg(target_arch = "wasm32")]
        {
            use leptos::wasm_bindgen::JsCast;

            let handle_scroll = move || {
                if let Some(window) = web_sys::window() {
                    let scroll_y = window.scroll_y().unwrap_or(0.0);
                    set_is_visible.set(scroll_y > threshold.get());
                }
            };

            // Initial check
            handle_scroll();

            // Add scroll listener
            if let Some(window) = web_sys::window() {
                let closure = leptos::wasm_bindgen::closure::Closure::wrap(Box::new(move || {
                    handle_scroll();
                }) as Box<dyn Fn()>);

                let _ = window.add_event_listener_with_callback(
                    "scroll",
                    closure.as_ref().unchecked_ref(),
                );
                closure.forget();
            }
        }
    });

    let scroll_to_top = move |_| {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                let _ = window.scroll_to_with_x_and_y(0.0, 0.0);
            }
        }
    };

    view! {
        <button
            node_ref=node_ref
            class=move || {
                let base_classes = "btn btn-circle fixed bottom-8 right-8 transition-opacity";
                let visibility = if is_visible.get() {
                    "opacity-100"
                } else {
                    "opacity-0 pointer-events-none"
                };
                merge_classes!(base_classes, visibility, class)
            }

            on:click=scroll_to_top
        >
            {if let Some(content) = children {
                content().into_any()
            } else {
                view! {
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        class="h-6 w-6"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke="currentColor"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M5 10l7-7m0 0l7 7m-7-7v18"
                        ></path>
                    </svg>
                }
                    .into_any()
            }}

        </button>
    }
}
