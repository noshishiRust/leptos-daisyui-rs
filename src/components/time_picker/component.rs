use crate::merge_classes;
use leptos::{html::Div, prelude::*};

/// # TimePicker Component
///
/// A time selection interface with hour and minute controls.
/// Built on HTML5 time input with daisyUI styling.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("input input-bordered");
/// ```
///
/// ## Node References
/// - `node_ref` - References the container div element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn TimePicker(
    /// Current time value (HH:MM format, 24-hour)
    #[prop(optional, into)]
    value: Signal<String>,

    /// Minimum time (HH:MM format)
    #[prop(optional, into)]
    min: Signal<Option<String>>,

    /// Maximum time (HH:MM format)
    #[prop(optional, into)]
    max: Signal<Option<String>>,

    /// Step in seconds (default: 60 for 1-minute intervals)
    #[prop(optional, into)]
    step: Signal<Option<u32>>,

    /// Callback when time changes
    #[prop(optional, into)]
    on_change: Option<Callback<String>>,

    /// Whether the time picker is disabled
    #[prop(optional, into)]
    disabled: Signal<bool>,

    /// Placeholder text
    #[prop(optional, into)]
    placeholder: Signal<String>,

    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the container div
    #[prop(optional)]
    node_ref: NodeRef<Div>,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=move || merge_classes!("form-control", class)>
            <input
                type="time"
                class="input input-bordered w-full"
                value=move || value.get()
                min=move || min.get()
                max=move || max.get()
                step=move || step.get().map(|s| s.to_string())
                placeholder=move || placeholder.get()
                disabled=disabled
                on:input=move |ev| {
                    let val = event_target_value(&ev);
                    if let Some(ref callback) = on_change {
                        callback.run(val);
                    }
                }
            />
        </div>
    }
}
