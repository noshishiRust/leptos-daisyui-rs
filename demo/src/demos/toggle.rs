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
                        on:change=move |ev| set_checked1.set(event_target_checked(&ev))
                    />
                    <Toggle
                        checked=Signal::derive(move || checked2.get())
                        on:change=move |ev| set_checked2.set(event_target_checked(&ev))
                    />
                </div>

                <h2 class="text-xl font-semibold">"Colors"</h2>
                <div class="flex flex-wrap gap-2">
                    <Toggle checked=true />
                    <Toggle color=ToggleColor::Primary checked=true />
                    <Toggle color=ToggleColor::Secondary checked=true />
                    <Toggle color=ToggleColor::Accent checked=true />
                    <Toggle color=ToggleColor::Success checked=true />
                    <Toggle color=ToggleColor::Warning checked=true />
                    <Toggle color=ToggleColor::Info checked=true />
                    <Toggle color=ToggleColor::Error checked=true />
                </div>

                <h2 class="text-xl font-semibold">"Sizes"</h2>
                <div class="flex items-center gap-2">
                    <Toggle size=ToggleSize::Xs checked=true />
                    <Toggle size=ToggleSize::Sm checked=true />
                    <Toggle size=ToggleSize::Md checked=true />
                    <Toggle size=ToggleSize::Lg checked=true />
                </div>
            </div>
        </div>
    }
}
