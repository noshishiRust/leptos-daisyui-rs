use crate::merge_classes;
use leptos::{html::Div, prelude::*};

/// # TagPicker Component
///
/// A multi-select tag input component that allows adding and removing tags.
/// Built on daisyUI's badge and input components.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("input badge badge-primary flex flex-wrap gap-2");
/// ```
///
/// ## Node References
/// - `node_ref` - References the container div element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn TagPicker(
    /// Current selected tags
    #[prop(optional, into)]
    tags: Signal<Vec<String>>,

    /// Placeholder text for input
    #[prop(optional, into)]
    placeholder: Signal<String>,

    /// Callback when tags change
    #[prop(optional, into)]
    on_change: Option<Callback<Vec<String>>>,

    /// Whether the tag picker is disabled
    #[prop(optional, into)]
    disabled: Signal<bool>,

    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the container div
    #[prop(optional)]
    node_ref: NodeRef<Div>,
) -> impl IntoView {
    let (input_value, set_input_value) = signal(String::new());

    let add_tag = move |tag: String| {
        if !tag.trim().is_empty() && !tags.get().contains(&tag) {
            let mut new_tags = tags.get();
            new_tags.push(tag);
            if let Some(ref callback) = on_change {
                callback.run(new_tags);
            }
            set_input_value.set(String::new());
        }
    };

    let remove_tag = move |index: usize| {
        let mut new_tags = tags.get();
        if index < new_tags.len() {
            new_tags.remove(index);
            if let Some(ref callback) = on_change {
                callback.run(new_tags);
            }
        }
    };

    view! {
        <div node_ref=node_ref class=move || merge_classes!("flex flex-col gap-2", class)>
            <div class="flex flex-wrap gap-2 p-2 border rounded-lg min-h-12">
                {move || {
                    tags.get()
                        .into_iter()
                        .enumerate()
                        .map(|(index, tag)| {
                            view! {
                                <div class="badge badge-primary gap-2">
                                    <span>{tag}</span>
                                    <button
                                        class="btn btn-xs btn-circle btn-ghost"
                                        disabled=disabled
                                        on:click=move |_| remove_tag(index)
                                    >
                                        "\u{00d7}"
                                    </button>
                                </div>
                            }
                        })
                        .collect_view()
                }}

            </div>
            <input
                type="text"
                class="input input-bordered w-full"
                placeholder=move || placeholder.get()
                value=move || input_value.get()
                disabled=disabled
                on:input=move |ev| {
                    set_input_value.set(event_target_value(&ev));
                }

                on:keydown=move |ev| {
                    if ev.key() == "Enter" {
                        ev.prevent_default();
                        add_tag(input_value.get());
                    }
                }
            />
        </div>
    }
}
