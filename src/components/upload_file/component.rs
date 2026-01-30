use crate::merge_classes;
use leptos::{html::Div, prelude::*};

/// # UploadFile Component
///
/// A file upload component with styled file input.
/// Built on daisyUI's file-input styling.
///
/// ### Add to `input.css`
/// ```css
/// @source inline("file-input file-input-bordered card bg-base-200");
/// ```
///
/// ## Node References
/// - `node_ref` - References the container div element ([HTMLDivElement](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement))
#[component]
pub fn UploadFile(
    /// Callback when files are selected
    #[prop(optional, into)]
    on_upload: Option<Callback<String>>,

    /// Whether multiple files can be uploaded
    #[prop(optional, into)]
    multiple: Signal<bool>,

    /// Accepted file types (e.g., "image/*", ".pdf")
    #[prop(optional, into)]
    accept: Signal<String>,

    /// Whether the upload is disabled
    #[prop(optional, into)]
    disabled: Signal<bool>,

    /// Additional CSS classes
    #[prop(optional, into)]
    class: &'static str,

    /// Node reference for the container div
    #[prop(optional)]
    node_ref: NodeRef<Div>,
) -> impl IntoView {
    let (selected_file, set_selected_file) = signal(String::new());

    view! {
        <div node_ref=node_ref class=move || merge_classes!("form-control", class)>
            <div class="card bg-base-200 p-6">
                <div class="flex flex-col items-center gap-4">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        class="h-12 w-12 text-base-content/50"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke="currentColor"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"
                        ></path>
                    </svg>

                    <div class="text-center">
                        <p class="text-lg font-semibold">"Click to upload"</p>
                        {move || {
                            let file = selected_file.get();
                            if !file.is_empty() {
                                view! {
                                    <div class="mt-2">
                                        <p class="text-sm text-base-content/70">
                                            "Selected: " {file}
                                        </p>
                                    </div>
                                }
                                    .into_any()
                            } else {
                                ().into_any()
                            }
                        }}

                    </div>

                    <input
                        type="file"
                        class="file-input file-input-bordered w-full max-w-xs"
                        multiple=move || multiple.get()
                        accept=move || {
                            let a = accept.get();
                            if a.is_empty() { None } else { Some(a) }
                        }

                        disabled=disabled
                        on:change=move |ev| {
                            let val = event_target_value(&ev);
                            let filename = val
                                .split('\\')
                                .next_back()
                                .unwrap_or(&val)
                                .to_string();
                            set_selected_file.set(filename.clone());
                            if let Some(ref callback) = on_upload {
                                callback.run(filename);
                            }
                        }
                    />
                </div>
            </div>
        </div>
    }
}
