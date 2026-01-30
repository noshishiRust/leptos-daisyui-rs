use leptos::prelude::*;
use crate::theme::{use_theme_context, FontScale, TypographyOverrides};

/// Typography Customizer Component
///
/// Provides UI controls for customizing typography including font families,
/// font sizes, type scale, and other text properties.
///
/// This component requires a `ThemeProvider` ancestor in the component tree.
///
/// ## Features
/// - Font family selection (primary, heading, monospace)
/// - Base font size control
/// - Type scale ratio selection
/// - Live preview of typography changes
/// - Integration with ThemeProvider for reactive updates
///
/// ## Example
/// ```rust,no_run
/// use leptos::prelude::*;
/// use leptos_daisyui_rs::components::TypographyCustomizer;
/// use leptos_daisyui_rs::theme::ThemeProvider;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <ThemeProvider load_from_storage=true>
///             <div class="p-8">
///                 <h2 class="text-2xl font-bold mb-4">"Customize Typography"</h2>
///                 <TypographyCustomizer />
///             </div>
///         </ThemeProvider>
///     }
/// }
/// ```
///
/// ### Add to `input.css`
/// ```css
/// @source inline("form-control label label-text label-text-alt");
/// @source inline("select select-bordered range");
/// ```
#[component]
pub fn TypographyCustomizer() -> impl IntoView {
    let theme_ctx = use_theme_context();

    // Common font families
    let font_families = vec![
        ("System Default", ""),
        ("Inter", "Inter, system-ui, sans-serif"),
        ("Roboto", "Roboto, sans-serif"),
        ("Open Sans", "Open Sans, sans-serif"),
        ("Lato", "Lato, sans-serif"),
        ("Poppins", "Poppins, sans-serif"),
        ("Montserrat", "Montserrat, sans-serif"),
        ("Playfair Display", "Playfair Display, serif"),
        ("Merriweather", "Merriweather, serif"),
    ];

    let mono_families = vec![
        ("System Default", ""),
        ("Fira Code", "Fira Code, monospace"),
        ("JetBrains Mono", "JetBrains Mono, monospace"),
        ("Source Code Pro", "Source Code Pro, monospace"),
        ("Inconsolata", "Inconsolata, monospace"),
    ];

    // Type scale ratios
    let scale_ratios = vec![
        ("Minor Second", 1.125),
        ("Major Second", 1.125),
        ("Minor Third", 1.2),
        ("Major Third", 1.25),
        ("Perfect Fourth", 1.333),
        ("Augmented Fourth", 1.414),
        ("Perfect Fifth", 1.5),
        ("Golden Ratio", 1.618),
    ];

    // Get current typography settings
    let current_typography = move || {
        theme_ctx.config.get()
            .overrides
            .and_then(|o| o.typography)
            .unwrap_or_default()
    };

    // Helper to update typography
    let update_typography = move |updater: Box<dyn Fn(&mut TypographyOverrides)>| {
        theme_ctx.config.update(|config| {
            if config.overrides.is_none() {
                config.overrides = Some(Default::default());
            }
            if let Some(ref mut overrides) = config.overrides {
                if overrides.typography.is_none() {
                    overrides.typography = Some(Default::default());
                }
                if let Some(ref mut typography) = overrides.typography {
                    updater(typography);
                }
            }
        });
    };

    view! {
        <div class="space-y-6">
            <p class="text-sm text-base-content/70">
                "Customize fonts, sizes, and type scales. Changes apply instantly."
            </p>

            // Primary Font Family
            <div class="form-control">
                <label class="label">
                    <span class="label-text font-semibold">"Primary Font Family"</span>
                    <span class="label-text-alt">
                        {move || {
                            current_typography()
                                .font_family
                                .as_ref()
                                .and_then(|f| f.primary.clone())
                                .unwrap_or_else(|| "System Default".to_string())
                        }}
                    </span>
                </label>
                <select
                    class="select select-bordered"
                    on:change=move |ev| {
                        let value = event_target_value(&ev);
                        let update_value = value.clone();
                        update_typography(Box::new(move |typography| {
                            if typography.font_family.is_none() {
                                typography.font_family = Some(Default::default());
                            }
                            if let Some(ref mut font) = typography.font_family {
                                font.primary = if update_value.is_empty() {
                                    None
                                } else {
                                    Some(update_value.clone())
                                };
                            }
                        }));
                    }
                >
                    {font_families.clone()
                        .into_iter()
                        .map(|(name, value)| {
                            view! {
                                <option value=value>{name}</option>
                            }
                        })
                        .collect_view()}
                </select>
            </div>

            // Heading Font Family
            <div class="form-control">
                <label class="label">
                    <span class="label-text font-semibold">"Heading Font Family"</span>
                    <span class="label-text-alt">"Optional - defaults to primary"</span>
                </label>
                <select
                    class="select select-bordered"
                    on:change=move |ev| {
                        let value = event_target_value(&ev);
                        let update_value = value.clone();
                        update_typography(Box::new(move |typography| {
                            if typography.font_family.is_none() {
                                typography.font_family = Some(Default::default());
                            }
                            if let Some(ref mut font) = typography.font_family {
                                font.heading = if update_value.is_empty() {
                                    None
                                } else {
                                    Some(update_value.clone())
                                };
                            }
                        }));
                    }
                >
                    {vec![
                        ("System Default", ""),
                        ("Inter", "Inter, system-ui, sans-serif"),
                        ("Roboto", "Roboto, sans-serif"),
                        ("Open Sans", "Open Sans, sans-serif"),
                        ("Lato", "Lato, sans-serif"),
                        ("Poppins", "Poppins, sans-serif"),
                        ("Montserrat", "Montserrat, sans-serif"),
                        ("Playfair Display", "Playfair Display, serif"),
                        ("Merriweather", "Merriweather, serif"),
                    ]
                        .into_iter()
                        .map(|(name, value)| {
                            view! {
                                <option value=value>{name}</option>
                            }
                        })
                        .collect_view()}
                </select>
            </div>

            // Monospace Font Family
            <div class="form-control">
                <label class="label">
                    <span class="label-text font-semibold">"Monospace Font Family"</span>
                    <span class="label-text-alt">"For code blocks"</span>
                </label>
                <select
                    class="select select-bordered"
                    on:change=move |ev| {
                        let value = event_target_value(&ev);
                        let update_value = value.clone();
                        update_typography(Box::new(move |typography| {
                            if typography.font_family.is_none() {
                                typography.font_family = Some(Default::default());
                            }
                            if let Some(ref mut font) = typography.font_family {
                                font.monospace = if update_value.is_empty() {
                                    None
                                } else {
                                    Some(update_value.clone())
                                };
                            }
                        }));
                    }
                >
                    {mono_families
                        .into_iter()
                        .map(|(name, value)| {
                            view! {
                                <option value=value>{name}</option>
                            }
                        })
                        .collect_view()}
                </select>
            </div>

            // Base Font Size
            <div class="form-control">
                <label class="label">
                    <span class="label-text font-semibold">"Base Font Size"</span>
                    <span class="label-text-alt">
                        {move || {
                            format!("{}rem", current_typography().base_font_size.unwrap_or(1.0))
                        }}
                    </span>
                </label>
                <input
                    type="range"
                    min="0.75"
                    max="1.5"
                    step="0.0625"
                    value="1.0"
                    class="range"
                    on:input=move |ev| {
                        if let Ok(value) = event_target_value(&ev).parse::<f32>() {
                            update_typography(Box::new(move |typography| {
                                typography.base_font_size = Some(value);
                            }));
                        }
                    }
                />
                <div class="flex justify-between text-xs px-2 mt-1">
                    <span>"0.75rem"</span>
                    <span>"1.0rem"</span>
                    <span>"1.5rem"</span>
                </div>
            </div>

            // Type Scale Ratio
            <div class="form-control">
                <label class="label">
                    <span class="label-text font-semibold">"Type Scale Ratio"</span>
                    <span class="label-text-alt">"Controls size relationships"</span>
                </label>
                <select
                    class="select select-bordered"
                    on:change=move |ev| {
                        if let Ok(ratio) = event_target_value(&ev).parse::<f32>() {
                            update_typography(Box::new(move |typography| {
                                let base = typography.base_font_size.unwrap_or(1.0);
                                typography.font_scale = Some(FontScale {
                                    xs: Some(base / (ratio * ratio)),
                                    sm: Some(base / ratio),
                                    base: Some(base),
                                    lg: Some(base * ratio),
                                    xl: Some(base * ratio * ratio),
                                    xl2: Some(base * ratio * ratio * ratio),
                                    xl3: Some(base * ratio * ratio * ratio * ratio),
                                });
                            }));
                        }
                    }
                >
                    {scale_ratios
                        .into_iter()
                        .map(|(name, ratio)| {
                            view! {
                                <option value=ratio.to_string()>{name} " (" {ratio} ")"</option>
                            }
                        })
                        .collect_view()}
                </select>
            </div>

            // Typography Preview
            <div class="border border-base-300 rounded-lg p-6 space-y-4 bg-base-200/30">
                <h4 class="text-sm font-bold uppercase text-base-content/60 mb-4">"Preview"</h4>

                <div class="space-y-2">
                    <p class="text-3xl font-bold">"Heading 1"</p>
                    <p class="text-2xl font-bold">"Heading 2"</p>
                    <p class="text-xl font-bold">"Heading 3"</p>
                    <p class="text-lg">"Large Text"</p>
                    <p class="text-base">"Base Text - The quick brown fox jumps over the lazy dog."</p>
                    <p class="text-sm">"Small Text - The quick brown fox jumps over the lazy dog."</p>
                    <p class="text-xs">"Extra Small Text - The quick brown fox jumps over the lazy dog."</p>
                    <code class="block p-2 bg-base-300 rounded font-mono text-sm">
                        "Monospace: function example() {{ return 42; }}"
                    </code>
                </div>
            </div>

            // Reset Button
            <div class="flex justify-end">
                <button
                    class="btn btn-ghost btn-sm"
                    on:click=move |_| {
                        theme_ctx.config.update(|config| {
                            if let Some(ref mut overrides) = config.overrides {
                                overrides.typography = None;
                            }
                        });
                    }
                >
                    "Reset Typography"
                </button>
            </div>
        </div>
    }
}
