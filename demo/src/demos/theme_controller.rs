use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn ThemeControllerDemo() -> impl IntoView {
    let (current_theme, set_current_theme) = signal("light".to_string());
    
    let themes = vec![
        "light", "dark", "cupcake", "bumblebee", "emerald", "corporate", 
        "synthwave", "retro", "cyberpunk", "valentine", "halloween", "garden",
        "forest", "aqua", "lofi", "pastel", "fantasy", "wireframe", "black",
        "luxury", "dracula", "cmyk", "autumn", "business", "acid", "lemonade",
        "night", "coffee", "winter", "dim", "nord", "sunset"
    ];

    view! {
        <div class="space-y-6">
            <h1 class="text-3xl font-bold">"Theme Controller"</h1>
            <p class="text-base-content/70">
                "Theme controller allows users to change the theme of the website"
            </p>

            <div class="space-y-4">
                <h2 class="text-xl font-semibold">
                    "Current Theme: " {move || current_theme.get()}
                </h2>

                <h2 class="text-xl font-semibold">"Toggle Themes"</h2>
                <div class="flex gap-2">
                    <ThemeController
                        theme_name="light"
                        checked=Signal::derive(move || current_theme.get() == "light")
                    />
                    <span class="text-sm self-center">"Light"</span>

                    <ThemeController
                        theme_name="dark"
                        checked=Signal::derive(move || current_theme.get() == "dark")
                    />
                    <span class="text-sm self-center">"Dark"</span>
                </div>

                <h2 class="text-xl font-semibold">"Theme Dropdown"</h2>
                <div class="dropdown">
                    <div tabindex="0" role="button" class="btn m-1">
                        "Theme: "
                        {move || current_theme.get()}
                        <svg
                            width="12"
                            height="12"
                            class="inline-block h-2 w-2 fill-current opacity-60"
                            xmlns="http://www.w3.org/2000/svg"
                            viewBox="0 0 2048 2048"
                        >
                            <path d="M1799 349l242 241-1017 1017L7 590l242-241 775 775 775-775z"></path>
                        </svg>
                    </div>
                    <ul
                        tabindex="0"
                        class="dropdown-content bg-base-300 rounded-box z-[1] w-52 p-2 shadow-2xl"
                    >
                        {themes
                            .into_iter()
                            .map(|theme| {
                                let theme = theme.to_string();
                                let theme_clone = theme.clone();
                                view! {
                                    <li>
                                        <input
                                            type="radio"
                                            name="theme-dropdown"
                                            class="theme-controller btn btn-sm btn-block btn-ghost justify-start"
                                            aria-label=theme.clone()
                                            value=theme.clone()
                                            on:change=move |_| {
                                                set_current_theme.set(theme_clone.clone());
                                            }
                                        />
                                    </li>
                                }
                            })
                            .collect::<Vec<_>>()}
                    </ul>
                </div>

                <h2 class="text-xl font-semibold">"Theme Preview Cards"</h2>
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                    <Card class="bg-base-100 shadow-xl">
                        <CardBody>
                            <h2 class="card-title">"Sample Card"</h2>
                            <p>"This card changes with the theme"</p>
                            <div class="card-actions justify-end">
                                <Button color=ButtonColor::Primary>"Primary"</Button>
                                <Button color=ButtonColor::Secondary>"Secondary"</Button>
                            </div>
                        </CardBody>
                    </Card>

                    <Alert color=AlertColor::Info>
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
                        <span>"Theme affects all components!"</span>
                    </Alert>

                    <div class="bg-base-200 p-4 rounded-lg">
                        <h3 class="font-bold text-lg">"Colors Preview"</h3>
                        <div class="flex gap-1 mt-2">
                            <div class="w-4 h-4 bg-primary rounded"></div>
                            <div class="w-4 h-4 bg-secondary rounded"></div>
                            <div class="w-4 h-4 bg-accent rounded"></div>
                            <div class="w-4 h-4 bg-neutral rounded"></div>
                        </div>
                    </div>
                </div>

                <h2 class="text-xl font-semibold">"Swap-based Theme Toggle"</h2>
                <label class="swap swap-rotate">
                    <input
                        type="checkbox"
                        class="theme-controller"
                        value="dark"
                        on:change=move |ev| {
                            let checked = event_target_checked(&ev);
                            set_current_theme
                                .set(
                                    if checked { "dark".to_string() } else { "light".to_string() },
                                );
                        }
                    />

                    <svg
                        class="swap-off fill-current w-8 h-8"
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 24 24"
                    >
                        <path d="M5.64,17l-.71.71a1,1,0,0,0,0,1.41,1,1,0,0,0,1.41,0l.71-.71A1,1,0,0,0,5.64,17ZM5,12a1,1,0,0,0-1-1H3a1,1,0,0,0,0,2H4A1,1,0,0,0,5,12Zm7-7a1,1,0,0,0,1-1V3a1,1,0,0,0-2,0V4A1,1,0,0,0,12,5ZM5.64,7.05a1,1,0,0,0,.7.29,1,1,0,0,0,.71-.29,1,1,0,0,0,0-1.41l-.71-.71A1,1,0,0,0,4.93,6.34Zm12,.29a1,1,0,0,0,.7-.29l.71-.71a1,1,0,1,0-1.41-1.41L17,5.64a1,1,0,0,0,0,1.41A1,1,0,0,0,17.66,7.34ZM21,11H20a1,1,0,0,0,0,2h1a1,1,0,0,0,0-2Zm-9,8a1,1,0,0,0-1,1v1a1,1,0,0,0,2,0V20A1,1,0,0,0,12,19ZM18.36,17A1,1,0,0,0,17,18.36l.71.71a1,1,0,0,0,1.41,0,1,1,0,0,0,0-1.41ZM12,6.5A5.5,5.5,0,1,0,17.5,12,5.51,5.51,0,0,0,12,6.5Zm0,9A3.5,3.5,0,1,1,15.5,12,3.5,3.5,0,0,1,12,15.5Z" />
                    </svg>

                    <svg
                        class="swap-on fill-current w-8 h-8"
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 24 24"
                    >
                        <path d="M21.64,13a1,1,0,0,0-1.05-.14,8.05,8.05,0,0,1-3.37.73A8.15,8.15,0,0,1,9.08,5.49a8.59,8.59,0,0,1,.25-2A1,1,0,0,0,8,2.36,10.14,10.14,0,1,0,22,14.05,1,1,0,0,0,21.64,13Zm-9.5,6.69A8.14,8.14,0,0,1,7.08,5.22v.27A10.15,10.15,0,0,0,17.22,15.63a9.79,9.79,0,0,0,2.1-.22A8.11,8.11,0,0,1,12.14,19.73Z" />
                    </svg>
                </label>
            </div>
        </div>
    }
}