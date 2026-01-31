use crate::theme::{hex_to_oklch, use_theme_context};
use leptos::prelude::*;

/// Color Customizer Component
///
/// Provides UI controls for customizing all semantic color tokens including
/// primary, secondary, accent, neutral, base colors, and state colors.
///
/// ## Features
/// - Color pickers for all daisyUI semantic tokens
/// - Automatic hex to Oklahoma LCH conversion
/// - Live color preview cards
/// - Reset individual colors or all colors
/// - Integration with ThemeProvider
///
/// ## Example
/// ```rust,no_run
/// use leptos::prelude::*;
/// use leptos_daisyui_rs::components::ColorCustomizer;
/// use leptos_daisyui_rs::theme::ThemeProvider;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <ThemeProvider load_from_storage=true>
///             <ColorCustomizer />
///         </ThemeProvider>
///     }
/// }
/// ```
///
/// ### Add to `input.css`
/// ```css
/// @source inline("form-control label input grid");
/// @source inline("btn btn-primary btn-secondary btn-accent");
/// @source inline("badge badge-info badge-success badge-warning badge-error");
/// ```
#[component]
pub fn ColorCustomizer() -> impl IntoView {
    let theme_ctx = use_theme_context();

    // Color token definitions with labels
    let color_tokens = vec![
        // Brand colors
        ("Primary", "primary", "Main brand color"),
        ("Secondary", "secondary", "Secondary brand color"),
        ("Accent", "accent", "Accent/highlight color"),
        ("Neutral", "neutral", "Neutral/gray color"),
        // Base colors
        ("Base 100", "base_100", "Background color"),
        ("Base 200", "base_200", "Secondary background"),
        ("Base 300", "base_300", "Tertiary background"),
        ("Base Content", "base_content", "Text color on base"),
        // State colors
        ("Info", "info", "Information state"),
        ("Success", "success", "Success state"),
        ("Warning", "warning", "Warning state"),
        ("Error", "error", "Error state"),
    ];

    // Helper to get current color value
    let get_color = move |key: &str| -> Option<String> {
        theme_ctx
            .config
            .get()
            .overrides
            .as_ref()
            .and_then(|o| o.colors.as_ref())
            .and_then(|c| match key {
                "primary" => c.primary.clone(),
                "secondary" => c.secondary.clone(),
                "accent" => c.accent.clone(),
                "neutral" => c.neutral.clone(),
                "base_100" => c.base_100.clone(),
                "base_200" => c.base_200.clone(),
                "base_300" => c.base_300.clone(),
                "base_content" => c.base_content.clone(),
                "info" => c.info.clone(),
                "success" => c.success.clone(),
                "warning" => c.warning.clone(),
                "error" => c.error.clone(),
                _ => None,
            })
    };

    // Helper to update color
    let update_color = move |key: String, hex_value: String| {
        let oklch = hex_to_oklch(&hex_value);
        theme_ctx.config.update(|config| {
            let overrides = config.overrides.get_or_insert_with(Default::default);
            let colors = overrides.colors.get_or_insert_with(Default::default);

            match key.as_str() {
                "primary" => colors.primary = Some(oklch.clone()),
                "secondary" => colors.secondary = Some(oklch.clone()),
                "accent" => colors.accent = Some(oklch.clone()),
                "neutral" => colors.neutral = Some(oklch.clone()),
                "base_100" => colors.base_100 = Some(oklch.clone()),
                "base_200" => colors.base_200 = Some(oklch.clone()),
                "base_300" => colors.base_300 = Some(oklch.clone()),
                "base_content" => colors.base_content = Some(oklch.clone()),
                "info" => colors.info = Some(oklch.clone()),
                "success" => colors.success = Some(oklch.clone()),
                "warning" => colors.warning = Some(oklch.clone()),
                "error" => colors.error = Some(oklch.clone()),
                _ => {}
            }
        });
    };

    view! {
        <div class="space-y-6">
            <div>
                <h3 class="text-lg font-semibold mb-2">"Color Customization"</h3>
                <p class="text-sm opacity-70">
                    "Customize semantic color tokens. Colors are converted to Oklahoma LCH for perceptual uniformity."
                </p>
            </div>

            // Brand Colors Section
            <div class="space-y-4">
                <h4 class="text-base font-semibold">"Brand Colors"</h4>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    {color_tokens.iter()
                        .filter(|(_, key, _)| {
                            matches!(*key, "primary" | "secondary" | "accent" | "neutral")
                        })
                        .map(|(label, key, description)| {
                            let key_str = key.to_string();
                            let key_for_update = key_str.clone();
                            let key_for_reset = key_str.clone();

                            view! {
                                <div class="form-control">
                                    <label class="label">
                                        <span class="label-text font-medium">{*label}</span>
                                        <span class="label-text-alt text-xs">{*description}</span>
                                    </label>
                                    <div class="flex gap-2">
                                        <input
                                            type="color"
                                            class="input input-bordered w-16 p-1 h-12 cursor-pointer"
                                            on:change=move |ev| {
                                                let hex = event_target_value(&ev);
                                                update_color(key_for_update.clone(), hex);
                                            }
                                        />
                                        <input
                                            type="text"
                                            class="input input-bordered flex-1 font-mono text-sm"
                                            placeholder="oklch(...) or #hex"
                                            value=move || get_color(&key_str).unwrap_or_default()
                                            readonly
                                        />
                                        <button
                                            class="btn btn-ghost btn-sm"
                                            on:click=move |_| {
                                                theme_ctx.config.update(|config| {
                                                    if let Some(ref mut overrides) = config.overrides
                                                        && let Some(ref mut colors) = overrides.colors {
                                                        match key_for_reset.as_str() {
                                                            "primary" => colors.primary = None,
                                                            "secondary" => colors.secondary = None,
                                                            "accent" => colors.accent = None,
                                                            "neutral" => colors.neutral = None,
                                                            _ => {}
                                                        }
                                                    }
                                                });
                                            }
                                        >
                                            "Reset"
                                        </button>
                                    </div>
                                </div>
                            }
                        })
                        .collect_view()}
                </div>
            </div>

            // Base Colors Section
            <div class="space-y-4">
                <h4 class="text-base font-semibold">"Base Colors"</h4>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    {color_tokens.iter()
                        .filter(|(_, key, _)| {
                            matches!(*key, "base_100" | "base_200" | "base_300" | "base_content")
                        })
                        .map(|(label, key, description)| {
                            let key_str = key.to_string();
                            let key_for_update = key_str.clone();
                            let key_for_reset = key_str.clone();

                            view! {
                                <div class="form-control">
                                    <label class="label">
                                        <span class="label-text font-medium">{*label}</span>
                                        <span class="label-text-alt text-xs">{*description}</span>
                                    </label>
                                    <div class="flex gap-2">
                                        <input
                                            type="color"
                                            class="input input-bordered w-16 p-1 h-12 cursor-pointer"
                                            on:change=move |ev| {
                                                let hex = event_target_value(&ev);
                                                update_color(key_for_update.clone(), hex);
                                            }
                                        />
                                        <input
                                            type="text"
                                            class="input input-bordered flex-1 font-mono text-sm"
                                            placeholder="oklch(...) or #hex"
                                            value=move || get_color(&key_str).unwrap_or_default()
                                            readonly
                                        />
                                        <button
                                            class="btn btn-ghost btn-sm"
                                            on:click=move |_| {
                                                theme_ctx.config.update(|config| {
                                                    if let Some(ref mut overrides) = config.overrides
                                                        && let Some(ref mut colors) = overrides.colors {
                                                        match key_for_reset.as_str() {
                                                            "base_100" => colors.base_100 = None,
                                                            "base_200" => colors.base_200 = None,
                                                            "base_300" => colors.base_300 = None,
                                                            "base_content" => colors.base_content = None,
                                                            _ => {}
                                                        }
                                                    }
                                                });
                                            }
                                        >
                                            "Reset"
                                        </button>
                                    </div>
                                </div>
                            }
                        })
                        .collect_view()}
                </div>
            </div>

            // State Colors Section
            <div class="space-y-4">
                <h4 class="text-base font-semibold">"State Colors"</h4>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    {color_tokens.iter()
                        .filter(|(_, key, _)| {
                            matches!(*key, "info" | "success" | "warning" | "error")
                        })
                        .map(|(label, key, description)| {
                            let key_str = key.to_string();
                            let key_for_update = key_str.clone();
                            let key_for_reset = key_str.clone();

                            view! {
                                <div class="form-control">
                                    <label class="label">
                                        <span class="label-text font-medium">{*label}</span>
                                        <span class="label-text-alt text-xs">{*description}</span>
                                    </label>
                                    <div class="flex gap-2">
                                        <input
                                            type="color"
                                            class="input input-bordered w-16 p-1 h-12 cursor-pointer"
                                            on:change=move |ev| {
                                                let hex = event_target_value(&ev);
                                                update_color(key_for_update.clone(), hex);
                                            }
                                        />
                                        <input
                                            type="text"
                                            class="input input-bordered flex-1 font-mono text-sm"
                                            placeholder="oklch(...) or #hex"
                                            value=move || get_color(&key_str).unwrap_or_default()
                                            readonly
                                        />
                                        <button
                                            class="btn btn-ghost btn-sm"
                                            on:click=move |_| {
                                                theme_ctx.config.update(|config| {
                                                    if let Some(ref mut overrides) = config.overrides
                                                        && let Some(ref mut colors) = overrides.colors {
                                                        match key_for_reset.as_str() {
                                                            "info" => colors.info = None,
                                                            "success" => colors.success = None,
                                                            "warning" => colors.warning = None,
                                                            "error" => colors.error = None,
                                                            _ => {}
                                                        }
                                                    }
                                                });
                                            }
                                        >
                                            "Reset"
                                        </button>
                                    </div>
                                </div>
                            }
                        })
                        .collect_view()}
                </div>
            </div>

            // Color Preview
            <div class="border border-base-300 rounded-lg p-6 bg-base-100">
                <h4 class="text-sm font-semibold mb-4">"Color Preview"</h4>
                <div class="space-y-4">
                    // Brand buttons
                    <div class="flex flex-wrap gap-2">
                        <button class="btn btn-primary btn-sm">"Primary"</button>
                        <button class="btn btn-secondary btn-sm">"Secondary"</button>
                        <button class="btn btn-accent btn-sm">"Accent"</button>
                        <button class="btn btn-neutral btn-sm">"Neutral"</button>
                    </div>

                    // State badges
                    <div class="flex flex-wrap gap-2">
                        <span class="badge badge-info">"Info"</span>
                        <span class="badge badge-success">"Success"</span>
                        <span class="badge badge-warning">"Warning"</span>
                        <span class="badge badge-error">"Error"</span>
                    </div>

                    // Base colors
                    <div class="grid grid-cols-3 gap-2">
                        <div class="bg-base-100 border border-base-300 p-4 rounded text-center">
                            "Base 100"
                        </div>
                        <div class="bg-base-200 border border-base-300 p-4 rounded text-center">
                            "Base 200"
                        </div>
                        <div class="bg-base-300 border border-base-300 p-4 rounded text-center">
                            "Base 300"
                        </div>
                    </div>
                </div>
            </div>

            // Reset All Button
            <div class="flex justify-end gap-2">
                <button
                    class="btn btn-ghost btn-sm"
                    on:click=move |_| {
                        theme_ctx.config.update(|config| {
                            if let Some(ref mut overrides) = config.overrides {
                                overrides.colors = None;
                            }
                        });
                    }
                >
                    "Reset All Colors"
                </button>
            </div>
        </div>
    }
}
