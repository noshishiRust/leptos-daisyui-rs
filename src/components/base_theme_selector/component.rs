use leptos::prelude::*;
use crate::theme::{use_theme_context, get_available_themes};

/// Base Theme Selector Component
///
/// Displays a grid of all available daisyUI themes with preview cards.
/// Users can click a theme card to switch themes. The selected theme is
/// highlighted with a border and checkmark indicator.
///
/// This component requires a `ThemeProvider` ancestor in the component tree.
///
/// ## Features
/// - Grid layout with responsive columns (2 on mobile, 4 on tablet+)
/// - Visual preview of each theme's colors
/// - Active theme indication with border and checkmark
/// - Smooth transitions when switching themes
/// - Integration with ThemeProvider for reactive updates
///
/// ## Example
/// ```rust,no_run
/// use leptos::prelude::*;
/// use leptos_daisyui_rs::components::BaseThemeSelector;
/// use leptos_daisyui_rs::theme::ThemeProvider;
///
/// #[component]
/// fn App() -> impl IntoView {
///     view! {
///         <ThemeProvider load_from_storage=true>
///             <div class="p-8">
///                 <h2 class="text-2xl font-bold mb-4">"Choose a Theme"</h2>
///                 <BaseThemeSelector />
///             </div>
///         </ThemeProvider>
///     }
/// }
/// ```
///
/// ### Add to `input.css`
/// ```css
/// @source inline("grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4");
/// @source inline("card bg-base-100 shadow-xl border-2 border-transparent");
/// @source inline("card-body p-4 cursor-pointer transition-all duration-300");
/// @source inline("hover:shadow-2xl hover:scale-105");
/// @source inline("border-primary ring-2 ring-primary ring-offset-2");
/// ```
#[component]
pub fn BaseThemeSelector(
    /// Optional callback when theme changes
    #[prop(optional, into)]
    on_theme_change: Option<Callback<String>>,
) -> impl IntoView {
    // Get theme context
    let theme_ctx = use_theme_context();

    // Get all available themes
    let themes = get_available_themes();

    // Get current theme reactively
    let current_theme = move || theme_ctx.config.get().base_theme;

    view! {
        <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
            {themes
                .into_iter()
                .map(|theme_name| {
                    let theme_name_str = theme_name.to_string();
                    let theme_for_click = theme_name_str.clone();
                    let theme_for_callback = theme_name_str.clone();
                    let theme_for_border = theme_name_str.clone();
                    let theme_for_ring2 = theme_name_str.clone();
                    let theme_for_ring_primary = theme_name_str.clone();
                    let theme_for_ring_offset = theme_name_str.clone();
                    let theme_for_transparent = theme_name_str.clone();
                    let theme_for_checkmark = theme_name_str.clone();
                    let theme_for_attr = theme_name_str.clone();

                    view! {
                        <div
                            class="card bg-base-100 shadow-xl border-2 transition-all duration-300 hover:shadow-2xl hover:scale-105 cursor-pointer"
                            class:border-primary=move || current_theme() == theme_for_border
                            class:ring-2=move || current_theme() == theme_for_ring2
                            class:ring-primary=move || current_theme() == theme_for_ring_primary
                            class:ring-offset-2=move || current_theme() == theme_for_ring_offset
                            class:border-transparent=move || current_theme() != theme_for_transparent
                            on:click=move |_| {
                                theme_ctx.set_base_theme(&theme_for_click);
                                if let Some(ref callback) = on_theme_change {
                                    callback.run(theme_for_callback.clone());
                                }
                            }
                            data-theme=theme_for_attr
                        >
                            <div class="card-body p-4">
                                // Theme name with checkmark if selected
                                <div class="flex items-center justify-between mb-2">
                                    <h3 class="card-title text-sm capitalize">{theme_name}</h3>
                                    {move || {
                                        if current_theme() == theme_for_checkmark {
                                            view! {
                                                <svg
                                                    xmlns="http://www.w3.org/2000/svg"
                                                    class="h-5 w-5 text-primary"
                                                    viewBox="0 0 20 20"
                                                    fill="currentColor"
                                                >
                                                    <path
                                                        fill-rule="evenodd"
                                                        d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
                                                        clip-rule="evenodd"
                                                    />
                                                </svg>
                                            }
                                                .into_any()
                                        } else {
                                            view! { <div class="w-5 h-5"></div> }.into_any()
                                        }
                                    }}

                                </div>

                                // Color preview swatches
                                <div class="flex gap-1">
                                    <div class="w-full h-6 rounded bg-primary"></div>
                                    <div class="w-full h-6 rounded bg-secondary"></div>
                                    <div class="w-full h-6 rounded bg-accent"></div>
                                    <div class="w-full h-6 rounded bg-neutral"></div>
                                </div>
                            </div>
                        </div>
                    }
                })
                .collect_view()}
        </div>
    }
}
