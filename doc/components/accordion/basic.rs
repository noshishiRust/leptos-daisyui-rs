view! {
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

    <Accordion name="demo1" modifier=AccordionModifier::Arrow class="border border-base-300">
        <AccordionTitle>"How do I use it with Leptos?"</AccordionTitle>
        <AccordionContent>
            <p>
                "This library provides Leptos components that wrap daisyUI classes with type-safe, reactive props."
            </p>
        </AccordionContent>
    </Accordion>
}
