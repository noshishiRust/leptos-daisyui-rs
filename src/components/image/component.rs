use crate::merge_classes;
use leptos::{html::Img, prelude::*};

/// # Image Component
///
/// Enhanced image component with loading states, error fallback, and lazy loading.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("skeleton w-full h-full");
/// ```
#[component]
pub fn Image(
    #[prop(into)] src: Signal<String>,
    #[prop(optional, into)] alt: Signal<String>,
    #[prop(optional, into)] fallback_src: Signal<String>,
    #[prop(optional, into)] lazy: Signal<bool>,
    #[prop(optional, into)] class: &'static str,
    #[prop(optional)] node_ref: NodeRef<Img>,
) -> impl IntoView {
    let (is_loading, set_is_loading) = signal(true);
    let (has_error, set_has_error) = signal(false);
    let (current_src, set_current_src) = signal(src.get());

    Effect::new(move |_| {
        set_current_src.set(src.get());
        set_is_loading.set(true);
        set_has_error.set(false);
    });

    view! {
        <div class="relative">
            {move || {
                if is_loading.get() && !has_error.get() {
                    view! { <div class="skeleton w-full h-full absolute inset-0"></div> }.into_any()
                } else {
                    ().into_any()
                }
            }}
            <img
                node_ref=node_ref
                src=move || current_src.get()
                alt=move || alt.get()
                loading=move || if lazy.get() { "lazy" } else { "eager" }
                class=move || merge_classes!("w-full", class)
                on:load=move |_| set_is_loading.set(false)
                on:error=move |_| {
                    set_has_error.set(true);
                    set_is_loading.set(false);
                    let fallback = fallback_src.get();
                    if !fallback.is_empty() && current_src.get() != fallback {
                        set_current_src.set(fallback);
                    }
                }
            />
        </div>
    }
}
