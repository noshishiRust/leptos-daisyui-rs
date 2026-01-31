use crate::theme::use_theme_context;
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use leptos::prelude::*;

/// Theme Share Component
///
/// Provides functionality to share themes via URL encoding. Generates shareable
/// URLs containing the theme configuration encoded as base64.
///
/// ## Features
/// - Generate shareable URL with theme encoded as base64
/// - Copy share URL to clipboard
/// - Load theme from URL parameter
/// - Visual feedback for success/error states
///
/// ## Example
/// ```rust,ignore
/// use leptos::prelude::*;
/// use leptos_daisyui_rs::components::ThemeShare;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <ThemeShare />
///     }
/// }
/// ```
#[component]
pub fn ThemeShare() -> impl IntoView {
    let theme_ctx = use_theme_context();
    let (share_status, set_share_status) = signal::<Option<String>>(None);
    let (share_url, set_share_url) = signal::<String>(String::new());

    // Generate shareable URL
    let generate_share_url = move |_| {
        match theme_ctx.export_theme() {
            Ok(json) => {
                // Encode JSON as base64
                let encoded = URL_SAFE_NO_PAD.encode(json.as_bytes());

                // Get current URL and append theme parameter
                #[cfg(target_arch = "wasm32")]
                {
                    if let Some(window) = web_sys::window() {
                        if let Some(location) = window.location().href().ok() {
                            // Parse URL and add/update theme parameter
                            let separator = if location.contains('?') { "&" } else { "?" };
                            let url = format!("{}{}theme={}", location, separator, encoded);
                            set_share_url.set(url.clone());
                            set_share_status
                                .set(Some(format!("Share URL generated ({} chars)", url.len())));
                        } else {
                            set_share_status.set(Some("Failed to get current URL".to_string()));
                        }
                    }
                }

                #[cfg(not(target_arch = "wasm32"))]
                {
                    let url = format!("?theme={}", encoded);
                    set_share_url.set(url.clone());
                    set_share_status.set(Some("Share URL generated (preview only)".to_string()));
                }
            }
            Err(e) => {
                set_share_status.set(Some(format!("Failed to generate URL: {}", e)));
            }
        }

        // Clear status after 3 seconds
        set_timeout(
            move || set_share_status.set(None),
            std::time::Duration::from_secs(3),
        );
    };

    // Copy share URL to clipboard
    let copy_share_url =
        move |_| {
            let url = share_url.get();

            if url.is_empty() {
                set_share_status.set(Some("Please generate a share URL first".to_string()));
                return;
            }

            #[cfg(target_arch = "wasm32")]
            {
                if let Some(window) = web_sys::window() {
                    let clipboard = window.navigator().clipboard();
                    let promise = clipboard.write_text(&url);
                    wasm_bindgen_futures::spawn_local(async move {
                        match wasm_bindgen_futures::JsFuture::from(promise).await {
                            Ok(_) => set_share_status
                                .set(Some("Share URL copied to clipboard!".to_string())),
                            Err(_) => set_share_status
                                .set(Some("Failed to copy to clipboard".to_string())),
                        }
                    });
                }
            }

            #[cfg(not(target_arch = "wasm32"))]
            {
                set_share_status.set(Some("Clipboard not available on this platform".to_string()));
            }

            // Clear status after 3 seconds
            set_timeout(
                move || set_share_status.set(None),
                std::time::Duration::from_secs(3),
            );
        };

    view! {
        <div class="space-y-6">
            <div>
                <h3 class="text-lg font-semibold mb-2">"Share Theme"</h3>
                <p class="text-sm opacity-70">
                    "Generate a shareable URL containing your theme configuration."
                </p>
            </div>

            <div class="space-y-4">
                // Generate URL button
                <div>
                    <button
                        class="btn btn-primary"
                        on:click=generate_share_url
                    >
                        "Generate Share URL"
                    </button>
                </div>

                // Share URL display and copy
                {move || {
                    let url = share_url.get();
                    if !url.is_empty() {
                        view! {
                            <div class="space-y-2">
                                <label class="label">
                                    <span class="label-text font-medium">"Share URL"</span>
                                </label>
                                <div class="flex gap-2">
                                    <input
                                        type="text"
                                        class="input input-bordered flex-1 font-mono text-sm"
                                        value=url.clone()
                                        readonly=true
                                    />
                                    <button
                                        class="btn btn-ghost"
                                        on:click=copy_share_url
                                    >
                                        "Copy"
                                    </button>
                                </div>
                            </div>
                        }.into_any()
                    } else {
                        let _: () = view! { <></> };
                        ().into_any()
                    }
                }}

                // Status messages
                {move || share_status.get().map(|msg| view! {
                    <div class="alert alert-info">
                        <span>{msg}</span>
                    </div>
                })}

                // Info box
                <div class="alert">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="stroke-current shrink-0 w-6 h-6">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                    </svg>
                    <span class="text-sm">
                        "Share URLs contain your theme configuration encoded in the URL. Recipients can load the theme by visiting the URL."
                    </span>
                </div>
            </div>
        </div>
    }
}
