use crate::core::{ContentLayout, Section};
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn TextRotateDemo() -> impl IntoView {
    view! {
        <ContentLayout
            title="Text Rotate"
            description="Animated text rotation with up to 6 items"
        >
            <Section title="Basic Example">
                <div class="text-4xl font-bold">
                    "Hello, I'm "
                    <TextRotate>
                        <TextRotateContainer>
                            <TextRotateItem>"a Developer"</TextRotateItem>
                            <TextRotateItem>"a Designer"</TextRotateItem>
                            <TextRotateItem>"a Creator"</TextRotateItem>
                        </TextRotateContainer>
                    </TextRotate>
                </div>
            </Section>

            <Section title="Multiple Words">
                <div class="text-3xl font-bold text-center">
                    <TextRotate>
                        <TextRotateContainer>
                            <TextRotateItem>"Welcome to our website"</TextRotateItem>
                            <TextRotateItem>"Explore amazing features"</TextRotateItem>
                            <TextRotateItem>"Build something great"</TextRotateItem>
                            <TextRotateItem>"Join our community"</TextRotateItem>
                        </TextRotateContainer>
                    </TextRotate>
                </div>
            </Section>

            <Section title="With Colors">
                <div class="text-4xl font-bold text-center space-y-4">
                    <div>
                        "We are "
                        <TextRotate class="text-primary">
                            <TextRotateContainer>
                                <TextRotateItem>"Fast"</TextRotateItem>
                                <TextRotateItem>"Reliable"</TextRotateItem>
                                <TextRotateItem>"Secure"</TextRotateItem>
                            </TextRotateContainer>
                        </TextRotate>
                    </div>
                    <div>
                        "Status: "
                        <TextRotate class="text-success">
                            <TextRotateContainer>
                                <TextRotateItem>"Online"</TextRotateItem>
                                <TextRotateItem>"Active"</TextRotateItem>
                                <TextRotateItem>"Available"</TextRotateItem>
                            </TextRotateContainer>
                        </TextRotate>
                    </div>
                </div>
            </Section>

            <Section title="Maximum Items (6)">
                <div class="text-3xl font-bold text-center">
                    "Season: "
                    <TextRotate class="text-accent">
                        <TextRotateContainer>
                            <TextRotateItem>"Spring"</TextRotateItem>
                            <TextRotateItem>"Summer"</TextRotateItem>
                            <TextRotateItem>"Fall"</TextRotateItem>
                            <TextRotateItem>"Winter"</TextRotateItem>
                            <TextRotateItem>"Autumn"</TextRotateItem>
                            <TextRotateItem>"Harvest"</TextRotateItem>
                        </TextRotateContainer>
                    </TextRotate>
                </div>
            </Section>

            <Section title="In Sentences">
                <div class="text-2xl text-center">
                    "Our product is "
                    <TextRotate class="text-primary font-bold">
                        <TextRotateContainer>
                            <TextRotateItem>"innovative"</TextRotateItem>
                            <TextRotateItem>"powerful"</TextRotateItem>
                            <TextRotateItem>"easy to use"</TextRotateItem>
                            <TextRotateItem>"affordable"</TextRotateItem>
                        </TextRotateContainer>
                    </TextRotate>
                    " and loved by developers worldwide."
                </div>
            </Section>

            <Section title="Usage Note">
                <div class="alert alert-info">
                    <div>
                        <strong>"Note:"</strong>
                        " The animation loops infinitely and pauses on hover. "
                        "Supports up to 6 items for optimal performance."
                    </div>
                </div>
            </Section>
        </ContentLayout>
    }
}
