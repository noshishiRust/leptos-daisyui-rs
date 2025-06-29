use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn ToggleDemo() -> impl IntoView {
    let (checked1, set_checked1) = signal(false);
    let (checked2, set_checked2) = signal(true);

    view! {
        <div class="space-y-6">
            <h1 class="text-3xl font-bold">"Toggle"</h1>
            <p class="text-base-content/70">"Toggle is used to switch between two states"</p>

            <div class="space-y-4">
                <h2 class="text-xl font-semibold">"Basic Toggle"</h2>
                <div class="flex flex-col gap-2">
                    <Toggle
                        checked=Signal::derive(move || checked1.get())
                        on_change=Callback::new(move |value| set_checked1.set(value))
                    />
                    <Toggle
                        checked=Signal::derive(move || checked2.get())
                        on_change=Callback::new(move |value| set_checked2.set(value))
                    />
                </div>

                <h2 class="text-xl font-semibold">"Colors"</h2>
                <div class="flex flex-wrap gap-2">
                    <Toggle checked=Signal::derive(move || true) />
                    <Toggle
                        color=Signal::derive(move || ToggleColor::Primary)
                        checked=Signal::derive(move || true)
                    />
                    <Toggle
                        color=Signal::derive(move || ToggleColor::Secondary)
                        checked=Signal::derive(move || true)
                    />
                    <Toggle
                        color=Signal::derive(move || ToggleColor::Accent)
                        checked=Signal::derive(move || true)
                    />
                    <Toggle
                        color=Signal::derive(move || ToggleColor::Success)
                        checked=Signal::derive(move || true)
                    />
                    <Toggle
                        color=Signal::derive(move || ToggleColor::Warning)
                        checked=Signal::derive(move || true)
                    />
                    <Toggle
                        color=Signal::derive(move || ToggleColor::Info)
                        checked=Signal::derive(move || true)
                    />
                    <Toggle
                        color=Signal::derive(move || ToggleColor::Error)
                        checked=Signal::derive(move || true)
                    />
                </div>

                <h2 class="text-xl font-semibold">"Sizes"</h2>
                <div class="flex items-center gap-2">
                    <Toggle
                        size=Signal::derive(move || ToggleSize::Xs)
                        checked=Signal::derive(move || true)
                    />
                    <Toggle
                        size=Signal::derive(move || ToggleSize::Sm)
                        checked=Signal::derive(move || true)
                    />
                    <Toggle
                        size=Signal::derive(move || ToggleSize::Md)
                        checked=Signal::derive(move || true)
                    />
                    <Toggle
                        size=Signal::derive(move || ToggleSize::Lg)
                        checked=Signal::derive(move || true)
                    />
                </div>
            </div>
        </div>
    }
}