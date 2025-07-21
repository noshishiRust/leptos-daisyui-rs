use crate::core::{ContentLayout, Section};
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn RangeDemo() -> impl IntoView {
    let (value1, set_value1) = signal(25.0);
    let (value2, set_value2) = signal(40.0);
    let (value3, set_value3) = signal(60.0);

    view! {
        <ContentLayout
            title="Range"
            description="Range slider is used to select a value by sliding a handle"
        >

            <Section title="Basic Range">"coming soon..."</Section>
        </ContentLayout>
    }
}
