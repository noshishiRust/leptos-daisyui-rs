use leptos::prelude::*;
use crate::theme::{use_theme_context, ComponentOverrides};

/// Component Customizer
///
/// Provides UI controls for customizing individual component styles.
/// Allows fine-tuning of specific components beyond the global theme settings.
///
/// This component requires a `ThemeProvider` ancestor in the component tree.
///
/// ## Features
/// - Button border radius customization
/// - Card border width customization
/// - Input border width customization
/// - Live preview of changes
/// - Reset functionality
///
/// ## Example
/// ```rust,no_run
/// use leptos::prelude::*;
/// use leptos_daisyui_rs::components::ComponentCustomizer;
/// use leptos_daisyui_rs::theme::ThemeProvider;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <ThemeProvider load_from_storage=true>
///             <div class="p-8">
///                 <h2 class="text-2xl font-bold mb-4">"Customize Components"</h2>
///                 <ComponentCustomizer />
///             </div>
///         </ThemeProvider>
///     }
/// }
/// ```
///
/// ### Add to `input.css`
/// ```css
/// @source inline("form-control label label-text label-text-alt select select-bordered");
/// ```
#[component]
pub fn ComponentCustomizer() -> impl IntoView {
    let theme_ctx = use_theme_context();

    // Border radius options
    let border_radius_options = vec![
        ("None (Square)", "0"),
        ("Small (0.125rem)", "0.125rem"),
        ("Medium (0.25rem)", "0.25rem"),
        ("Large (0.5rem)", "0.5rem"),
        ("Extra Large (1rem)", "1rem"),
        ("Full (Pill)", "9999px"),
    ];

    // Border width options
    let border_width_options = vec![
        ("None", "0"),
        ("Thin (1px)", "1px"),
        ("Medium (2px)", "2px"),
        ("Thick (3px)", "3px"),
        ("Extra Thick (4px)", "4px"),
    ];

    // Get current component overrides
    let current_components = move || {
        theme_ctx.config.get()
            .overrides
            .and_then(|o| o.components)
            .unwrap_or_default()
    };

    // Helper to update component overrides
    let update_components = move |updater: Box<dyn Fn(&mut ComponentOverrides)>| {
        theme_ctx.config.update(|config| {
            if config.overrides.is_none() {
                config.overrides = Some(Default::default());
            }
            if let Some(ref mut overrides) = config.overrides {
                if overrides.components.is_none() {
                    overrides.components = Some(Default::default());
                }
                if let Some(ref mut components) = overrides.components {
                    updater(components);
                }
            }
        });
    };

    view! {
        <div class="space-y-6">
            <p class="text-sm text-base-content/70">
                "Fine-tune individual component styles. These override global theme settings."
            </p>

            // Button Customization
            <div class="card bg-base-200/50 border border-base-300">
                <div class="card-body">
                    <h4 class="card-title text-lg">"Button Customization"</h4>

                    <div class="form-control">
                        <label class="label">
                            <span class="label-text font-semibold">"Border Radius"</span>
                            <span class="label-text-alt">
                                {move || {
                                    current_components()
                                        .button
                                        .as_ref()
                                        .and_then(|b| b.border_radius.map(|r| format!("{}rem", r)))
                                        .unwrap_or_else(|| "Default".to_string())
                                }}
                            </span>
                        </label>
                        <select
                            class="select select-bordered"
                            on:change=move |ev| {
                                let value = event_target_value(&ev);
                                update_components(Box::new(move |components| {
                                    if value.is_empty() {
                                        if let Some(ref mut button) = components.button {
                                            button.border_radius = None;
                                        }
                                    } else if let Ok(radius) = value.trim_end_matches("rem").trim_end_matches("px").parse::<f32>() {
                                        if components.button.is_none() {
                                            components.button = Some(Default::default());
                                        }
                                        if let Some(ref mut button) = components.button {
                                            button.border_radius = Some(radius);
                                        }
                                    }
                                }));
                            }
                        >
                            <option value="">"Use Default"</option>
                            {border_radius_options.clone()
                                .into_iter()
                                .map(|(name, value)| {
                                    view! {
                                        <option value=value>{name}</option>
                                    }
                                })
                                .collect_view()}
                        </select>
                    </div>
                </div>
            </div>

            // Card Customization
            <div class="card bg-base-200/50 border border-base-300">
                <div class="card-body">
                    <h4 class="card-title text-lg">"Card Customization"</h4>

                    <div class="form-control">
                        <label class="label">
                            <span class="label-text font-semibold">"Border Width"</span>
                            <span class="label-text-alt">
                                {move || {
                                    current_components()
                                        .card
                                        .as_ref()
                                        .and_then(|c| c.border_radius.map(|r| format!("{}rem", r)))
                                        .unwrap_or_else(|| "Default".to_string())
                                }}
                            </span>
                        </label>
                        <select
                            class="select select-bordered"
                            on:change=move |ev| {
                                let value = event_target_value(&ev);
                                update_components(Box::new(move |components| {
                                    if value.is_empty() {
                                        if let Some(ref mut card) = components.card {
                                            card.border_radius = None;
                                        }
                                    } else if let Ok(radius) = value.trim_end_matches("rem").trim_end_matches("px").parse::<f32>() {
                                        if components.card.is_none() {
                                            components.card = Some(Default::default());
                                        }
                                        if let Some(ref mut card) = components.card {
                                            card.border_radius = Some(radius);
                                        }
                                    }
                                }));
                            }
                        >
                            <option value="">"Use Default"</option>
                            {border_width_options.clone()
                                .into_iter()
                                .map(|(name, value)| {
                                    view! {
                                        <option value=value>{name}</option>
                                    }
                                })
                                .collect_view()}
                        </select>
                    </div>
                </div>
            </div>

            // Input Customization
            <div class="card bg-base-200/50 border border-base-300">
                <div class="card-body">
                    <h4 class="card-title text-lg">"Input Customization"</h4>

                    <div class="form-control">
                        <label class="label">
                            <span class="label-text font-semibold">"Border Width"</span>
                            <span class="label-text-alt">
                                {move || {
                                    current_components()
                                        .input
                                        .as_ref()
                                        .and_then(|i| i.border_width.map(|w| format!("{}px", w)))
                                        .unwrap_or_else(|| "Default".to_string())
                                }}
                            </span>
                        </label>
                        <select
                            class="select select-bordered"
                            on:change=move |ev| {
                                let value = event_target_value(&ev);
                                update_components(Box::new(move |components| {
                                    if value.is_empty() {
                                        if let Some(ref mut input) = components.input {
                                            input.border_width = None;
                                        }
                                    } else if let Ok(width) = value.trim_end_matches("px").parse::<f32>() {
                                        if components.input.is_none() {
                                            components.input = Some(Default::default());
                                        }
                                        if let Some(ref mut input) = components.input {
                                            input.border_width = Some(width);
                                        }
                                    }
                                }));
                            }
                        >
                            <option value="">"Use Default"</option>
                            {border_width_options
                                .into_iter()
                                .map(|(name, value)| {
                                    view! {
                                        <option value=value>{name}</option>
                                    }
                                })
                                .collect_view()}
                        </select>
                    </div>
                </div>
            </div>

            // Preview Section
            <div class="border border-base-300 rounded-lg p-6 space-y-4 bg-base-100">
                <h4 class="text-sm font-bold uppercase text-base-content/60 mb-4">"Live Preview"</h4>

                <div class="space-y-4">
                    <div>
                        <p class="text-sm mb-2">"Buttons:"</p>
                        <div class="flex gap-2">
                            <button class="btn btn-primary">"Primary"</button>
                            <button class="btn btn-secondary">"Secondary"</button>
                            <button class="btn btn-accent">"Accent"</button>
                        </div>
                    </div>

                    <div>
                        <p class="text-sm mb-2">"Card:"</p>
                        <div class="card bg-base-200 border border-base-300 max-w-sm">
                            <div class="card-body">
                                <h3 class="card-title">"Sample Card"</h3>
                                <p>"This card preview shows the applied customizations."</p>
                            </div>
                        </div>
                    </div>

                    <div>
                        <p class="text-sm mb-2">"Inputs:"</p>
                        <input type="text" placeholder="Text input" class="input input-bordered w-full max-w-xs" />
                    </div>
                </div>
            </div>

            // Reset Button
            <div class="flex justify-end">
                <button
                    class="btn btn-ghost btn-sm"
                    on:click=move |_| {
                        theme_ctx.config.update(|config| {
                            if let Some(ref mut overrides) = config.overrides {
                                overrides.components = None;
                            }
                        });
                    }
                >
                    "Reset Component Customizations"
                </button>
            </div>
        </div>
    }
}
