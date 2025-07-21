use crate::core::{ContentLayout, Section};
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn CollapseDemo() -> impl IntoView {
    view! {
        <ContentLayout
            title="Collapse"
            description="Collapse is used for showing and hiding content"
        >
            <Section title="Basic Variations" col=true>
                <Collapse focus_open=true class="border border-base-300">
                    <CollapseTitle class="text-xl font-medium">
                        "Focus me to see content"
                    </CollapseTitle>
                    <CollapseContent>
                        <p>"tabindex=\"0\" attribute is necessary to make the div focusable"</p>
                    </CollapseContent>
                </Collapse>

                <Collapse focus_open=true class="border border-base-300">
                    <CollapseTitle class="text-xl font-medium">
                        "Click me to open content"
                    </CollapseTitle>
                    <CollapseContent>
                        <p>"The plus icon changes to minus when expanded"</p>
                    </CollapseContent>
                </Collapse>
            </Section>

            <Section title="Force States" col=true>
                <Collapse
                    force=CollapseForceModifier::Open
                    class="border border-base-300 bg-base-100 rounded-box"
                >
                    <CollapseTitle class="text-xl font-medium">"Always Open"</CollapseTitle>
                    <CollapseContent>
                        <p>"This collapse is always open"</p>
                    </CollapseContent>
                </Collapse>

                <Collapse
                    force=CollapseForceModifier::Close
                    class="border border-base-300 bg-base-100 rounded-box"
                >
                    <CollapseTitle class="text-xl font-medium">"Always Closed"</CollapseTitle>
                    <CollapseContent>
                        <p>"This collapse is always closed"</p>
                    </CollapseContent>
                </Collapse>
            </Section>
        </ContentLayout>
    }
}
