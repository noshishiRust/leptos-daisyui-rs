use leptos::prelude::*;
use crate::theme::use_theme_context;

#[cfg(target_arch = "wasm32")]
use {
    wasm_bindgen::closure::Closure,
    wasm_bindgen::JsCast,
    web_sys::{Event, HtmlInputElement},
};

/// Theme Export/Import Component
///
/// Provides UI controls for exporting the current theme configuration to JSON
/// and importing theme configurations from JSON files.
///
/// ## Features
/// - Export current theme as JSON file
/// - Import theme from JSON file with validation
/// - Copy theme JSON to clipboard
/// - Visual feedback for success/error states
///
/// ## Example
/// ```rust,ignore
/// use leptos::prelude::*;
/// use leptos_daisyui_rs::components::ThemeExportImport;
/// use leptos_daisyui_rs::theme::ThemeProvider;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <ThemeProvider load_from_storage=true>
///             <ThemeExportImport />
///         </ThemeProvider>
///     }
/// }
/// ```
#[component]
pub fn ThemeExportImport() -> impl IntoView {
    let theme_ctx = use_theme_context();

    let (import_status, set_import_status) = signal::<Option<String>>(None);
    let (export_status, set_export_status) = signal::<Option<String>>(None);

    // Export theme to JSON file
    let handle_export = move |_| {
        match theme_ctx.export_theme() {
            Ok(json) => {
                // Create and download file
                #[cfg(target_arch = "wasm32")]
                {
                    use crate::theme::download_theme;
                    let config = theme_ctx.config.get();
                    match download_theme(&config, Some("my-theme.json")) {
                        Ok(_) => set_export_status.set(Some("Theme exported successfully!".to_string())),
                        Err(e) => set_export_status.set(Some(format!("Export failed: {}", e))),
                    }
                }
                #[cfg(not(target_arch = "wasm32"))]
                {
                    let _ = json;
                    set_export_status.set(Some("Export not available on this platform".to_string()));
                }
            }
            Err(e) => {
                set_export_status.set(Some(format!("Export failed: {}", e)));
            }
        }

        // Clear status after 3 seconds
        set_timeout(
            move || set_export_status.set(None),
            std::time::Duration::from_secs(3),
        );
    };

    // Copy theme JSON to clipboard
    let handle_copy = move |_| {
        match theme_ctx.export_theme() {
            Ok(json) => {
                #[cfg(target_arch = "wasm32")]
                {
                    use wasm_bindgen::JsCast;
                    if let Some(window) = web_sys::window() {
                        if let Some(navigator) = window.navigator().clipboard() {
                            let promise = navigator.write_text(&json);
                            wasm_bindgen_futures::spawn_local(async move {
                                match wasm_bindgen_futures::JsFuture::from(promise).await {
                                    Ok(_) => set_export_status.set(Some("Copied to clipboard!".to_string())),
                                    Err(_) => set_export_status.set(Some("Failed to copy to clipboard".to_string())),
                                }
                            });
                        }
                    }
                }
                #[cfg(not(target_arch = "wasm32"))]
                {
                    let _ = json;
                    set_export_status.set(Some("Clipboard not available on this platform".to_string()));
                }
            }
            Err(e) => {
                set_export_status.set(Some(format!("Failed to generate JSON: {}", e)));
            }
        }

        // Clear status after 3 seconds
        set_timeout(
            move || set_export_status.set(None),
            std::time::Duration::from_secs(3),
        );
    };

    // Import theme from file
    #[cfg(target_arch = "wasm32")]
    let handle_import = move |ev: Event| {
        use web_sys::FileList;

        if let Some(target) = ev.target() {
            if let Ok(input) = target.dyn_into::<HtmlInputElement>() {
                if let Some(files) = input.files() {
                    if let Some(file) = files.get(0) {
                        let file_reader = match web_sys::FileReader::new() {
                            Ok(reader) => reader,
                            Err(_) => {
                                set_import_status.set(Some("Failed to create FileReader".to_string()));
                                return;
                            }
                        };

                        let fr_clone = file_reader.clone();
                        let set_status_clone = set_import_status;

                        let onload = Closure::wrap(Box::new(move |_: Event| {
                            if let Ok(result) = fr_clone.result() {
                                if let Some(text) = result.as_string() {
                                    match theme_ctx.import_theme(&text) {
                                        Ok(_) => {
                                            set_status_clone.set(Some("Theme imported successfully!".to_string()));
                                        }
                                        Err(e) => {
                                            set_status_clone.set(Some(format!("Import failed: {}", e)));
                                        }
                                    }

                                    // Clear status after 3 seconds
                                    set_timeout(
                                        move || set_status_clone.set(None),
                                        std::time::Duration::from_secs(3),
                                    );
                                }
                            }
                        }) as Box<dyn FnMut(_)>);

                        file_reader.set_onload(Some(onload.as_ref().unchecked_ref()));
                        onload.forget();

                        if let Err(e) = file_reader.read_as_text(&file) {
                            set_import_status.set(Some(format!("Failed to read file: {:?}", e)));
                        }
                    }
                }
            }
        }
    };

    #[cfg(not(target_arch = "wasm32"))]
    let handle_import = move |_ev: leptos::ev::Event| {
        set_import_status.set(Some("Import not available on this platform".to_string()));
    };

    view! {
        <div class="space-y-6">
            <div>
                <h3 class="text-lg font-semibold mb-2">"Import / Export"</h3>
                <p class="text-sm opacity-70">
                    "Save your theme configuration or load a previously saved theme."
                </p>
            </div>

            // Export Section
            <div class="space-y-2">
                <label class="label">
                    <span class="label-text font-medium">"Export Theme"</span>
                </label>
                <div class="flex gap-2">
                    <button
                        class="btn btn-primary"
                        on:click=handle_export
                    >
                        "Download JSON"
                    </button>
                    <button
                        class="btn btn-ghost"
                        on:click=handle_copy
                    >
                        "Copy to Clipboard"
                    </button>
                </div>
                {move || export_status.get().map(|msg| view! {
                    <div class="alert alert-info mt-2">
                        <span>{msg}</span>
                    </div>
                })}
            </div>

            // Import Section
            <div class="space-y-2">
                <label class="label">
                    <span class="label-text font-medium">"Import Theme"</span>
                </label>
                <input
                    type="file"
                    accept=".json"
                    class="file-input file-input-bordered w-full"
                    on:change=handle_import
                />
                {move || import_status.get().map(|msg| view! {
                    <div class="alert alert-info mt-2">
                        <span>{msg}</span>
                    </div>
                })}
            </div>

            // Info Box
            <div class="alert">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="stroke-current shrink-0 w-6 h-6">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                </svg>
                <span class="text-sm">
                    "Theme files are JSON configurations that include your base theme, color overrides, typography settings, and more."
                </span>
            </div>
        </div>
    }
}
