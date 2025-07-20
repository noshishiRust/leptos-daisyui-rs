use crate::core::ContentLayout;
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn AccordionDemo() -> impl IntoView {
    view! {
        <ContentLayout
            title="Accordion"
            description="Accordion is used for showing and hiding content but only one item can stay open at a time"
        >

            <h2 class="text-xl font-semibold">"Examples"</h2>
            <Accordion
                name="demo1"
                modifier=AccordionModifier::Arrow
                class="border border-base-300"
            >
                <AccordionTitle>"What is daisyUI?"</AccordionTitle>
                <AccordionContent>
                    <p>
                        "daisyUI is a Tailwind CSS component library that provides semantic class names for common UI components."
                    </p>
                </AccordionContent>
            </Accordion>

            <Accordion
                name="demo1"
                modifier=AccordionModifier::Arrow
                class="border border-base-300"
            >
                <AccordionTitle>"How do I use it with Leptos?"</AccordionTitle>
                <AccordionContent>
                    <p>
                        "This library provides Leptos components that wrap daisyUI classes with type-safe, reactive props."
                    </p>
                </AccordionContent>
            </Accordion>

            <Accordion
                name="demo1"
                modifier=AccordionModifier::Arrow
                class="border border-base-300"
            >
                <AccordionTitle>"Plus style accordion"</AccordionTitle>
                <AccordionContent>
                    <p>"This accordion uses the plus modifier for a different icon style."</p>
                </AccordionContent>
            </Accordion>
        </ContentLayout>
    }
}
