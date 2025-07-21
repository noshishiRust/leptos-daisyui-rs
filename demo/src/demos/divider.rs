use crate::core::{ContentLayout, Section};
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn DividerDemo() -> impl IntoView {
    let direction = RwSignal::new(DividerDirection::Horizontal);
    let color = RwSignal::new(DividerColor::Primary);
    let text_content = RwSignal::new("OR");

    view! {
        <ContentLayout
            title="Divider"
            description="Divider visually separates content in lists or groups"
        >

            <Section col=true title="Basic Usage">
                "coming soon..."
            </Section>

            <Section col=true title="Orientations">
                "coming soon..."
            </Section>

            <Section col=true title="Colors">
                "coming soon..."
            </Section>

        </ContentLayout>
    }
}
