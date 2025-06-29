use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn TextareaDemo() -> impl IntoView {
    let (value, set_value) = signal("initial text".to_string());

    view! {
        <div class="space-y-6">
            <h1 class="text-3xl font-bold">"Textarea"</h1>
            <p class="text-base-content/70">"Textarea is used to get multi-line user input"</p>

            <div class="space-y-4">
                <h2 class="text-xl font-semibold">"Basic Textarea"</h2>
                <Textarea
                    prop:prop:placeholder="Bio"
                    prop:value=Signal::derive(move || value.get())
                    on:input=move |ev| set_value.set(event_target_value(&ev))
                    class="w-full max-w-xs"
                />

                <h2 class="text-xl font-semibold">"Colors"</h2>
                <div class="space-y-2">
                    <Textarea prop:placeholder="Default" class="w-full max-w-xs" />
                    <Textarea
                        color=Signal::derive(move || TextareaColor::Primary)
                        class="w-full max-w-xs"
                        prop:disabled=true
                        prop:placeholder="Primary"
                    />
                    <Textarea
                        color=Signal::derive(move || TextareaColor::Secondary)
                        prop:placeholder="Secondary"
                        class="w-full max-w-xs"
                    />
                    <Textarea
                        color=Signal::derive(move || TextareaColor::Accent)
                        prop:placeholder="Accent"
                        class="w-full max-w-xs"
                    />
                </div>

                <h2 class="text-xl font-semibold">"Sizes"</h2>
                <div class="space-y-2">
                    <Textarea
                        size=Signal::derive(move || TextareaSize::Xs)
                        prop:placeholder="XS"
                        class="w-full max-w-xs"
                    />
                    <Textarea
                        size=Signal::derive(move || TextareaSize::Sm)
                        prop:placeholder="SM"
                        class="w-full max-w-xs"
                    />
                    <Textarea
                        size=Signal::derive(move || TextareaSize::Md)
                        prop:placeholder="MD"
                        class="w-full max-w-xs"
                    />
                    <Textarea
                        size=Signal::derive(move || TextareaSize::Lg)
                        prop:placeholder="LG"
                        class="w-full max-w-xs"
                    />
                </div>
            </div>
        </div>
    }
}
