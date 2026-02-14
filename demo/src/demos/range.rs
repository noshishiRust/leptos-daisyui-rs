use crate::core::{ContentLayout, Section};
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn RangeDemo() -> impl IntoView {
    let a = view! {
        <>
            <Accordion>
                <AccordionTitle>"aa"</AccordionTitle>
            </Accordion>
        </>
    }
    .into_any();
    let b = a.to_html();

    view! {
        <ContentLayout
            title="Range"
            description="Range slider is used to select a value by sliding a handle"
        >
            <Section title="Basic Range">"coming soon..."</Section>
            <code>{b}</code>
        </ContentLayout>
    }
}
