use leptos::{attr::Attribute, html, prelude::*};

/// An enhanced image component with lazy loading and fallback support.
///
/// # Example
/// ```rust
/// use leptos::prelude::*;
/// use leptos_daisyui::*;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <Image
///             src="https://example.com/image.jpg"
///             alt="Example image"
///             fallback_src="/fallback.jpg"
///             lazy=true
///         />
///     }
/// }
/// ```
#[component]
pub fn Image(
    /// Image source URL
    #[prop(into)]
    src: Signal<String>,
    /// Alternative text for accessibility
    #[prop(into)]
    alt: Signal<String>,
    /// Fallback image source if main image fails to load
    #[prop(optional, into)]
    fallback_src: Signal<String>,
    /// Enable lazy loading
    #[prop(optional, into)]
    lazy: Signal<bool>,
    /// Image width
    #[prop(optional, into)]
    width: Signal<Option<String>>,
    /// Image height
    #[prop(optional, into)]
    height: Signal<Option<String>>,
    /// Reference to the underlying DOM node. See [MDN docs](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/img)
    #[prop(optional)]
    node_ref: NodeRef<html::Img>,
    /// Spread additional attributes onto the `<img>` element
    #[prop(attrs)]
    attributes: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    let (current_src, set_current_src) = signal(String::new());
    let (is_loading, set_is_loading) = signal(true);
    let (has_error, set_has_error) = signal(false);

    Effect::new(move |_| {
        set_current_src.set(src.get());
        set_is_loading.set(true);
        set_has_error.set(false);
    });

    let loading_attr = move || {
        if lazy.get() {
            Some("lazy")
        } else {
            None
        }
    };

    view! {
        <img
            node_ref=node_ref
            src=move || current_src.get()
            alt=move || alt.get()
            loading=loading_attr
            width=move || width.get()
            height=move || height.get()
            class:opacity-0=move || is_loading.get()
            class:opacity-100=move || !is_loading.get()
            class="transition-opacity duration-300"
            on:load=move |_| {
                set_is_loading.set(false);
            }
            on:error=move |_| {
                set_has_error.set(true);
                let fallback = fallback_src.get();
                if !fallback.is_empty() && current_src.get() != fallback {
                    set_current_src.set(fallback);
                }
            }
            {..attributes}
        />
    }
}
