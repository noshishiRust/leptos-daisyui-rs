use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn AccordionDemo() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <h1 class="text-3xl font-bold">"Accordion"</h1>
            <p class="text-base-content/70">
                "Accordion is used for showing and hiding content but only one item can stay open at a time"
            </p>

            <div class="space-y-4">
                <h2 class="text-xl font-semibold">"Examples"</h2>

                <div class="space-y-2">
                    <Accordion
                        modifier=Signal::derive(move || AccordionModifier::Arrow)
                        class="border border-base-300"
                    >
                        <input type="radio" name="accordion-1" checked />
                        <AccordionTitle>"What is daisyUI?"</AccordionTitle>
                        <AccordionContent>
                            <p>
                                "daisyUI is a Tailwind CSS component library that provides semantic class names for common UI components."
                            </p>
                        </AccordionContent>
                    </Accordion>

                    <Accordion
                        modifier=Signal::derive(move || AccordionModifier::Arrow)
                        class="border border-base-300"
                    >
                        <input type="radio" name="accordion-1" />
                        <AccordionTitle>"How do I use it with Leptos?"</AccordionTitle>
                        <AccordionContent>
                            <p>
                                "This library provides Leptos components that wrap daisyUI classes with type-safe, reactive props."
                            </p>
                        </AccordionContent>
                    </Accordion>

                    <Accordion
                        modifier=Signal::derive(move || AccordionModifier::Plus)
                        class="border border-base-300"
                    >
                        <input type="radio" name="accordion-1" />
                        <AccordionTitle>"Plus style accordion"</AccordionTitle>
                        <AccordionContent>
                            <p>
                                "This accordion uses the plus modifier for a different icon style."
                            </p>
                        </AccordionContent>
                    </Accordion>
                </div>
            </div>
        </div>
    }
}
