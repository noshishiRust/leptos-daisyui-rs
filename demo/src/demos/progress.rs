use crate::core::{ContentLayout, Section};
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn ProgressDemo() -> impl IntoView {
    view! {
        <ContentLayout
            title="Progress"
            description="Progress bars show the progress of a task or show the loading state"
        >

            <Section title="Colors" col=true>
                <Progress attr:value=70.0 attr:max=100.0 class="w-56" />
                <Progress
                    color=ProgressColor::Primary
                    attr:value=70.0
                    attr:max=100.0
                    class="w-56"
                />
                <Progress
                    color=ProgressColor::Secondary
                    attr:value=70.0
                    attr:max=100.0
                    class="w-56"
                />
                <Progress color=ProgressColor::Accent attr:value=70.0 attr:max=100.0 class="w-56" />
                <Progress color=ProgressColor::Info attr:value=70.0 attr:max=100.0 class="w-56" />
                <Progress
                    color=ProgressColor::Success
                    attr:value=70.0
                    attr:max=100.0
                    class="w-56"
                />
                <Progress
                    color=ProgressColor::Warning
                    attr:value=70.0
                    attr:max=100.0
                    class="w-56"
                />
                <Progress color=ProgressColor::Error attr:value=70.0 attr:max=100.0 class="w-56" />
            </Section>

            <Section title="Indeterminate Progress">
                <Progress class="w-56" />
            </Section>

        </ContentLayout>
    }
}
