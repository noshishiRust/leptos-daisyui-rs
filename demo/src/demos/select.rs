use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn SelectDemo() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <h1 class="text-3xl font-bold">"Select"</h1>
            <p class="text-base-content/70">
                "Select is used to pick a value from a list of options"
            </p>

            <div class="space-y-4">
                <h2 class="text-xl font-semibold">"Basic Select"</h2>
                <Select class="w-full max-w-xs">
                    <option disabled selected>
                        "Pick your favorite language"
                    </option>
                    <option>"Rust"</option>
                    <option>"TypeScript"</option>
                    <option>"Python"</option>
                    <option>"JavaScript"</option>
                </Select>

                <h2 class="text-xl font-semibold">"Colors"</h2>
                <div class="space-y-2">
                    <Select class="w-full max-w-xs">
                        <option disabled selected>
                            "Default"
                        </option>
                        <option>"Option 1"</option>
                        <option>"Option 2"</option>
                    </Select>
                    <Select
                        color=SelectColor::Primary
                        class="w-full max-w-xs"
                    >
                        <option disabled selected>
                            "Primary"
                        </option>
                        <option>"Option 1"</option>
                        <option>"Option 2"</option>
                    </Select>
                    <Select
                        color=SelectColor::Secondary
                        class="w-full max-w-xs"
                    >
                        <option disabled selected>
                            "Secondary"
                        </option>
                        <option>"Option 1"</option>
                        <option>"Option 2"</option>
                    </Select>
                </div>

                <h2 class="text-xl font-semibold">"Sizes"</h2>
                <div class="space-y-2">
                    <Select size=SelectSize::Xs class="w-full max-w-xs">
                        <option disabled selected>
                            "XS"
                        </option>
                        <option>"Option 1"</option>
                    </Select>
                    <Select size=SelectSize::Sm class="w-full max-w-xs">
                        <option disabled selected>
                            "SM"
                        </option>
                        <option>"Option 1"</option>
                    </Select>
                    <Select size=SelectSize::Md class="w-full max-w-xs">
                        <option disabled selected>
                            "MD"
                        </option>
                        <option>"Option 1"</option>
                    </Select>
                    <Select size=SelectSize::Lg class="w-full max-w-xs">
                        <option disabled selected>
                            "LG"
                        </option>
                        <option>"Option 1"</option>
                    </Select>
                </div>
            </div>
        </div>
    }
}