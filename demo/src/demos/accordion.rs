use crate::core::{ContentLayout, Section};
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn AccordionDemo() -> impl IntoView {
    view! {
        <ContentLayout
            title="Accordion"
            description="Accordion is used for showing and hiding content but only one item can stay open at a time"
        >
            <Section title="Basic Accordion">
                <div class="max-w-md">
                    <Accordion
                        name="demo1"
                        checked=true
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
                </div>
            </Section>

            <Section title="Modifiers">
                <div class="max-w-md space-y-2">
                    <Accordion
                        name="demo2"
                        modifier=AccordionModifier::Default
                        class="border border-base-300"
                    >
                        <AccordionTitle>"Default (no icon)"</AccordionTitle>
                        <AccordionContent>
                            <p>"No visual indicator"</p>
                        </AccordionContent>
                    </Accordion>

                    <Accordion
                        name="demo2"
                        modifier=AccordionModifier::Arrow
                        class="border border-base-300"
                    >
                        <AccordionTitle>"Arrow indicator"</AccordionTitle>
                        <AccordionContent>
                            <p>"Shows rotating arrow"</p>
                        </AccordionContent>
                    </Accordion>

                    <Accordion
                        name="demo2"
                        modifier=AccordionModifier::Plus
                        class="border border-base-300"
                    >
                        <AccordionTitle>"Plus indicator"</AccordionTitle>
                        <AccordionContent>
                            <p>"Shows plus/minus toggle"</p>
                        </AccordionContent>
                    </Accordion>
                </div>
            </Section>
        </ContentLayout>
    }
}
