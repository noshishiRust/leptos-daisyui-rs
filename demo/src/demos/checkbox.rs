use crate::core::{ContentLayout, Section};
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn CheckboxDemo() -> impl IntoView {
    let checked_state = RwSignal::new(false);
    let selected_color = RwSignal::new(CheckboxColor::Primary);
    let selected_size = RwSignal::new(CheckboxSize::Md);

    view! {
        <ContentLayout
            title="Checkbox"
            description="Checkboxes are used to select one or multiple options from a list"
        >
            <Section row=true title="Colors">
                <Checkbox attr:checked=true />
                <Checkbox color=CheckboxColor::Primary attr:checked=true />
                <Checkbox color=CheckboxColor::Secondary attr:checked=true />
                <Checkbox color=CheckboxColor::Accent attr:checked=true />
                <Checkbox color=CheckboxColor::Success attr:checked=true />
                <Checkbox color=CheckboxColor::Warning attr:checked=true />
                <Checkbox color=CheckboxColor::Info attr:checked=true />
                <Checkbox color=CheckboxColor::Error attr:checked=true />
            </Section>

            <Section row=true title="Sizes">
                <Checkbox size=CheckboxSize::Xs attr:checked=true />
                <Checkbox size=CheckboxSize::Sm attr:checked=true />
                <Checkbox size=CheckboxSize::Md attr:checked=true />
                <Checkbox size=CheckboxSize::Lg attr:checked=true />
                <Checkbox size=CheckboxSize::Xl attr:checked=true />
            </Section>

            <Section row=true title="States">
                <Checkbox />
                <Checkbox attr:checked=true />
                <Checkbox attr:disabled=true />
                <Checkbox attr:disabled=true attr:checked=true />
            </Section>

            <Section title="Interactive Example" col=true>
                <label class="cursor-pointer label">
                    <span class="label-text">
                        {move || { if checked_state.get() { "Checked" } else { "Unchecked" } }}
                    </span>
                    <Checkbox
                        color=selected_color
                        size=selected_size
                        prop:checked=move || checked_state.get()
                        on:change=move |ev| checked_state.set(event_target_checked(&ev))
                    />
                </label>
            </Section>
        </ContentLayout>
    }
}
