use leptos::prelude::*;
use leptos_daisyui_rs::components::BaseThemeSelector;
use leptos_daisyui_rs::theme::use_theme_context;

#[component]
pub fn BaseThemeSelectorDemo() -> impl IntoView {
    let theme_ctx = use_theme_context();

    view! {
        <div class="space-y-8 p-8">
            <div class="space-y-4">
                <h2 class="text-2xl font-bold">"Base Theme Selector"</h2>
                <p class="text-base-content/70">
                    "Choose from all 32 daisyUI themes with live preview. Click any theme card to switch instantly."
                </p>
            </div>

            <div class="alert alert-info">
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 24 24"
                    class="stroke-current shrink-0 w-6 h-6"
                >
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                    ></path>
                </svg>
                <span>
                    "Active theme: "
                    <strong class="capitalize">{move || theme_ctx.base_theme()}</strong>
                </span>
            </div>

            <div class="space-y-4">
                <h3 class="text-xl font-semibold">"Available Themes"</h3>
                <BaseThemeSelector />
            </div>
        </div>
    }
}
