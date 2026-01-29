use crate::core::{ContentLayout, Section};
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn TooltipDemo() -> impl IntoView {
    let (open, set_open) = signal(false);
    let (position, set_position) = signal(TooltipPosition::Top);

    view! {
        <ContentLayout
            title="Tooltip"
            description="Tooltips display contextual information on hover"
        >
            <Section title="Positions" row=true>
                <Tooltip tip=Signal::derive(|| "Top tooltip".to_string()) position=TooltipPosition::Top>
                    <Button color=ButtonColor::Primary>"Top"</Button>
                </Tooltip>
                <Tooltip tip=Signal::derive(|| "Bottom tooltip".to_string()) position=TooltipPosition::Bottom>
                    <Button color=ButtonColor::Primary>"Bottom"</Button>
                </Tooltip>
                <Tooltip tip=Signal::derive(|| "Left tooltip".to_string()) position=TooltipPosition::Left>
                    <Button color=ButtonColor::Primary>"Left"</Button>
                </Tooltip>
                <Tooltip tip=Signal::derive(|| "Right tooltip".to_string()) position=TooltipPosition::Right>
                    <Button color=ButtonColor::Primary>"Right"</Button>
                </Tooltip>
            </Section>

            <Section title="Colors" row=true>
                <Tooltip tip=Signal::derive(|| "Neutral tooltip".to_string()) color=TooltipColor::Neutral>
                    <Button>"Neutral"</Button>
                </Tooltip>
                <Tooltip tip=Signal::derive(|| "Primary tooltip".to_string()) color=TooltipColor::Primary>
                    <Button>"Primary"</Button>
                </Tooltip>
                <Tooltip tip=Signal::derive(|| "Secondary tooltip".to_string()) color=TooltipColor::Secondary>
                    <Button>"Secondary"</Button>
                </Tooltip>
                <Tooltip tip=Signal::derive(|| "Accent tooltip".to_string()) color=TooltipColor::Accent>
                    <Button>"Accent"</Button>
                </Tooltip>
                <Tooltip tip=Signal::derive(|| "Info tooltip".to_string()) color=TooltipColor::Info>
                    <Button>"Info"</Button>
                </Tooltip>
                <Tooltip tip=Signal::derive(|| "Success tooltip".to_string()) color=TooltipColor::Success>
                    <Button>"Success"</Button>
                </Tooltip>
                <Tooltip tip=Signal::derive(|| "Warning tooltip".to_string()) color=TooltipColor::Warning>
                    <Button>"Warning"</Button>
                </Tooltip>
                <Tooltip tip=Signal::derive(|| "Error tooltip".to_string()) color=TooltipColor::Error>
                    <Button>"Error"</Button>
                </Tooltip>
            </Section>

            <Section title="Always Open">
                <div class="flex gap-4">
                    <Tooltip
                        tip=Signal::derive(|| "This tooltip is always visible".to_string())
                        color=TooltipColor::Info
                        open=Signal::derive(|| true)
                    >
                        <Button color=ButtonColor::Info>"Open Tooltip"</Button>
                    </Tooltip>
                </div>
            </Section>

            <Section title="Reactive Examples">
                <div class="flex flex-col gap-4">
                    <div class="flex items-center gap-4">
                        <Tooltip tip=Signal::derive(|| "Click to toggle me!".to_string()) open=open>
                            <Button
                                color=ButtonColor::Primary
                                on:click=move |_| set_open.update(|o| *o = !*o)
                            >
                                "Toggle Tooltip"
                            </Button>
                        </Tooltip>
                        <span class="text-sm">
                            {move || if open.get() { "Tooltip: On" } else { "Tooltip: Off" }}
                        </span>
                    </div>

                    <div class="flex items-center gap-2">
                        <Tooltip
                            tip=Signal::derive(|| "Dynamic position!".to_string())
                            position=position
                            color=TooltipColor::Accent
                        >
                            <Button color=ButtonColor::Accent>"Dynamic Position"</Button>
                        </Tooltip>
                        <Button
                            size=ButtonSize::Sm
                            on:click=move |_| {
                                set_position
                                    .update(|pos| {
                                        *pos = match pos {
                                            TooltipPosition::Top => TooltipPosition::Right,
                                            TooltipPosition::Right => TooltipPosition::Bottom,
                                            TooltipPosition::Bottom => TooltipPosition::Left,
                                            TooltipPosition::Left => TooltipPosition::Top,
                                        }
                                    });
                            }
                        >
                            "Cycle Position: " {move || format!("{:?}", position.get())}
                        </Button>
                    </div>
                </div>
            </Section>

            <Section title="With Different Elements">
                <div class="flex flex-wrap gap-4">
                    <Tooltip tip=Signal::derive(|| "Tooltip on text".to_string())>
                        <span class="underline cursor-help">"Hover over me"</span>
                    </Tooltip>
                    <Tooltip tip=Signal::derive(|| "Tooltip on badge".to_string()) color=TooltipColor::Success>
                        <Badge color=BadgeColor::Success>"New"</Badge>
                    </Tooltip>
                    <Tooltip tip=Signal::derive(|| "Tooltip on avatar".to_string()) position=TooltipPosition::Bottom>
                        <div class="avatar">
                            <div class="w-12 rounded-full bg-neutral text-neutral-content flex items-center justify-center">
                                <span class="text-xl">"U"</span>
                            </div>
                        </div>
                    </Tooltip>
                </div>
            </Section>
        </ContentLayout>
    }
}
