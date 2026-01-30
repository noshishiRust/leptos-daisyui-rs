use leptos::prelude::*;
use leptos_daisyui_rs::components::ComponentCustomizer;

#[component]
pub fn ComponentCustomizerDemo() -> impl IntoView {
    view! {
        <div class="space-y-8 p-8">
            <div class="space-y-4">
                <h2 class="text-2xl font-bold">"Component Customizer"</h2>
                <p class="text-base-content/70">
                    "Fine-tune individual component styles beyond global theme settings. Customize buttons, cards, and inputs."
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
                <div>
                    <div>"Component-specific customizations active"</div>
                    <div class="text-xs opacity-70">
                        "These override global theme settings and persist to localStorage"
                    </div>
                </div>
            </div>

            <div class="card bg-base-100 shadow-xl">
                <div class="card-body">
                    <h3 class="card-title">"Component Settings"</h3>
                    <ComponentCustomizer />
                </div>
            </div>

            <div class="space-y-2">
                <h3 class="text-xl font-semibold">"Features"</h3>
                <ul class="list-disc list-inside space-y-2 text-base-content/70">
                    <li>"Button border radius customization (square to pill)"</li>
                    <li>"Card border radius customization"</li>
                    <li>"Input border width customization (0px to 4px)"</li>
                    <li>"Live preview showing all changes instantly"</li>
                    <li>"Reset button to restore component defaults"</li>
                    <li>"Changes apply globally to all instances of each component"</li>
                </ul>
            </div>

            <div class="space-y-2">
                <h3 class="text-xl font-semibold">"Usage Example"</h3>
                <div class="mockup-code text-sm">
                    <pre data-prefix="1"><code>"use leptos::prelude::*;"</code></pre>
                    <pre data-prefix="2"><code>"use leptos_daisyui_rs::components::ComponentCustomizer;"</code></pre>
                    <pre data-prefix="3"><code>"use leptos_daisyui_rs::theme::ThemeProvider;"</code></pre>
                    <pre data-prefix="4"><code></code></pre>
                    <pre data-prefix="5"><code>"#[component]"</code></pre>
                    <pre data-prefix="6"><code>"fn App() -> impl IntoView {{"</code></pre>
                    <pre data-prefix="7"><code>"    view! {{"</code></pre>
                    <pre data-prefix="8"><code>"        <ThemeProvider load_from_storage=true>"</code></pre>
                    <pre data-prefix="9"><code>"            <ComponentCustomizer />"</code></pre>
                    <pre data-prefix="10"><code>"        </ThemeProvider>"</code></pre>
                    <pre data-prefix="11"><code>"    }}"</code></pre>
                    <pre data-prefix="12"><code>"}}"</code></pre>
                </div>
            </div>
        </div>
    }
}
