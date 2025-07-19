use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn InputDemo() -> impl IntoView {
    let (value, set_value) = signal("".to_string());

    view! {
        <div class="space-y-6">
            <h1 class="text-3xl font-bold">"Input"</h1>
            <p class="text-base-content/70">"Inputs are used to get user input in a text field"</p>

            <div class="space-y-4">
                <h2 class="text-xl font-semibold">"Basic Input"</h2>
                <input
                    placeholder="Type here"
                    value=value
                    on:input=move |ev| set_value.set(event_target_value(&ev))
                    class="input input-bordered w-full max-w-xs"
                />

                <h2 class="text-xl font-semibold">"Colors"</h2>
                <div class="space-y-2">
                    <Input placeholder="Default" class="w-full max-w-xs" />
                    <Input
                        color=InputColor::Primary
                        placeholder="Primary"
                        class="w-full max-w-xs"
                    />
                    <Input
                        color=InputColor::Secondary
                        placeholder="Secondary"
                        class="w-full max-w-xs"
                    />
                    <Input color=InputColor::Accent placeholder="Accent" class="w-full max-w-xs" />
                    <Input color=InputColor::Info placeholder="Info" class="w-full max-w-xs" />
                    <Input
                        color=InputColor::Success
                        placeholder="Success"
                        class="w-full max-w-xs"
                    />
                    <Input
                        color=InputColor::Warning
                        placeholder="Warning"
                        class="w-full max-w-xs"
                    />
                    <Input color=InputColor::Error placeholder="Error" class="w-full max-w-xs" />
                </div>

                <h2 class="text-xl font-semibold">"Sizes"</h2>
                <div class="space-y-2">
                    <Input size=InputSize::Xs placeholder="XS" class="w-full max-w-xs" />
                    <Input size=InputSize::Sm placeholder="SM" class="w-full max-w-xs" />
                    <Input size=InputSize::Md placeholder="MD" class="w-full max-w-xs" />
                    <Input size=InputSize::Lg placeholder="LG" class="w-full max-w-xs" />
                </div>

                <h2 class="text-xl font-semibold">"Styles"</h2>
                <div class="space-y-2">
                    <Input placeholder="Default" class="w-full max-w-xs" />
                    <Input placeholder="Bordered" class="w-full max-w-xs" />
                    <Input style=InputStyle::Ghost placeholder="Ghost" class="w-full max-w-xs" />
                </div>
            </div>
        </div>
    }
}