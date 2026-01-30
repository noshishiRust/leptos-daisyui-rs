use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

/// Comprehensive theming demo showcasing all customizer components
///
/// This demo provides a complete theme customization interface with:
/// - Base theme selection (32 daisyUI themes)
/// - Color customization (Oklahoma LCH color space)
/// - Typography controls (fonts, sizes, scales)
/// - Border & spacing customization
/// - Component-specific overrides
/// - Import/Export functionality
/// - Theme sharing via URL
/// - Preset themes gallery
#[component]
pub fn ThemingDemo() -> impl IntoView {
    let (active_tab, set_active_tab) = signal("base".to_string());

    view! {
        <div class="min-h-screen bg-base-100 p-8">
            <div class="max-w-7xl mx-auto space-y-8">
                // Header
                <div class="text-center space-y-4">
                    <h1 class="text-5xl font-bold">"Theme Customizer"</h1>
                    <p class="text-xl opacity-70">
                        "Customize your app's appearance with our comprehensive theming system"
                    </p>
                </div>

                // Navigation Tabs
                <div class="tabs tabs-boxed justify-center">
                    <a
                        class=move || if active_tab.get() == "base" { "tab tab-active" } else { "tab" }
                        on:click=move |_| set_active_tab.set("base".to_string())
                    >
                        "Base Themes"
                    </a>
                    <a
                        class=move || if active_tab.get() == "colors" { "tab tab-active" } else { "tab" }
                        on:click=move |_| set_active_tab.set("colors".to_string())
                    >
                        "Colors"
                    </a>
                    <a
                        class=move || if active_tab.get() == "typography" { "tab tab-active" } else { "tab" }
                        on:click=move |_| set_active_tab.set("typography".to_string())
                    >
                        "Typography"
                    </a>
                    <a
                        class=move || if active_tab.get() == "borders" { "tab tab-active" } else { "tab" }
                        on:click=move |_| set_active_tab.set("borders".to_string())
                    >
                        "Borders"
                    </a>
                    <a
                        class=move || if active_tab.get() == "components" { "tab tab-active" } else { "tab" }
                        on:click=move |_| set_active_tab.set("components".to_string())
                    >
                        "Components"
                    </a>
                    <a
                        class=move || if active_tab.get() == "presets" { "tab tab-active" } else { "tab" }
                        on:click=move |_| set_active_tab.set("presets".to_string())
                    >
                        "Presets"
                    </a>
                    <a
                        class=move || if active_tab.get() == "share" { "tab tab-active" } else { "tab" }
                        on:click=move |_| set_active_tab.set("share".to_string())
                    >
                        "Share"
                    </a>
                </div>

                // Tab Content
                <div class="card bg-base-200 shadow-2xl">
                    <div class="card-body">
                        {move || match active_tab.get().as_str() {
                            "base" => view! {
                                <div>
                                    <h2 class="card-title text-2xl mb-4">"Base Themes"</h2>
                                    <p class="mb-6 opacity-70">
                                        "Choose from 32 built-in daisyUI themes. Each theme provides a complete color palette and design system."
                                    </p>
                                    <BaseThemeSelector />
                                </div>
                            }.into_any(),

                            "colors" => view! {
                                <div>
                                    <h2 class="card-title text-2xl mb-4">"Color Customization"</h2>
                                    <p class="mb-6 opacity-70">
                                        "Customize semantic colors using Oklahoma LCH color space for perceptual uniformity. Changes apply instantly across all components."
                                    </p>
                                    <ColorCustomizer />
                                </div>
                            }.into_any(),

                            "typography" => view! {
                                <div>
                                    <h2 class="card-title text-2xl mb-4">"Typography"</h2>
                                    <p class="mb-6 opacity-70">
                                        "Adjust font families, base size, and type scale ratio. Supports system fonts and monospace options."
                                    </p>
                                    <TypographyCustomizer />
                                </div>
                            }.into_any(),

                            "borders" => view! {
                                <div>
                                    <h2 class="card-title text-2xl mb-4">"Borders & Spacing"</h2>
                                    <p class="mb-6 opacity-70">
                                        "Control border widths and spacing scale to match your design preferences."
                                    </p>
                                    <BorderCustomizer />
                                </div>
                            }.into_any(),

                            "components" => view! {
                                <div>
                                    <h2 class="card-title text-2xl mb-4">"Component Overrides"</h2>
                                    <p class="mb-6 opacity-70">
                                        "Fine-tune specific components with custom styles. Override button padding, card shadows, input heights, and more."
                                    </p>
                                    <ComponentCustomizer />
                                </div>
                            }.into_any(),

                            "presets" => view! {
                                <div>
                                    <h2 class="card-title text-2xl mb-4">"Preset Themes"</h2>
                                    <p class="mb-6 opacity-70">
                                        "Browse our curated collection of preset themes. Each preset is professionally designed with custom color schemes and metadata."
                                    </p>
                                    <PresetThemesGallery />
                                </div>
                            }.into_any(),

                            "share" => view! {
                                <div class="space-y-8">
                                    <div>
                                        <h2 class="card-title text-2xl mb-4">"Export & Import"</h2>
                                        <p class="mb-6 opacity-70">
                                            "Save your theme as a JSON file or copy it to clipboard. Import previously saved themes to restore your customizations."
                                        </p>
                                        <ThemeExportImport />
                                    </div>

                                    <div class="divider" />

                                    <div>
                                        <h2 class="card-title text-2xl mb-4">"Share Theme"</h2>
                                        <p class="mb-6 opacity-70">
                                            "Generate a shareable URL containing your theme configuration. Anyone with the link can apply your theme instantly."
                                        </p>
                                        <ThemeShare />
                                    </div>
                                </div>
                            }.into_any(),

                            _ => view! { <div>"Select a tab"</div> }.into_any(),
                        }}
                    </div>
                </div>

                // Preview Section
                <div class="card bg-base-200 shadow-2xl">
                    <div class="card-body">
                        <h2 class="card-title text-2xl mb-4">"Live Preview"</h2>
                        <p class="mb-6 opacity-70">
                            "See your theme changes in action with these example components."
                        </p>

                        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                            // Buttons
                            <div class="space-y-2">
                                <h3 class="font-semibold">"Buttons"</h3>
                                <div class="flex flex-wrap gap-2">
                                    <button class="btn btn-primary">"Primary"</button>
                                    <button class="btn btn-secondary">"Secondary"</button>
                                    <button class="btn btn-accent">"Accent"</button>
                                    <button class="btn btn-ghost">"Ghost"</button>
                                </div>
                            </div>

                            // Cards
                            <div class="space-y-2">
                                <h3 class="font-semibold">"Card"</h3>
                                <div class="card bg-base-100 shadow-xl">
                                    <div class="card-body p-4">
                                        <h4 class="card-title text-base">"Sample Card"</h4>
                                        <p class="text-sm">"This card uses theme colors"</p>
                                    </div>
                                </div>
                            </div>

                            // Alerts
                            <div class="space-y-2">
                                <h3 class="font-semibold">"Alerts"</h3>
                                <div class="alert alert-info">
                                    <span>"Info message"</span>
                                </div>
                                <div class="alert alert-success">
                                    <span>"Success message"</span>
                                </div>
                            </div>

                            // Inputs
                            <div class="space-y-2">
                                <h3 class="font-semibold">"Inputs"</h3>
                                <input type="text" placeholder="Text input" class="input input-bordered w-full" />
                                <input type="text" placeholder="Primary input" class="input input-primary w-full" />
                            </div>

                            // Badges
                            <div class="space-y-2">
                                <h3 class="font-semibold">"Badges"</h3>
                                <div class="flex flex-wrap gap-2">
                                    <span class="badge badge-primary">"Primary"</span>
                                    <span class="badge badge-secondary">"Secondary"</span>
                                    <span class="badge badge-accent">"Accent"</span>
                                    <span class="badge badge-ghost">"Ghost"</span>
                                </div>
                            </div>

                            // Progress
                            <div class="space-y-2">
                                <h3 class="font-semibold">"Progress"</h3>
                                <progress class="progress progress-primary w-full" value="70" max="100" />
                                <progress class="progress progress-secondary w-full" value="40" max="100" />
                            </div>
                        </div>
                    </div>
                </div>

                // Footer with Tips
                <div class="alert">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="stroke-current shrink-0 w-6 h-6">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                    </svg>
                    <div>
                        <h3 class="font-bold">"Pro Tips"</h3>
                        <div class="text-sm space-y-1">
                            <p>"• Theme changes are automatically saved to localStorage"</p>
                            <p>"• Use Oklahoma LCH colors for consistent brightness across hues"</p>
                            <p>"• Export your theme to share with your team"</p>
                            <p>"• Try preset themes for instant professional designs"</p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
