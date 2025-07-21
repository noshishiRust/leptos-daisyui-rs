use crate::core::{ContentLayout, Section};
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn RadioDemo() -> impl IntoView {
    let (selected_option, set_selected_option) = signal("option1".to_string());
    let (selected_color, set_selected_color) = signal("primary".to_string());
    let (selected_size, set_selected_size) = signal("md".to_string());

    view! {
        <ContentLayout
            title="Radio"
            description="Radio buttons allow users to select one option from a set"
        >
            <Section title="Basic Radio Buttons" col=true>
                <div class="flex flex-col gap-2">
                    <label class="label cursor-pointer justify-start gap-2">
                        <Radio
                            attr:name="radio-basic"
                            attr:value="option1"
                            prop:checked=move || selected_option.get() == "option1"
                            on:change=move |_| set_selected_option.set("option1".to_string())
                        />
                        <span class="label-text">"Option 1"</span>
                    </label>

                    <label class="label cursor-pointer justify-start gap-2">
                        <Radio
                            attr:name="radio-basic"
                            attr:value="option2"
                            prop:checked=move || selected_option.get() == "option2"
                            on:change=move |_| set_selected_option.set("option2".to_string())
                        />
                        <span class="label-text">"Option 2"</span>
                    </label>

                    <label class="label cursor-pointer justify-start gap-2">
                        <Radio
                            attr:name="radio-basic"
                            attr:value="option3"
                            prop:checked=move || selected_option.get() == "option3"
                            on:change=move |_| set_selected_option.set("option3".to_string())
                        />
                        <span class="label-text">"Option 3"</span>
                    </label>
                </div>

                <p class="text-sm text-base-content/70">
                    "Selected: " {move || selected_option.get()}
                </p>
            </Section>

            <Section title="Radio Button Colors" row=true>

                <label class="label cursor-pointer justify-start gap-2">
                    <Radio
                        attr:name="radio-color"
                        attr:value="default"
                        prop:checked=move || selected_color.get() == "default"
                        on:change=move |_| set_selected_color.set("default".to_string())
                    />
                </label>

                <label class="label cursor-pointer justify-start gap-2">
                    <Radio
                        color=RadioColor::Primary
                        attr:name="radio-color"
                        attr:value="primary"
                        prop:checked=move || selected_color.get() == "primary"
                        on:change=move |_| set_selected_color.set("primary".to_string())
                    />
                </label>

                <label class="label cursor-pointer justify-start gap-2">
                    <Radio
                        color=RadioColor::Secondary
                        attr:name="radio-color"
                        attr:value="secondary"
                        prop:checked=move || selected_color.get() == "secondary"
                        on:change=move |_| set_selected_color.set("secondary".to_string())
                    />
                </label>

                <label class="label cursor-pointer justify-start gap-2">
                    <Radio
                        color=RadioColor::Accent
                        attr:name="radio-color"
                        attr:value="accent"
                        prop:checked=move || selected_color.get() == "accent"
                        on:change=move |_| set_selected_color.set("accent".to_string())
                    />
                </label>

                <label class="label cursor-pointer justify-start gap-2">
                    <Radio
                        color=RadioColor::Success
                        attr:name="radio-color"
                        attr:value="success"
                        prop:checked=move || selected_color.get() == "success"
                        on:change=move |_| set_selected_color.set("success".to_string())
                    />
                </label>

                <label class="label cursor-pointer justify-start gap-2">
                    <Radio
                        color=RadioColor::Warning
                        attr:name="radio-color"
                        attr:value="warning"
                        prop:checked=move || selected_color.get() == "warning"
                        on:change=move |_| set_selected_color.set("warning".to_string())
                    />
                </label>

                <label class="label cursor-pointer justify-start gap-2">
                    <Radio
                        color=RadioColor::Info
                        attr:name="radio-color"
                        attr:value="info"
                        prop:checked=move || selected_color.get() == "info"
                        on:change=move |_| set_selected_color.set("info".to_string())
                    />
                </label>

                <label class="label cursor-pointer justify-start gap-2">
                    <Radio
                        color=RadioColor::Error
                        attr:name="radio-color"
                        attr:value="error"
                        prop:checked=move || selected_color.get() == "error"
                        on:change=move |_| set_selected_color.set("error".to_string())
                    />
                </label>
            </Section>

            <Section title="Radio Button Sizes" row=true>
                <label class="label cursor-pointer justify-start gap-2">
                    <Radio
                        size=RadioSize::Xs
                        attr:name="radio-size"
                        attr:value="xs"
                        prop:checked=move || selected_size.get() == "xs"
                        on:change=move |_| set_selected_size.set("xs".to_string())
                    />
                </label>

                <label class="label cursor-pointer justify-start gap-2">
                    <Radio
                        size=RadioSize::Sm
                        attr:name="radio-size"
                        attr:value="sm"
                        prop:checked=move || selected_size.get() == "sm"
                        on:change=move |_| set_selected_size.set("sm".to_string())
                    />
                </label>

                <label class="label cursor-pointer justify-start gap-2">
                    <Radio
                        size=RadioSize::Md
                        attr:name="radio-size"
                        attr:value="md"
                        prop:checked=move || selected_size.get() == "md"
                        on:change=move |_| set_selected_size.set("md".to_string())
                    />
                </label>

                <label class="label cursor-pointer justify-start gap-2">
                    <Radio
                        size=RadioSize::Lg
                        attr:name="radio-size"
                        attr:value="lg"
                        prop:checked=move || selected_size.get() == "lg"
                        on:change=move |_| set_selected_size.set("lg".to_string())
                    />
                </label>
                <label class="label cursor-pointer justify-start gap-2">
                    <Radio
                        size=RadioSize::Xl
                        attr:name="radio-size"
                        attr:value="xl"
                        prop:checked=move || selected_size.get() == "xl"
                        on:change=move |_| set_selected_size.set("xl".to_string())
                    />
                </label>

            </Section>

            <Section title="Disabled State" row=true>
                <label class="label cursor-pointer justify-start gap-2">
                    <Radio attr:name="radio-disabled" attr:disabled=true />
                    <span class="label-text">"Disabled unchecked"</span>
                </label>
                <label class="label cursor-pointer justify-start gap-2">
                    <Radio attr:name="radio-disabled" attr:disabled=true attr:checked=true />
                    <span class="label-text">"Disabled checked"</span>
                </label>
            </Section>

            <Section title="Radio Group Example" col=true>
                <div class="space-x-2">
                    <label class="label cursor-pointer justify-start gap-2">
                        <Radio color=RadioColor::Primary attr:name="framework" />
                        <span class="label-text">"Leptos (Rust)"</span>
                    </label>

                    <label class="label cursor-pointer justify-start gap-2">
                        <Radio color=RadioColor::Primary attr:name="framework" />
                        <span class="label-text">"React (JavaScript)"</span>
                    </label>

                    <label class="label cursor-pointer justify-start gap-2">
                        <Radio color=RadioColor::Primary attr:name="framework" />
                        <span class="label-text">"Vue (JavaScript)"</span>
                    </label>

                    <label class="label cursor-pointer justify-start gap-2">
                        <Radio color=RadioColor::Primary attr:name="framework" />
                        <span class="label-text">"Svelte (JavaScript)"</span>
                    </label>
                </div>
                <div class="card-actions justify-end mt-4">
                    <Button color=ButtonColor::Primary>"Submit"</Button>
                </div>
            </Section>
        </ContentLayout>
    }
}
