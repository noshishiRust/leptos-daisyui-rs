use crate::core::{ContentLayout, Section};
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn FabDemo() -> impl IntoView {
    view! {
        <ContentLayout
            title="FAB (Floating Action Button)"
            description="Floating action buttons with speed dial functionality"
        >
            <Section title="Basic FAB - Vertical Layout">
                <div class="flex justify-center">
                    <Fab>
                        <Button color=ButtonColor::Primary shape=ButtonShape::Circle>
                            "+"
                        </Button>
                        <FabClose>
                            <Button color=ButtonColor::Error shape=ButtonShape::Circle>
                                "×"
                            </Button>
                        </FabClose>
                        <Button color=ButtonColor::Secondary shape=ButtonShape::Circle>
                            "1"
                        </Button>
                        <Button color=ButtonColor::Accent shape=ButtonShape::Circle>
                            "2"
                        </Button>
                        <Button color=ButtonColor::Info shape=ButtonShape::Circle>
                            "3"
                        </Button>
                    </Fab>
                </div>
            </Section>

            <Section title="FAB with Main Action">
                <div class="flex justify-center">
                    <Fab>
                        <Button color=ButtonColor::Success shape=ButtonShape::Circle>
                            "☰"
                        </Button>
                        <FabMainAction>
                            <Button color=ButtonColor::Primary shape=ButtonShape::Circle>
                                "✓"
                            </Button>
                        </FabMainAction>
                        <Button color=ButtonColor::Secondary shape=ButtonShape::Circle>
                            "📧"
                        </Button>
                        <Button color=ButtonColor::Accent shape=ButtonShape::Circle>
                            "🔔"
                        </Button>
                        <Button color=ButtonColor::Warning shape=ButtonShape::Circle>
                            "⚙"
                        </Button>
                    </Fab>
                </div>
            </Section>

            <Section title="Flower Layout (Quarter Circle)">
                <div class="flex justify-center">
                    <Fab flower=Signal::derive(|| true)>
                        <Button color=ButtonColor::Primary shape=ButtonShape::Circle>
                            "+"
                        </Button>
                        <FabClose>
                            <Button color=ButtonColor::Error shape=ButtonShape::Circle>
                                "×"
                            </Button>
                        </FabClose>
                        <Button color=ButtonColor::Secondary shape=ButtonShape::Circle>
                            "A"
                        </Button>
                        <Button color=ButtonColor::Accent shape=ButtonShape::Circle>
                            "B"
                        </Button>
                        <Button color=ButtonColor::Info shape=ButtonShape::Circle>
                            "C"
                        </Button>
                        <Button color=ButtonColor::Success shape=ButtonShape::Circle>
                            "D"
                        </Button>
                    </Fab>
                </div>
            </Section>

            <Section title="Different Sizes">
                <div class="flex gap-8 justify-center items-start">
                    <Fab>
                        <Button
                            color=ButtonColor::Primary
                            shape=ButtonShape::Circle
                            size=ButtonSize::Sm
                        >
                            "+"
                        </Button>
                        <FabClose>
                            <Button
                                color=ButtonColor::Error
                                shape=ButtonShape::Circle
                                size=ButtonSize::Sm
                            >
                                "×"
                            </Button>
                        </FabClose>
                        <Button
                            color=ButtonColor::Secondary
                            shape=ButtonShape::Circle
                            size=ButtonSize::Sm
                        >
                            "1"
                        </Button>
                    </Fab>

                    <Fab>
                        <Button
                            color=ButtonColor::Primary
                            shape=ButtonShape::Circle
                            size=ButtonSize::Lg
                        >
                            "+"
                        </Button>
                        <FabClose>
                            <Button
                                color=ButtonColor::Error
                                shape=ButtonShape::Circle
                                size=ButtonSize::Lg
                            >
                                "×"
                            </Button>
                        </FabClose>
                        <Button
                            color=ButtonColor::Secondary
                            shape=ButtonShape::Circle
                            size=ButtonSize::Lg
                        >
                            "1"
                        </Button>
                    </Fab>
                </div>
            </Section>

            <Section title="Usage Note">
                <div class="alert alert-info">
                    <div>
                        <strong>"Tip:"</strong>
                        " Click or focus on the FAB trigger button to expand the speed dial menu. "
                        "The component uses focusable divs for Safari compatibility."
                    </div>
                </div>
            </Section>
        </ContentLayout>
    }
}
