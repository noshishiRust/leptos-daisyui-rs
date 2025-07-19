use leptos::prelude::*;
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
                            checked=Signal::derive(move || checked1.get())
                            on_change=Box::new(move |value| set_checked1.set(value))
                        />
                    </label>
                    <label class="cursor-pointer label">
                        <span class="label-text">"Checked"</span>
                        <Checkbox
                            checked=Signal::derive(move || checked2.get())
                            on_change=Box::new(move |value| set_checked2.set(value))
                        />
                    </label>
                </div>

                <h2 class="text-xl font-semibold">"Colors"</h2>
                <div class="flex flex-wrap gap-2">
                    <Checkbox checked=true />
                    <Checkbox color=CheckboxColor::Primary checked=true />
                    <Checkbox color=CheckboxColor::Secondary checked=true />
                    <Checkbox color=CheckboxColor::Accent checked=true />
                    <Checkbox color=CheckboxColor::Success checked=true />
                    <Checkbox color=CheckboxColor::Warning checked=true />
                    <Checkbox color=CheckboxColor::Info checked=true />
                    <Checkbox color=CheckboxColor::Error checked=true />
                </div>

                <h2 class="text-xl font-semibold">"Sizes"</h2>
                <div class="flex items-center gap-2">
                    <Checkbox size=CheckboxSize::Xs checked=true />
                    <Checkbox size=CheckboxSize::Sm checked=true />
                    <Checkbox size=CheckboxSize::Md checked=true />
                    <Checkbox size=CheckboxSize::Lg checked=true />
                </div>
            </div>
        </div>
    }
}