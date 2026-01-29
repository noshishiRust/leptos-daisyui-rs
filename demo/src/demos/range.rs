use crate::core::{ContentLayout, Section};
use leptos::prelude::*;

#[component]
pub fn RangeDemo() -> impl IntoView {
    let (_value1, _set_value1) = signal(25.0);
    let (_value2, _set_value2) = signal(40.0);
    let (_value3, _set_value3) = signal(60.0);

    view! {
        <ContentLayout
            title="Range"
            description="Range slider is used to select a value by sliding a handle"
        >

            <Section title="Basic Range">"coming soon..."</Section>
        </ContentLayout>
    }
}
