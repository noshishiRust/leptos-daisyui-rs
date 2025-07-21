use crate::core::{ContentLayout, Section};
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn ToggleDemo() -> impl IntoView {
    let (checked1, set_checked1) = signal(false);
    let (checked2, set_checked2) = signal(true);

    view! {
        <ContentLayout title="Toggle" description="Toggle is used to switch between two states">

            <Section row=true title="Basic Toggle">
                <Toggle
                    attr:checked=checked1
                    on:change=move |ev| set_checked1.set(event_target_checked(&ev))
                />
                <Toggle
                    attr:checked=checked2
                    on:change=move |ev| set_checked2.set(event_target_checked(&ev))
                />
                <Toggle attr:disabled=true />
                <Toggle attr:disabled=true attr:checked=true />
            </Section>

            <Section row=true title="Colors">
                <Toggle attr:checked=true />
                <Toggle color=ToggleColor::Primary attr:checked=true />
                <Toggle color=ToggleColor::Secondary attr:checked=true />
                <Toggle color=ToggleColor::Accent attr:checked=true />
                <Toggle color=ToggleColor::Success attr:checked=true />
                <Toggle color=ToggleColor::Warning attr:checked=true />
                <Toggle color=ToggleColor::Info attr:checked=true />
                <Toggle color=ToggleColor::Error attr:checked=true />
            </Section>

            <Section row=true title="Sizes">
                <Toggle size=ToggleSize::Xs attr:checked=true />
                <Toggle size=ToggleSize::Sm attr:checked=true />
                <Toggle size=ToggleSize::Md attr:checked=true />
                <Toggle size=ToggleSize::Lg attr:checked=true />
                <Toggle size=ToggleSize::Xl attr:checked=true />
            </Section>

        </ContentLayout>
    }
}
