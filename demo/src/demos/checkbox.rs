use leptos::{ev, prelude::*};
use leptos_daisyui_rs::components::*;

#[component]
pub fn CheckboxDemo() -> impl IntoView {
    let (checked1, set_checked1) = signal(false);
    let (checked2, set_checked2) = signal(true);

    view! {
        <div class="space-y-6">
            <h1 class="text-3xl font-bold">"Checkbox"</h1>
            <p class="text-base-content/70">
                "Checkboxes are used to select one or multiple options from a list"
            </p>

            <div class="space-y-4">
                <h2 class="text-xl font-semibold">"Basic Checkboxes"</h2>
                <div class="flex flex-col gap-2">
                    <label class="cursor-pointer label">
                        <span class="label-text">"Unchecked"</span>
                        <Checkbox
                            attr:checked=Signal::derive(move || checked1.get())
                            on:change=move |ev| set_checked1.set(event_target_checked(&ev))
                        />
                    </label>
                    <label class="cursor-pointer label">
                        <span class="label-text">"Checked"</span>
                        <Checkbox
                            attr:checked=Signal::derive(move || checked2.get())
                            on:change=move |ev| set_checked2.set(event_target_checked(&ev))
                        />
                    </label>
                </div>

                <h2 class="text-xl font-semibold">"Colors"</h2>
                <div class="flex flex-wrap gap-2">
                    <Checkbox attr:checked=true />
                    <Checkbox color=CheckboxColor::Primary attr:checked=true />
                    <Checkbox color=CheckboxColor::Secondary attr:checked=true />
                    <Checkbox color=CheckboxColor::Accent attr:checked=true />
                    <Checkbox color=CheckboxColor::Success attr:checked=true />
                    <Checkbox color=CheckboxColor::Warning attr:checked=true />
                    <Checkbox color=CheckboxColor::Info attr:checked=true />
                    <Checkbox color=CheckboxColor::Error attr:checked=true />
                </div>

                <h2 class="text-xl font-semibold">"Sizes"</h2>
                <div class="flex items-center gap-2">
                    <Checkbox size=CheckboxSize::Xs attr:checked=true />
                    <Checkbox size=CheckboxSize::Sm attr:checked=true />
                    <Checkbox size=CheckboxSize::Md attr:checked=true />
                    <Checkbox size=CheckboxSize::Lg attr:checked=true />
                </div>
            </div>
        </div>
    }
}
