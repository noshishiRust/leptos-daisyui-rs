use super::style::FieldState;
use crate::merge_classes;
use leptos::{html::Div, prelude::*};

/// # Field Component
///
/// A form field wrapper that provides labels, validation states, error messages,
/// and help text for form inputs. Wraps any form control with consistent styling
/// and validation feedback.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("form-control label label-text label-text-alt text-error text-success text-warning");
/// ```
///
/// ## Node References
/// - `node_ref` - References the container div element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn Field(
    /// Label text displayed above the field
    #[prop(optional, into)]
    label: Signal<Option<String>>,

    /// Help text displayed below the field
    #[prop(optional, into)]
    help_text: Signal<Option<String>>,

    /// Error message displayed when state is Error
    #[prop(optional, into)]
    error: Signal<Option<String>>,

    /// Success message displayed when state is Success
    #[prop(optional, into)]
    success: Signal<Option<String>>,

    /// Validation state (Default, Error, Success, Warning)
    #[prop(optional, into)]
    state: Signal<FieldState>,

    /// Whether the field is required
    #[prop(optional, into)]
    required: Signal<bool>,

    /// Additional CSS classes for the container
    #[prop(optional, into)]
    class: &'static str,

    /// Additional CSS classes for the label
    #[prop(optional, into)]
    label_class: &'static str,

    /// Node reference for the container div
    #[prop(optional)]
    node_ref: NodeRef<Div>,

    /// The form control (input, select, textarea, etc.) to wrap
    children: Children,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            class=move || merge_classes!("form-control", class)
        >
            {move || {
                label
                    .get()
                    .map(|label_text| {
                        view! {
                            <label class=move || merge_classes!("label", label_class)>
                                <span class="label-text">
                                    {label_text}
                                    {move || {
                                        if required.get() {
                                            view! { <span class="text-error ml-1">"*"</span> }.into_any()
                                        } else {
                                            ().into_any()
                                        }
                                    }}

                                </span>
                            </label>
                        }
                            .into_any()
                    })
            }}

            {children()}

            {move || {
                let current_state = state.get();
                let error_msg = error.get();
                let success_msg = success.get();
                let help = help_text.get();
                if matches!(current_state, FieldState::Error) {
                    if let Some(msg) = error_msg {
                        view! {
                            <label class="label">
                                <span class="label-text-alt text-error">{msg}</span>
                            </label>
                        }
                            .into_any()
                    } else {
                        ().into_any()
                    }
                } else if matches!(current_state, FieldState::Success) {
                    if let Some(msg) = success_msg {
                        view! {
                            <label class="label">
                                <span class="label-text-alt text-success">{msg}</span>
                            </label>
                        }
                            .into_any()
                    } else {
                        ().into_any()
                    }
                } else if matches!(current_state, FieldState::Warning) {
                    view! {
                        <label class="label">
                            <span class="label-text-alt text-warning">
                                {help.unwrap_or_default()}
                            </span>
                        </label>
                    }
                        .into_any()
                } else if let Some(msg) = help {
                    view! {
                        <label class="label">
                            <span class="label-text-alt">{msg}</span>
                        </label>
                    }
                        .into_any()
                } else {
                    ().into_any()
                }
            }}

        </div>
    }
}
