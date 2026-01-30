use crate::merge_classes;
use leptos::{html::Div, prelude::*};

/// # AutoComplete Component
///
/// An input field with auto-completion suggestions displayed in a dropdown.
/// Built on daisyUI's dropdown and input components.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("input dropdown dropdown-content menu bg-base-200 rounded-box z-[1] w-full shadow");
/// ```
///
/// ## Node References
/// - `node_ref` - References the container div element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn AutoComplete(
    /// Current input value
    #[prop(optional, into)]
    value: Signal<String>,

    /// List of suggestions to display
    #[prop(optional, into)]
    suggestions: Signal<Vec<String>>,

    /// Placeholder text
    #[prop(optional, into)]
    placeholder: Signal<String>,

    /// Callback when a suggestion is selected
    #[prop(optional, into)]
    on_select: Option<Callback<String>>,

    /// Callback when input value changes
    #[prop(optional, into)]
    on_input: Option<Callback<String>>,

    /// Whether the autocomplete is disabled
    #[prop(optional, into)]
    disabled: Signal<bool>,

    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the container div
    #[prop(optional)]
    node_ref: NodeRef<Div>,
) -> impl IntoView {
    let (show_suggestions, set_show_suggestions) = signal(false);

    // Filter suggestions based on current value
    let filtered_suggestions = Signal::derive(move || {
        let val = value.get().to_lowercase();
        if val.is_empty() {
            Vec::new()
        } else {
            suggestions
                .get()
                .into_iter()
                .filter(|s| s.to_lowercase().contains(&val))
                .take(10)
                .collect()
        }
    });

    view! {
        <div
            node_ref=node_ref
            class=move || merge_classes!("dropdown w-full", class)
        >
            <input
                type="text"
                placeholder=move || placeholder.get()
                value=move || value.get()
                disabled=disabled
                class="input input-bordered w-full"
                on:input=move |ev| {
                    let val = event_target_value(&ev);
                    set_show_suggestions.set(!val.is_empty());
                    if let Some(ref callback) = on_input {
                        callback.run(val.clone());
                    }
                }

                on:focus=move |_| {
                    set_show_suggestions.set(!value.get().is_empty());
                }

                on:blur=move |_| {
                    // Delay to allow click on suggestion
                    set_timeout(
                        move || {
                            set_show_suggestions.set(false);
                        },
                        std::time::Duration::from_millis(200),
                    );
                }
            />

            {move || {
                if show_suggestions.get() && !filtered_suggestions.get().is_empty() {
                    view! {
                        <ul
                            tabindex="0"
                            class="dropdown-content menu bg-base-200 rounded-box z-[1] w-full p-2 shadow max-h-60 overflow-auto"
                        >
                            {filtered_suggestions
                                .get()
                                .into_iter()
                                .map(|suggestion| {
                                    let suggestion_clone = suggestion.clone();
                                    view! {
                                        <li>
                                            <a
                                                on:click=move |_| {
                                                    if let Some(ref callback) = on_select {
                                                        callback.run(suggestion.clone());
                                                    }
                                                    set_show_suggestions.set(false);
                                                }
                                            >

                                                {suggestion_clone}
                                            </a>
                                        </li>
                                    }
                                })
                                .collect_view()}

                        </ul>
                    }
                        .into_any()
                } else {
                    ().into_any()
                }
            }}

        </div>
    }
}
