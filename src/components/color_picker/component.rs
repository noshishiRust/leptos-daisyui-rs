use crate::merge_classes;
use leptos::{html::Div, prelude::*};

/// # ColorPicker Component
///
/// A color selection interface using HTML5 color input.
/// Built on daisyUI's input styling.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("input input-bordered");
/// ```
///
/// ## Node References
/// - `node_ref` - References the container div element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn ColorPicker(
    /// Current color value (hex format: #RRGGBB)
    #[prop(optional, into)]
    value: Signal<String>,

    /// Callback when color changes
    #[prop(optional, into)]
    on_change: Option<Callback<String>>,

    /// Whether the color picker is disabled
    #[prop(optional, into)]
    disabled: Signal<bool>,

    /// Show color value as text
    #[prop(optional, into)]
    show_value: Signal<bool>,

    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the container div
    #[prop(optional)]
    node_ref: NodeRef<Div>,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("flex items-center gap-2", class)>
            <input
                type="color"
                value=move || value.get()
                disabled=disabled
                class="input input-bordered w-20 h-12 p-1 cursor-pointer"
                on:input=move |ev| {
                    let val = event_target_value(&ev);
                    if let Some(ref callback) = on_change {
                        callback.run(val);
                    }
                }
            />
            {move || {
                if show_value.get() {
                    view! {
                        <span class="font-mono text-sm">{value.get()}</span>
                    }
                        .into_any()
                } else {
                    ().into_any()
                }
            }}

        </div>
    }
}
