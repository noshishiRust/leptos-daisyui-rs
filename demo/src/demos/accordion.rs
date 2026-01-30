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
            <Section title="Radio Accordion (Only one open at a time)">
                <p class="text-sm text-base-content/70 mb-4">
                    "Radio accordions share a name and only allow one item open at a time. "
                    "Note: Cannot be closed by clicking again - only by opening another item."
                </p>
                <div class="max-w-md">
                    <Accordion
                        input_type=AccordionInputType::Radio
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
                        input_type=AccordionInputType::Radio
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

            <Section title="Checkbox Collapse (Toggle independently)">
                <p class="text-sm text-base-content/70 mb-4">
                    "Checkbox accordions can be toggled independently. Click again to close. Multiple can be open at once."
                </p>
                <div class="max-w-md space-y-2">
                    <Accordion
                        input_type=AccordionInputType::Checkbox
                        modifier=AccordionModifier::Plus
                        class="border border-base-300"
                    >
                        <AccordionTitle>"Click to toggle (Plus indicator)"</AccordionTitle>
                        <AccordionContent>
                            <p>"This accordion can be opened and closed by clicking the title."</p>
                        </AccordionContent>
                    </Accordion>

                    <Accordion
                        input_type=AccordionInputType::Checkbox
                        modifier=AccordionModifier::Arrow
                        class="border border-base-300"
                    >
                        <AccordionTitle>"Independent toggle (Arrow indicator)"</AccordionTitle>
                        <AccordionContent>
                            <p>"Multiple checkbox accordions can be open at the same time."</p>
                        </AccordionContent>
                    </Accordion>

                    <Accordion
                        input_type=AccordionInputType::Checkbox
                        modifier=AccordionModifier::Default
                        class="border border-base-300"
                    >
                        <AccordionTitle>"No indicator"</AccordionTitle>
                        <AccordionContent>
                            <p>"Still works without visual indicators."</p>
                        </AccordionContent>
                    </Accordion>
                </div>
            </Section>

            <Section title="Icon Modifiers">
                <div class="max-w-md space-y-2">
                    <Accordion
                        input_type=AccordionInputType::Checkbox
                        modifier=AccordionModifier::Default
                        class="border border-base-300"
                    >
                        <AccordionTitle>"Default (no icon)"</AccordionTitle>
                        <AccordionContent>
                            <p>"No visual indicator"</p>
                        </AccordionContent>
                    </Accordion>

                    <Accordion
                        input_type=AccordionInputType::Checkbox
                        modifier=AccordionModifier::Arrow
                        class="border border-base-300"
                    >
                        <AccordionTitle>"Arrow indicator"</AccordionTitle>
                        <AccordionContent>
                            <p>"Shows rotating arrow"</p>
                        </AccordionContent>
                    </Accordion>

                    <Accordion
                        input_type=AccordionInputType::Checkbox
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
