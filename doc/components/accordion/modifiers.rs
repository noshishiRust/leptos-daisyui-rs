view! {
    <Accordion name="demo2" modifier=AccordionModifier::Default class="border border-base-300">
        <AccordionTitle>"Default (no icon)"</AccordionTitle>
        <AccordionContent>
            <p>"No visual indicator"</p>
        </AccordionContent>
    </Accordion>

    <Accordion name="demo2" modifier=AccordionModifier::Arrow class="border border-base-300">
        <AccordionTitle>"Arrow indicator"</AccordionTitle>
        <AccordionContent>
            <p>"Shows rotating arrow"</p>
        </AccordionContent>
    </Accordion>

    <Accordion name="demo2" modifier=AccordionModifier::Plus class="border border-base-300">
        <AccordionTitle>"Plus indicator"</AccordionTitle>
        <AccordionContent>
            <p>"Shows plus/minus toggle"</p>
        </AccordionContent>
    </Accordion>
}
