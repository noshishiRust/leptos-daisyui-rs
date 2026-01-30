use leptos::prelude::*;
use leptos_daisyui_rs::*;

#[component]
pub fn ConfigProviderDemo() -> impl IntoView {
    // Example configuration
    let initial_config = AppConfig::new("en-US");

    view! {
        <ConfigProvider initial_config=initial_config>
            <div class="space-y-8 p-8">
                <div class="space-y-4">
                    <h2 class="text-2xl font-bold">"ConfigProvider Component"</h2>
                    <p class="text-base-content/70">
                        "Global configuration provider for locale, text direction, and accessibility settings."
                    </p>
                </div>

                // Current Configuration Display
                <ConfigDisplay />

                // Locale Switcher
                <LocaleSwitcher />

                // Direction Toggle
                <DirectionToggle />

                // Accessibility Controls
                <AccessibilityControls />

                // Example Content
                <ExampleContent />

                // Usage Example
                <div class="space-y-2">
                    <h3 class="text-xl font-semibold">"Usage Example"</h3>
                    <div class="mockup-code text-sm">
                        <pre data-prefix=">">
                            <code>"<ConfigProvider initial_config=config>"</code>
                        </pre>
                        <pre data-prefix=">">
                            <code>"  <div class=\"app\">"</code>
                        </pre>
                        <pre data-prefix=">">
                            <code>"    Your app content"</code>
                        </pre>
                        <pre data-prefix=">">
                            <code>"  </div>"</code>
                        </pre>
                        <pre data-prefix=">">
                            <code>"</ConfigProvider>"</code>
                        </pre>
                    </div>
                </div>
            </div>
        </ConfigProvider>
    }
}

#[component]
fn ConfigDisplay() -> impl IntoView {
    let config_ctx = use_config_context();

    view! {
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
                <p><strong>"Current Configuration:"</strong></p>
                <p>"Locale: " {move || config_ctx.locale()}</p>
                <p>"Direction: " {move || config_ctx.direction().as_str()}</p>
                <p>"Reduced Motion: " {move || config_ctx.reduce_motion().to_string()}</p>
                <p>"High Contrast: " {move || config_ctx.high_contrast().to_string()}</p>
            </div>
        </div>
    }
}

#[component]
fn LocaleSwitcher() -> impl IntoView {
    let config_ctx = use_config_context();

    let locales = vec![
        ("en-US", "English (US)"),
        ("ar-SA", "Arabic (Saudi Arabia)"),
        ("he-IL", "Hebrew (Israel)"),
        ("fr-FR", "French (France)"),
        ("de-DE", "German (Germany)"),
        ("ja-JP", "Japanese (Japan)"),
    ];

    view! {
        <div class="space-y-2">
            <h3 class="text-xl font-semibold">"Locale Selection"</h3>
            <div class="flex flex-wrap gap-2">
                {locales
                    .into_iter()
                    .map(|(locale, label)| {
                        let locale_str = locale.to_string();
                        view! {
                            <button
                                class="btn btn-sm"
                                class:btn-primary=move || config_ctx.locale() == locale_str
                                on:click=move |_| config_ctx.set_locale(locale)
                            >
                                {label}
                            </button>
                        }
                    })
                    .collect_view()}
            </div>
        </div>
    }
}

#[component]
fn DirectionToggle() -> impl IntoView {
    let config_ctx = use_config_context();

    view! {
        <div class="space-y-2">
            <h3 class="text-xl font-semibold">"Text Direction"</h3>
            <div class="flex gap-2">
                <button
                    class="btn btn-sm"
                    class:btn-primary=move || config_ctx.direction() == TextDirection::Ltr
                    on:click=move |_| config_ctx.set_direction(TextDirection::Ltr)
                >
                    "LTR (Left-to-Right)"
                </button>
                <button
                    class="btn btn-sm"
                    class:btn-primary=move || config_ctx.direction() == TextDirection::Rtl
                    on:click=move |_| config_ctx.set_direction(TextDirection::Rtl)
                >
                    "RTL (Right-to-Left)"
                </button>
            </div>
        </div>
    }
}

#[component]
fn AccessibilityControls() -> impl IntoView {
    let config_ctx = use_config_context();

    view! {
        <div class="space-y-2">
            <h3 class="text-xl font-semibold">"Accessibility Settings"</h3>
            <div class="space-y-2">
                <label class="label cursor-pointer justify-start gap-4">
                    <input
                        type="checkbox"
                        class="toggle toggle-primary"
                        prop:checked=move || config_ctx.reduce_motion()
                        on:change=move |ev| {
                            config_ctx.set_reduce_motion(event_target_checked(&ev));
                        }
                    />
                    <span class="label-text">"Reduce Motion"</span>
                </label>

                <label class="label cursor-pointer justify-start gap-4">
                    <input
                        type="checkbox"
                        class="toggle toggle-primary"
                        prop:checked=move || config_ctx.high_contrast()
                        on:change=move |ev| {
                            config_ctx.set_high_contrast(event_target_checked(&ev));
                        }
                    />
                    <span class="label-text">"High Contrast"</span>
                </label>
            </div>
        </div>
    }
}

#[component]
fn ExampleContent() -> impl IntoView {
    view! {
        <div class="space-y-2">
            <h3 class="text-xl font-semibold">"Example Content"</h3>
            <div class="card bg-base-200 shadow-xl">
                <div class="card-body">
                    <h2 class="card-title">"Sample Card"</h2>
                    <p>"This content will automatically inherit the text direction set above."</p>
                    <p class="text-sm opacity-70">
                        "Try switching to Arabic or Hebrew to see RTL layout in action."
                    </p>
                </div>
            </div>
        </div>
    }
}
