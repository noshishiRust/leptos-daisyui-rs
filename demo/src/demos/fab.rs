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
                <div class="relative flex justify-center min-h-80 items-center">
                    <Fab>
                        <div tabindex="0" role="button">
                            <Button color=ButtonColor::Primary shape=ButtonShape::Circle>
                                "+"
                            </Button>
                        </div>
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
                <div class="relative flex justify-center min-h-80 items-center">
                    <Fab>
                        <div tabindex="0" role="button">
                            <Button color=ButtonColor::Success shape=ButtonShape::Circle>
                                "☰"
                            </Button>
                        </div>
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

            <Section title="Flower Layout (Quarter Circle) - Responsive Directions">
                <div class="space-y-8">
                    <div class="alert alert-info">
                        <div>
                            <strong>"Responsive Design Tip:"</strong>
                            " Use the direction prop to control which quadrant the buttons fan out into. "
                            "This prevents buttons from rendering off-screen when the FAB is near page edges."
                        </div>
                    </div>

                    <div class="grid grid-cols-2 gap-8">
                        // Top-left corner: buttons fan to bottom-right
                        <div class="relative border border-base-300 rounded-lg p-4 min-h-64">
                            <div class="text-sm font-semibold mb-2">
                                "Top-Left: BottomRight Direction"
                            </div>
                            <Fab
                                flower=Signal::derive(|| true)
                                direction=Signal::derive(|| FabDirection::BottomRight)
                            >
                                <div tabindex="0" role="button">
                                    <Button color=ButtonColor::Primary shape=ButtonShape::Circle>
                                        "+"
                                    </Button>
                                </div>
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

                        // Top-right corner: buttons fan to bottom-left
                        <div class="relative border border-base-300 rounded-lg p-4 min-h-64 flex justify-end">
                            <div class="absolute top-4 left-4 text-sm font-semibold">
                                "Top-Right: BottomLeft Direction"
                            </div>
                            <Fab
                                flower=Signal::derive(|| true)
                                direction=Signal::derive(|| FabDirection::BottomLeft)
                            >
                                <div tabindex="0" role="button">
                                    <Button color=ButtonColor::Primary shape=ButtonShape::Circle>
                                        "+"
                                    </Button>
                                </div>
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
                                <Button color=ButtonColor::Success shape=ButtonShape::Circle>
                                    "4"
                                </Button>
                            </Fab>
                        </div>

                        // Bottom-left corner: buttons fan to top-right
                        <div class="relative border border-base-300 rounded-lg p-4 min-h-64 flex items-end">
                            <div class="absolute top-4 left-4 text-sm font-semibold">
                                "Bottom-Left: TopRight Direction"
                            </div>
                            <Fab
                                flower=Signal::derive(|| true)
                                direction=Signal::derive(|| FabDirection::TopRight)
                            >
                                <div tabindex="0" role="button">
                                    <Button color=ButtonColor::Success shape=ButtonShape::Circle>
                                        "+"
                                    </Button>
                                </div>
                                <FabClose>
                                    <Button color=ButtonColor::Error shape=ButtonShape::Circle>
                                        "×"
                                    </Button>
                                </FabClose>
                                <Button color=ButtonColor::Secondary shape=ButtonShape::Circle>
                                    "📧"
                                </Button>
                                <Button color=ButtonColor::Accent shape=ButtonShape::Circle>
                                    "🔔"
                                </Button>
                                <Button color=ButtonColor::Info shape=ButtonShape::Circle>
                                    "⚙"
                                </Button>
                                <Button color=ButtonColor::Warning shape=ButtonShape::Circle>
                                    "❤"
                                </Button>
                            </Fab>
                        </div>

                        // Bottom-right corner: buttons fan to top-left
                        <div class="relative border border-base-300 rounded-lg p-4 min-h-64 flex items-end justify-end">
                            <div class="absolute top-4 left-4 text-sm font-semibold">
                                "Bottom-Right: TopLeft Direction"
                            </div>
                            <Fab
                                flower=Signal::derive(|| true)
                                direction=Signal::derive(|| FabDirection::TopLeft)
                            >
                                <div tabindex="0" role="button">
                                    <Button color=ButtonColor::Accent shape=ButtonShape::Circle>
                                        "+"
                                    </Button>
                                </div>
                                <FabClose>
                                    <Button color=ButtonColor::Error shape=ButtonShape::Circle>
                                        "×"
                                    </Button>
                                </FabClose>
                                <Button color=ButtonColor::Primary shape=ButtonShape::Circle>
                                    "★"
                                </Button>
                                <Button color=ButtonColor::Secondary shape=ButtonShape::Circle>
                                    "♥"
                                </Button>
                                <Button color=ButtonColor::Info shape=ButtonShape::Circle>
                                    "♦"
                                </Button>
                                <Button color=ButtonColor::Success shape=ButtonShape::Circle>
                                    "♣"
                                </Button>
                            </Fab>
                        </div>
                    </div>
                </div>
            </Section>

            <Section title="Different Sizes">
                <div class="relative flex gap-8 justify-center items-start min-h-64">
                    <Fab>
                        <div tabindex="0" role="button">
                            <Button
                                color=ButtonColor::Primary
                                shape=ButtonShape::Circle
                                size=ButtonSize::Sm
                            >
                                "+"
                            </Button>
                        </div>
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
                        <div tabindex="0" role="button">
                            <Button
                                color=ButtonColor::Primary
                                shape=ButtonShape::Circle
                                size=ButtonSize::Lg
                            >
                                "+"
                            </Button>
                        </div>
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
                        "The trigger button should be wrapped in a div with tabindex=\"0\" and role=\"button\" for proper accessibility. "
                        "Flower layout can accommodate 1-4 action buttons arranged in a quarter-circle pattern."
                    </div>
                </div>
            </Section>
        </ContentLayout>
    }
}
