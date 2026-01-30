use leptos::prelude::*;
use leptos_daisyui_rs::components::TypographyCustomizer;
use leptos_daisyui_rs::theme::use_theme_context;

#[component]
pub fn TypographyCustomizerDemo() -> impl IntoView {
    let theme_ctx = use_theme_context();
    
    view! {
        <div class="space-y-8 p-8">
            <div class="space-y-4">
                <h2 class="text-2xl font-bold">"Typography Customizer"</h2>
                <p class="text-base-content/70">
                    "Customize font families, sizes, and type scales to create the perfect typography for your application."
                </p>
            </div>

            // Current Settings
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
                <div>
                    <div>"Active typography customizations applied"</div>
                    <div class="text-xs opacity-70">
                        "Changes are saved to localStorage and persist across sessions"
                    </div>
                </div>
            </div>

            // Typography Customizer
            <div class="card bg-base-100 shadow-xl">
                <div class="card-body">
                    <h3 class="card-title">"Typography Settings"</h3>
                    <TypographyCustomizer />
                </div>
            </div>

            // Features Section
            <div class="space-y-2">
                <h3 class="text-xl font-semibold">"Features"</h3>
                <ul class="list-disc list-inside space-y-2 text-base-content/70">
                    <li>"Primary font family selection with common web fonts"</li>
                    <li>"Optional heading font for typographic hierarchy"</li>
                    <li>"Dedicated monospace font for code blocks"</li>
                    <li>"Adjustable base font size (0.75rem to 1.5rem)"</li>
                    <li>"Type scale ratio selection (Minor Second to Golden Ratio)"</li>
                    <li>"Live preview showing all text sizes"</li>
                    <li>"Reset button to restore defaults"</li>
                </ul>
            </div>

            // Usage Example
            <div class="space-y-2">
                <h3 class="text-xl font-semibold">"Usage Example"</h3>
                <div class="mockup-code text-sm">
                    <pre data-prefix="1"><code>"use leptos::prelude::*;"</code></pre>
                    <pre data-prefix="2"><code>"use leptos_daisyui_rs::components::TypographyCustomizer;"</code></pre>
                    <pre data-prefix="3"><code>"use leptos_daisyui_rs::theme::ThemeProvider;"</code></pre>
                    <pre data-prefix="4"><code></code></pre>
                    <pre data-prefix="5"><code>"#[component]"</code></pre>
                    <pre data-prefix="6"><code>"fn App() -> impl IntoView {{"</code></pre>
                    <pre data-prefix="7"><code>"    view! {{"</code></pre>
                    <pre data-prefix="8"><code>"        <ThemeProvider load_from_storage=true>"</code></pre>
                    <pre data-prefix="9"><code>"            <TypographyCustomizer />"</code></pre>
                    <pre data-prefix="10"><code>"        </ThemeProvider>"</code></pre>
                    <pre data-prefix="11"><code>"    }}"</code></pre>
                    <pre data-prefix="12"><code>"}}"</code></pre>
                </div>
            </div>
        </div>
    }
}
