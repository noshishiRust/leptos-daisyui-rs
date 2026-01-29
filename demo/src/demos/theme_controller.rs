use leptos::prelude::*;
use leptos_daisyui_rs::components::ThemeController;

#[component]
pub fn ThemeControllerDemo() -> impl IntoView {
    // Get the global theme setter from context
    let set_theme = use_context::<WriteSignal<String>>()
        .expect("Theme context should be provided by App");

    // Track selected theme for UI display
    let (selected_theme, set_local) = signal("light".to_string());

    // Helper to update both global and local
    let update_theme_sync = move |theme: String| {
        set_theme.set(theme.clone());
        set_local.set(theme);
    };

    view! {
        <div class="space-y-8 p-8">
            <div class="space-y-4">
                <h2 class="text-2xl font-bold">"Theme Controller Component"</h2>
                <p class="text-base-content/70">
                    "Switch between daisyUI themes using form controls."
                </p>
            </div>

            // Current Theme Display
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
                    "Current theme: "
                    <strong>{move || selected_theme.get()}</strong>
                </span>
            </div>

            // Dropdown Theme Selector
            <div class="space-y-2">
                <h3 class="text-xl font-semibold">"Dropdown Theme Selector"</h3>
                <div class="dropdown">
                    <div tabindex="0" role="button" class="btn m-1">
                        "Theme"
                        <svg
                            width="12px"
                            height="12px"
                            class="h-2 w-2 fill-current opacity-60 inline-block"
                            xmlns="http://www.w3.org/2000/svg"
                            viewBox="0 0 2048 2048"
                        >
                            <path d="M1799 349l242 241-1017 1017L7 590l242-241 775 775 775-775z"></path>
                        </svg>
                    </div>
                    <ul
                        tabindex="0"
                        class="dropdown-content z-[1] p-2 shadow-2xl bg-base-300 rounded-box w-52"
                    >
                        <li>
                            <ThemeController theme_name="light">
                                <input
                                    type="radio"
                                    name="theme-dropdown"
                                    class="theme-controller btn btn-sm btn-block btn-ghost justify-start"
                                    aria-label="Light"
                                    value="light"
                                    prop:checked=move || selected_theme.get() == "light"
                                    on:change=move |_| update_theme_sync("light".to_string())
                                />
                            </ThemeController>
                        </li>
                        <li>
                            <ThemeController theme_name="dark">
                                <input
                                    type="radio"
                                    name="theme-dropdown"
                                    class="theme-controller btn btn-sm btn-block btn-ghost justify-start"
                                    aria-label="Dark"
                                    value="dark"
                                    prop:checked=move || selected_theme.get() == "dark"
                                    on:change=move |_| update_theme_sync("dark".to_string())
                                />
                            </ThemeController>
                        </li>
                        <li>
                            <ThemeController theme_name="cupcake">
                                <input
                                    type="radio"
                                    name="theme-dropdown"
                                    class="theme-controller btn btn-sm btn-block btn-ghost justify-start"
                                    aria-label="Cupcake"
                                    value="cupcake"
                                    prop:checked=move || selected_theme.get() == "cupcake"
                                    on:change=move |_| update_theme_sync("cupcake".to_string())
                                />
                            </ThemeController>
                        </li>
                        <li>
                            <ThemeController theme_name="cyberpunk">
                                <input
                                    type="radio"
                                    name="theme-dropdown"
                                    class="theme-controller btn btn-sm btn-block btn-ghost justify-start"
                                    aria-label="Cyberpunk"
                                    value="cyberpunk"
                                    prop:checked=move || selected_theme.get() == "cyberpunk"
                                    on:change=move |_| update_theme_sync("cyberpunk".to_string())
                                />
                            </ThemeController>
                        </li>
                    </ul>
                </div>
            </div>

            // Radio Button Theme Selector
            <div class="space-y-2">
                <h3 class="text-xl font-semibold">"Radio Button Theme Selector"</h3>
                <div class="flex flex-wrap gap-4">
                    <ThemeController theme_name="light">
                        <label class="label cursor-pointer gap-2">
                            <span class="label-text">"Light"</span>
                            <input
                                type="radio"
                                name="theme-radios"
                                class="radio"
                                value="light"
                                prop:checked=move || selected_theme.get() == "light"
                                on:change=move |_| update_theme_sync("light".to_string())
                            />
                        </label>
                    </ThemeController>

                    <ThemeController theme_name="dark">
                        <label class="label cursor-pointer gap-2">
                            <span class="label-text">"Dark"</span>
                            <input
                                type="radio"
                                name="theme-radios"
                                class="radio"
                                value="dark"
                                prop:checked=move || selected_theme.get() == "dark"
                                on:change=move |_| update_theme_sync("dark".to_string())
                            />
                        </label>
                    </ThemeController>

                    <ThemeController theme_name="retro">
                        <label class="label cursor-pointer gap-2">
                            <span class="label-text">"Retro"</span>
                            <input
                                type="radio"
                                name="theme-radios"
                                class="radio"
                                value="retro"
                                prop:checked=move || selected_theme.get() == "retro"
                                on:change=move |_| update_theme_sync("retro".to_string())
                            />
                        </label>
                    </ThemeController>

                    <ThemeController theme_name="forest">
                        <label class="label cursor-pointer gap-2">
                            <span class="label-text">"Forest"</span>
                            <input
                                type="radio"
                                name="theme-radios"
                                class="radio"
                                value="forest"
                                prop:checked=move || selected_theme.get() == "forest"
                                on:change=move |_| update_theme_sync("forest".to_string())
                            />
                        </label>
                    </ThemeController>
                </div>
            </div>

            // Toggle Theme Switch (Light/Dark)
            <div class="space-y-2">
                <h3 class="text-xl font-semibold">"Toggle Switch (Light/Dark)"</h3>
                <ThemeController theme_name="dark">
                    <label class="swap swap-rotate">
                        <input
                            type="checkbox"
                            prop:checked=move || selected_theme.get() == "dark"
                            on:change=move |ev| {
                                if event_target_checked(&ev) {
                                    update_theme_sync("dark".to_string());
                                } else {
                                    update_theme_sync("light".to_string());
                                }
                            }
                        />

                        // Sun icon
                        <svg
                            class="swap-on fill-current w-10 h-10"
                            xmlns="http://www.w3.org/2000/svg"
                            viewBox="0 0 24 24"
                        >
                            <path d="M5.64,17l-.71.71a1,1,0,0,0,0,1.41,1,1,0,0,0,1.41,0l.71-.71A1,1,0,0,0,5.64,17ZM5,12a1,1,0,0,0-1-1H3a1,1,0,0,0,0,2H4A1,1,0,0,0,5,12Zm7-7a1,1,0,0,0,1-1V3a1,1,0,0,0-2,0V4A1,1,0,0,0,12,5ZM5.64,7.05a1,1,0,0,0,.7.29,1,1,0,0,0,.71-.29,1,1,0,0,0,0-1.41l-.71-.71A1,1,0,0,0,4.93,6.34Zm12,.29a1,1,0,0,0,.7-.29l.71-.71a1,1,0,1,0-1.41-1.41L17,5.64a1,1,0,0,0,0,1.41A1,1,0,0,0,17.66,7.34ZM21,11H20a1,1,0,0,0,0,2h1a1,1,0,0,0,0-2Zm-9,8a1,1,0,0,0-1,1v1a1,1,0,0,0,2,0V20A1,1,0,0,0,12,19ZM18.36,17A1,1,0,0,0,17,18.36l.71.71a1,1,0,0,0,1.41,0,1,1,0,0,0,0-1.41ZM12,6.5A5.5,5.5,0,1,0,17.5,12,5.51,5.51,0,0,0,12,6.5Zm0,9A3.5,3.5,0,1,1,15.5,12,3.5,3.5,0,0,1,12,15.5Z" />
                        </svg>

                        // Moon icon
                        <svg
                            class="swap-off fill-current w-10 h-10"
                            xmlns="http://www.w3.org/2000/svg"
                            viewBox="0 0 24 24"
                        >
                            <path d="M21.64,13a1,1,0,0,0-1.05-.14,8.05,8.05,0,0,1-3.37.73A8.15,8.15,0,0,1,9.08,5.49a8.59,8.59,0,0,1,.25-2A1,1,0,0,0,8,2.36,10.14,10.14,0,1,0,22,14.05,1,1,0,0,0,21.64,13Zm-9.5,6.69A8.14,8.14,0,0,1,7.08,5.22v.27A10.15,10.15,0,0,0,17.22,15.63a9.79,9.79,0,0,0,2.1-.22A8.11,8.11,0,0,1,12.14,19.73Z" />
                        </svg>
                    </label>
                </ThemeController>
            </div>

            // Code Example
            <div class="space-y-2">
                <h3 class="text-xl font-semibold">"Usage Example"</h3>
                <div class="mockup-code text-sm">
                    <pre data-prefix=">">
                        <code>{"<ThemeController theme_name=\"dark\">"}</code>
                    </pre>
                    <pre data-prefix=">">
                        <code>
                            {"  <input type=\"radio\" name=\"theme\" class=\"radio\" />"}
                        </code>
                    </pre>
                    <pre data-prefix=">">
                        <code>"</ThemeController>"</code>
                    </pre>
                </div>
            </div>

            // Popular Themes List
            <div class="space-y-2">
                <h3 class="text-xl font-semibold">"Popular daisyUI Themes"</h3>
                <div class="overflow-x-auto">
                    <table class="table table-zebra">
                        <thead>
                            <tr>
                                <th>"Theme Name"</th>
                                <th>"Description"</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td><code>"light"</code></td>
                                <td>"Default light theme"</td>
                            </tr>
                            <tr>
                                <td><code>"dark"</code></td>
                                <td>"Default dark theme"</td>
                            </tr>
                            <tr>
                                <td><code>"cupcake"</code></td>
                                <td>"Pastel pink theme"</td>
                            </tr>
                            <tr>
                                <td><code>"cyberpunk"</code></td>
                                <td>"Neon cyberpunk theme"</td>
                            </tr>
                            <tr>
                                <td><code>"retro"</code></td>
                                <td>"Vintage retro theme"</td>
                            </tr>
                            <tr>
                                <td><code>"forest"</code></td>
                                <td>"Nature-inspired green theme"</td>
                            </tr>
                            <tr>
                                <td><code>"aqua"</code></td>
                                <td>"Ocean blue theme"</td>
                            </tr>
                            <tr>
                                <td><code>"dracula"</code></td>
                                <td>"Popular dark theme"</td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>

            <div class="alert alert-warning">
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    class="stroke-current shrink-0 h-6 w-6"
                    fill="none"
                    viewBox="0 0 24 24"
                >
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
                    />
                </svg>
                <span>
                    "Note: Theme changes in this demo are local to the component state. "
                    "For persistent theme switching, you'll need to configure daisyUI themes in your Tailwind config "
                    "and use JavaScript/localStorage to persist the selection."
                </span>
            </div>
        </div>
    }
}
