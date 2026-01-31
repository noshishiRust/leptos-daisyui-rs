use super::presets::get_preset_themes;
use crate::theme::use_theme_context;
use leptos::prelude::*;

/// Preset Themes Gallery Component
///
/// Displays a gallery of pre-configured theme presets with custom color schemes
/// and configurations. Users can preview and apply these preset themes.
///
/// ## Features
/// - Grid display of preset themes
/// - Theme preview cards with color swatches
/// - One-click theme application
/// - Theme metadata display (name, description, tags)
///
/// ## Example
/// ```rust,ignore
/// use leptos::prelude::*;
/// use leptos_daisyui_rs::components::PresetThemesGallery;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <PresetThemesGallery />
///     }
/// }
/// ```
#[component]
pub fn PresetThemesGallery() -> impl IntoView {
    let theme_ctx = use_theme_context();
    let presets = get_preset_themes();

    view! {
        <div class="space-y-6">
            <div>
                <h3 class="text-lg font-semibold mb-2">"Preset Themes"</h3>
                <p class="text-sm opacity-70">
                    "Choose from our curated collection of pre-configured themes."
                </p>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                {presets.into_iter().map(|(id, config)| {
                    let name = config.metadata.as_ref()
                        .and_then(|m| m.name.clone())
                        .unwrap_or_else(|| id.clone());

                    let description = config.metadata.as_ref()
                        .and_then(|m| m.description.clone())
                        .unwrap_or_default();

                    let tags = config.metadata.as_ref()
                        .and_then(|m| m.tags.clone())
                        .unwrap_or_default();

                    // Extract colors for preview
                    let primary = config.overrides.as_ref()
                        .and_then(|o| o.colors.as_ref())
                        .and_then(|c| c.primary.clone())
                        .unwrap_or_else(|| "oklch(0.5 0.2 240.0)".to_string());

                    let secondary = config.overrides.as_ref()
                        .and_then(|o| o.colors.as_ref())
                        .and_then(|c| c.secondary.clone())
                        .unwrap_or_else(|| "oklch(0.4 0.15 220.0)".to_string());

                    let accent = config.overrides.as_ref()
                        .and_then(|o| o.colors.as_ref())
                        .and_then(|c| c.accent.clone())
                        .unwrap_or_else(|| "oklch(0.6 0.18 180.0)".to_string());

                    let config_clone = config.clone();

                    view! {
                        <div class="card bg-base-200 shadow-xl hover:shadow-2xl transition-shadow">
                            <div class="card-body">
                                <h4 class="card-title text-base">{name.clone()}</h4>

                                {if !description.is_empty() {
                                    view! {
                                        <p class="text-sm opacity-70">{description}</p>
                                    }.into_any()
                                } else {
                                    ().into_any()
                                }}

                                // Color swatches
                                <div class="flex gap-2 my-2">
                                    <div
                                        class="w-8 h-8 rounded-full border-2 border-base-300"
                                        style:background-color=primary
                                        title="Primary"
                                    />
                                    <div
                                        class="w-8 h-8 rounded-full border-2 border-base-300"
                                        style:background-color=secondary
                                        title="Secondary"
                                    />
                                    <div
                                        class="w-8 h-8 rounded-full border-2 border-base-300"
                                        style:background-color=accent
                                        title="Accent"
                                    />
                                </div>

                                // Tags
                                {if !tags.is_empty() {
                                    view! {
                                        <div class="flex flex-wrap gap-1">
                                            {tags.into_iter().map(|tag| {
                                                view! {
                                                    <span class="badge badge-sm">{tag}</span>
                                                }
                                            }).collect::<Vec<_>>()}
                                        </div>
                                    }.into_any()
                                } else {
                                    ().into_any()
                                }}

                                <div class="card-actions justify-end mt-2">
                                    <button
                                        class="btn btn-primary btn-sm"
                                        on:click=move |_| {
                                            theme_ctx.apply_theme(config_clone.clone());
                                        }
                                    >
                                        "Apply Theme"
                                    </button>
                                </div>
                            </div>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
