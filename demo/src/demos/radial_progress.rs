use crate::core::{ContentLayout, Section};
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn RadialProgressDemo() -> impl IntoView {
    view! {
        <ContentLayout
            title="Radial Progress"
            description="Radial progress shows the progress in a circle format"
        >
            <Section title="Basic Radial Progress" row=true>
                <RadialProgress value=0.0 />
                <RadialProgress value=20.0 />
                <RadialProgress value=60.0 />
                <RadialProgress value=80.0 />
                <RadialProgress value=100.0 />
            </Section>

            <Section title="With Text" row=true>
                <RadialProgress value=70.0 class="text-primary">
                    "70%"
                </RadialProgress>
                <RadialProgress value=85.0 class="text-secondary">
                    "85%"
                </RadialProgress>
                <RadialProgress value=90.0 class="text-accent">
                    "90%"
                </RadialProgress>
            </Section>

            <Section title="Colors" row=true>
                <RadialProgress value=70.0 color=RadialProgressColor::Primary>
                    "70%"
                </RadialProgress>
                <RadialProgress value=70.0 color=RadialProgressColor::Secondary>
                    "70%"
                </RadialProgress>
                <RadialProgress value=70.0 color=RadialProgressColor::Accent>
                    "70%"
                </RadialProgress>
                <RadialProgress value=70.0 color=RadialProgressColor::Info>
                    "70%"
                </RadialProgress>
                <RadialProgress value=70.0 color=RadialProgressColor::Success>
                    "70%"
                </RadialProgress>
                <RadialProgress value=70.0 color=RadialProgressColor::Warning>
                    "70%"
                </RadialProgress>
                <RadialProgress value=70.0 color=RadialProgressColor::Error>
                    "70%"
                </RadialProgress>
            </Section>
        </ContentLayout>
    }
}
