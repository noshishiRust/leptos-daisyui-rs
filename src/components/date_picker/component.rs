use crate::merge_classes;
use leptos::{html::Div, prelude::*};

/// # DatePicker Component
///
/// A calendar-based date selection component with month/year navigation.
/// Built on HTML5 date input with daisyUI styling.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("input input-bordered");
/// ```
///
/// ## Node References
/// - `node_ref` - References the container div element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn DatePicker(
    /// Current date value (YYYY-MM-DD format)
    #[prop(optional, into)]
    value: Signal<String>,

    /// Minimum date (YYYY-MM-DD format)
    #[prop(optional, into)]
    min: Signal<Option<String>>,

    /// Maximum date (YYYY-MM-DD format)
    #[prop(optional, into)]
    max: Signal<Option<String>>,

    /// Callback when date changes
    #[prop(optional, into)]
    on_change: Option<Callback<String>>,

    /// Whether the date picker is disabled
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
                type="date"
                class="input input-bordered w-full"
                value=move || value.get()
                min=move || min.get()
                max=move || max.get()
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
