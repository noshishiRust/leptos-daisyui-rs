use crate::core::{ContentLayout, Section};
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn DividerDemo() -> impl IntoView {
    view! {
        <ContentLayout
            title="Divider"
            description="Visual separators for dividing content sections"
        >
            <Section title="Basic Horizontal Divider">
                <div class="flex flex-col w-full">
                    <div class="grid h-20 card bg-base-300 rounded-box place-items-center">
                        "Content Above"
                    </div>
                    <Divider />
                    <div class="grid h-20 card bg-base-300 rounded-box place-items-center">
                        "Content Below"
                    </div>
                </div>
            </Section>

            <Section title="Divider with Text">
                <div class="flex flex-col w-full">
                    <div class="grid h-20 card bg-base-300 rounded-box place-items-center">
                        "Section 1"
                    </div>
                    <Divider>
                        "OR"
                    </Divider>
                    <div class="grid h-20 card bg-base-300 rounded-box place-items-center">
                        "Section 2"
                    </div>
                </div>
            </Section>

            <Section title="Divider Colors">
                <div class="flex flex-col w-full gap-4">
                    <div class="grid h-16 card bg-base-300 rounded-box place-items-center">
                        "Content"
                    </div>
                    <Divider color=DividerColor::Neutral>
                        "Neutral"
                    </Divider>
                    <div class="grid h-16 card bg-base-300 rounded-box place-items-center">
                        "Content"
                    </div>
                    <Divider color=DividerColor::Primary>
                        "Primary"
                    </Divider>
                    <div class="grid h-16 card bg-base-300 rounded-box place-items-center">
                        "Content"
                    </div>
                    <Divider color=DividerColor::Secondary>
                        "Secondary"
                    </Divider>
                    <div class="grid h-16 card bg-base-300 rounded-box place-items-center">
                        "Content"
                    </div>
                    <Divider color=DividerColor::Accent>
                        "Accent"
                    </Divider>
                    <div class="grid h-16 card bg-base-300 rounded-box place-items-center">
                        "Content"
                    </div>
                    <Divider color=DividerColor::Success>
                        "Success"
                    </Divider>
                    <div class="grid h-16 card bg-base-300 rounded-box place-items-center">
                        "Content"
                    </div>
                    <Divider color=DividerColor::Warning>
                        "Warning"
                    </Divider>
                    <div class="grid h-16 card bg-base-300 rounded-box place-items-center">
                        "Content"
                    </div>
                    <Divider color=DividerColor::Info>
                        "Info"
                    </Divider>
                    <div class="grid h-16 card bg-base-300 rounded-box place-items-center">
                        "Content"
                    </div>
                    <Divider color=DividerColor::Error>
                        "Error"
                    </Divider>
                    <div class="grid h-16 card bg-base-300 rounded-box place-items-center">
                        "Content"
                    </div>
                </div>
            </Section>

            <Section title="Vertical Divider">
                <div class="flex w-full">
                    <div class="grid h-40 flex-grow card bg-base-300 rounded-box place-items-center">
                        "Left Content"
                    </div>
                    <Divider direction=DividerDirection::Vertical>
                        "OR"
                    </Divider>
                    <div class="grid h-40 flex-grow card bg-base-300 rounded-box place-items-center">
                        "Right Content"
                    </div>
                </div>
            </Section>

            <Section title="Divider Text Placement">
                <div class="flex flex-col w-full gap-4">
                    <div class="grid h-20 card bg-base-300 rounded-box place-items-center">
                        "Content"
                    </div>
                    <Divider placement=DividerPlacement::Start>
                        "Start Aligned"
                    </Divider>
                    <div class="grid h-20 card bg-base-300 rounded-box place-items-center">
                        "Content"
                    </div>
                    <Divider placement=DividerPlacement::Default>
                        "Center (Default)"
                    </Divider>
                    <div class="grid h-20 card bg-base-300 rounded-box place-items-center">
                        "Content"
                    </div>
                    <Divider placement=DividerPlacement::End>
                        "End Aligned"
                    </Divider>
                    <div class="grid h-20 card bg-base-300 rounded-box place-items-center">
                        "Content"
                    </div>
                </div>
            </Section>

            <Section title="Divider in Card Content">
                <div class="card bg-base-200 shadow-xl">
                    <div class="card-body">
                        <h2 class="card-title">"Article Title"</h2>
                        <p>"This is the introduction paragraph of the article. It provides context about the topic."</p>
                        <Divider />
                        <h3 class="text-lg font-semibold">"Section 1"</h3>
                        <p>"Content for the first section of the article goes here."</p>
                        <Divider color=DividerColor::Primary>
                            "Important Section"
                        </Divider>
                        <h3 class="text-lg font-semibold">"Section 2"</h3>
                        <p>"Content for the second section with important information."</p>
                        <Divider />
                        <h3 class="text-lg font-semibold">"Conclusion"</h3>
                        <p>"Final thoughts and summary of the article."</p>
                    </div>
                </div>
            </Section>

            <Section title="Responsive Layout with Dividers">
                <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                    <div class="card bg-base-300 p-4">
                        <h3 class="font-bold text-lg">"Feature 1"</h3>
                        <Divider color=DividerColor::Accent placement=DividerPlacement::Start />
                        <p>"Description of the first feature with divider separator."</p>
                    </div>
                    <div class="card bg-base-300 p-4">
                        <h3 class="font-bold text-lg">"Feature 2"</h3>
                        <Divider color=DividerColor::Primary placement=DividerPlacement::Start />
                        <p>"Description of the second feature with divider separator."</p>
                    </div>
                    <div class="card bg-base-300 p-4">
                        <h3 class="font-bold text-lg">"Feature 3"</h3>
                        <Divider color=DividerColor::Secondary placement=DividerPlacement::Start />
                        <p>"Description of the third feature with divider separator."</p>
                    </div>
                </div>
            </Section>

            <Section title="Multiple Vertical Dividers">
                <div class="flex w-full h-40">
                    <div class="grid flex-grow card bg-base-300 rounded-box place-items-center">
                        "Column 1"
                    </div>
                    <Divider direction=DividerDirection::Vertical />
                    <div class="grid flex-grow card bg-base-300 rounded-box place-items-center">
                        "Column 2"
                    </div>
                    <Divider direction=DividerDirection::Vertical />
                    <div class="grid flex-grow card bg-base-300 rounded-box place-items-center">
                        "Column 3"
                    </div>
                    <Divider direction=DividerDirection::Vertical />
                    <div class="grid flex-grow card bg-base-300 rounded-box place-items-center">
                        "Column 4"
                    </div>
                </div>
            </Section>

            <Section title="Themed Dividers">
                <div class="flex flex-col w-full gap-4">
                    <div class="card bg-success/10 border border-success p-4">
                        <p class="text-success-content">"This section succeeded!"</p>
                    </div>
                    <Divider color=DividerColor::Success>
                        "Success"
                    </Divider>
                    <div class="card bg-warning/10 border border-warning p-4">
                        <p class="text-warning-content">"Warning: Please review this section."</p>
                    </div>
                    <Divider color=DividerColor::Warning>
                        "Warning"
                    </Divider>
                    <div class="card bg-error/10 border border-error p-4">
                        <p class="text-error-content">"Error: Something went wrong here."</p>
                    </div>
                </div>
            </Section>
        </ContentLayout>
    }
}
