use leptos::prelude::*;
use crate::theme::use_theme_context;

/// Border and Spacing Customizer Component
///
/// Provides UI controls for customizing border widths, border radius, and spacing scale.
#[component]
pub fn BorderCustomizer() -> impl IntoView {
    let theme_ctx = use_theme_context();

    let current_borders = move || {
        theme_ctx.config.get()
            .overrides
            .as_ref()
            .and_then(|o| o.borders.clone())
    };

    view! {
        <div class="space-y-6">
            <div>
                <h3 class="text-lg font-semibold mb-2">"Border & Spacing"</h3>
                <p class="text-sm opacity-70">
                    "Customize border widths, radius, and spacing scale across all components."
                </p>
            </div>

            // Border Width
            <div class="form-control">
                <label class="label">
                    <span class="label-text font-medium">"Border Width"</span>
                    <span class="label-text-alt">
                        {move || {
                            current_borders()
                                .and_then(|b| b.border_width)
                                .map(|w| format!("{}px", w))
                                .unwrap_or_else(|| "1px".to_string())
                        }}
                    </span>
                </label>
                <input
                    type="range"
                    min="0"
                    max="8"
                    step="0.5"
                    value=move || current_borders().and_then(|b| b.border_width).unwrap_or(1.0).to_string()
                    class="range range-primary"
                    on:input=move |ev| {
                        if let Ok(value) = event_target_value(&ev).parse::<f32>() {
                            theme_ctx.config.update(|config| {
                                let overrides = config.overrides.get_or_insert_with(Default::default);
                                let borders = overrides.borders.get_or_insert_with(Default::default);
                                borders.border_width = Some(value);
                            });
                        }
                    }
                />
            </div>

            // Spacing Scale
            <div class="form-control">
                <label class="label">
                    <span class="label-text font-medium">"Spacing Scale"</span>
                    <span class="label-text-alt">
                        {move || {
                            current_borders()
                                .and_then(|b| b.spacing_scale)
                                .map(|s| format!("{}x", s))
                                .unwrap_or_else(|| "1.0x".to_string())
                        }}
                    </span>
                </label>
                <input
                    type="range"
                    min="0.5"
                    max="2.0"
                    step="0.25"
                    value=move || current_borders().and_then(|b| b.spacing_scale).unwrap_or(1.0).to_string()
                    class="range range-primary"
                    on:input=move |ev| {
                        if let Ok(value) = event_target_value(&ev).parse::<f32>() {
                            theme_ctx.config.update(|config| {
                                let overrides = config.overrides.get_or_insert_with(Default::default);
                                let borders = overrides.borders.get_or_insert_with(Default::default);
                                borders.spacing_scale = Some(value);
                            });
                        }
                    }
                />
            </div>

            // Preview
            <div class="border border-base-300 rounded-lg p-6 bg-base-100">
                <h4 class="text-sm font-semibold mb-4">"Preview"</h4>
                <div class="space-y-2">
                    <input type="text" placeholder="Input" class="input input-bordered w-full" />
                    <button class="btn btn-primary btn-sm">"Button"</button>
                </div>
            </div>

            // Reset
            <div class="flex justify-end">
                <button
                    class="btn btn-ghost btn-sm"
                    on:click=move |_| {
                        theme_ctx.config.update(|config| {
                            if let Some(ref mut overrides) = config.overrides {
                                overrides.borders = None;
                            }
                        });
                    }
                >
                    "Reset"
                </button>
            </div>
        </div>
    }
}
