use crate::core::{ContentLayout, Section};
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn InputDemo() -> impl IntoView {
    let (value, set_value) = signal("".to_string());

    view! {
        <ContentLayout
            title="Input"
            description="Inputs are used to get user input in a text field"
        >
            <Section title="Basic Input">
                <Input
                    attr:placeholder="Type here"
                    attr:value=value
                    on:input=move |ev| set_value.set(event_target_value(&ev))
                    class="w-full max-w-xs"
                />
            </Section>

            <Section title="Colors">
                <div class="space-y-2">
                    <Input attr:placeholder="Default" class="w-full max-w-xs" />
                    <Input
                        color=InputColor::Primary
                        attr:placeholder="Primary"
                        class="w-full max-w-xs"
                    />
                    <Input
                        color=InputColor::Secondary
                        attr:placeholder="Secondary"
                        class="w-full max-w-xs"
                    />
                    <Input
                        color=InputColor::Accent
                        attr:placeholder="Accent"
                        class="w-full max-w-xs"
                    />
                    <Input color=InputColor::Info attr:placeholder="Info" class="w-full max-w-xs" />
                    <Input
                        color=InputColor::Success
                        attr:placeholder="Success"
                        class="w-full max-w-xs"
                    />
                    <Input
                        color=InputColor::Warning
                        attr:placeholder="Warning"
                        class="w-full max-w-xs"
                    />
                    <Input
                        color=InputColor::Error
                        attr:placeholder="Error"
                        class="w-full max-w-xs"
                    />
                </div>
            </Section>

            <Section title="Sizes">
                <div class="space-y-2">
                    <Input size=InputSize::Xs attr:placeholder="XS" class="w-full max-w-xs" />
                    <Input size=InputSize::Sm attr:placeholder="SM" class="w-full max-w-xs" />
                    <Input size=InputSize::Md attr:placeholder="MD" class="w-full max-w-xs" />
                    <Input size=InputSize::Lg attr:placeholder="LG" class="w-full max-w-xs" />
                </div>
            </Section>

            <Section title="Styles">
                <div class="space-y-2">
                    <Input attr:placeholder="Default" class="w-full max-w-xs" />
                    <Input attr:placeholder="Bordered" class="w-full max-w-xs" />
                    <Input
                        style=InputStyle::Ghost
                        attr:placeholder="Ghost"
                        class="w-full max-w-xs"
                    />
                </div>
            </Section>
        </ContentLayout>
    }
}
