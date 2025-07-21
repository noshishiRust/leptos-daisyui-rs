use crate::core::{ContentLayout, Section};
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;
use leptos_icons::Icon;

#[component]
pub fn TimelineDemo() -> impl IntoView {
    view! {
        <ContentLayout
            title="Timeline"
            description="Timeline component shows a list of events in chronological order"
        >
            <Section title="Basic Timeline">
                <Timeline>
                    <TimelineItem position=TimelineItemPosition::Start>
                        <TimelineItemMiddle>
                            <Icon icon=icondata::AiCheckCircleFilled />
                        </TimelineItemMiddle>
                        <TimelineItemEnd boxed=true>"1984"</TimelineItemEnd>
                    </TimelineItem>
                    <TimelineItem position=TimelineItemPosition::Between>
                        <TimelineItemStart boxed=true>"1998"</TimelineItemStart>
                        <TimelineItemMiddle>
                            <Icon icon=icondata::AiCheckCircleFilled />
                        </TimelineItemMiddle>
                    </TimelineItem>
                    <TimelineItem position=TimelineItemPosition::Between>
                        <TimelineItemMiddle>
                            <Icon icon=icondata::AiCheckCircleFilled />
                        </TimelineItemMiddle>
                        <TimelineItemEnd boxed=true>"2001"</TimelineItemEnd>
                    </TimelineItem>
                    <TimelineItem position=TimelineItemPosition::End>
                        <TimelineItemStart boxed=true>"2007"</TimelineItemStart>
                        <TimelineItemMiddle>
                            <Icon icon=icondata::AiCheckCircleFilled />
                        </TimelineItemMiddle>
                    </TimelineItem>
                </Timeline>
            </Section>

            <Section title="Event Timeline with Status">
                <Timeline>
                    <TimelineItem position=TimelineItemPosition::Start end_class="bg-success">
                        <TimelineItemStart boxed=true class="bg-success text-success-content">
                            <div class="flex items-center gap-2">
                                <Badge color=BadgeColor::Success size=BadgeSize::Sm>
                                    "✓ Complete"
                                </Badge>
                                <span class="font-semibold">"Design Phase"</span>
                            </div>
                            <p class="text-sm mt-1">"UI/UX mockups and prototypes finished"</p>
                        </TimelineItemStart>
                        <TimelineItemMiddle>
                            <Icon icon=icondata::AiCheckCircleFilled attr:class="fill-success" />
                        </TimelineItemMiddle>
                        <TimelineItemEnd>
                            <time class="font-mono italic">"Week 1-2"</time>
                        </TimelineItemEnd>
                    </TimelineItem>
                    <TimelineItem
                        position=TimelineItemPosition::Between
                        start_class="bg-success"
                        end_class="bg-warning"
                    >

                        <TimelineItemStart>
                            <time class="font-mono italic">"Week 3-6"</time>
                        </TimelineItemStart>
                        <TimelineItemMiddle>
                            <Icon icon=icondata::AiCheckCircleFilled attr:class="fill-warning" />
                        </TimelineItemMiddle>
                        <TimelineItemEnd boxed=true class="bg-warning text-warning-content">
                            <div class="flex items-center gap-2">
                                <Badge color=BadgeColor::Warning size=BadgeSize::Sm>
                                    "⚡ In Progress"
                                </Badge>
                                <span class="font-semibold">"Development"</span>
                            </div>
                            <p class="text-sm mt-1">"Building core features - 65% complete"</p>
                            <Progress
                                attr:value=65.0
                                attr:max=100.0
                                color=ProgressColor::Warning
                                class="w-full mt-2"
                            />
                        </TimelineItemEnd>

                    </TimelineItem>
                    <TimelineItem
                        position=TimelineItemPosition::Between
                        start_class="bg-warning"
                        end_class="bg-base-300"
                    >
                        <TimelineItemStart
                            boxed=true
                            class="bg-base-300 text-base-content opacity-60"
                        >
                            <div class="flex items-center gap-2">
                                <Badge color=BadgeColor::Neutral size=BadgeSize::Sm>
                                    "⏳ Pending"
                                </Badge>
                                <span class="font-semibold">"Testing"</span>
                            </div>
                            <p class="text-sm mt-1">"Quality assurance and bug fixes"</p>
                        </TimelineItemStart>
                        <TimelineItemMiddle>
                            <Icon icon=icondata::AiCheckCircleFilled attr:class="fill-base-300" />
                        </TimelineItemMiddle>
                        <TimelineItemEnd>
                            <time class="font-mono italic opacity-60">"Week 7-8"</time>
                        </TimelineItemEnd>
                    </TimelineItem>
                    <TimelineItem position=TimelineItemPosition::End start_class="bg-base-300">
                        <TimelineItemStart>
                            <time class="font-mono italic opacity-60">"Week 9"</time>
                        </TimelineItemStart>
                        <TimelineItemMiddle>
                            <Icon icon=icondata::AiCheckCircleFilled attr:class="fill-base-300" />
                        </TimelineItemMiddle>
                        <TimelineItemEnd
                            boxed=true
                            class="bg-base-300 text-base-content opacity-60"
                        >
                            <div class="flex items-center gap-2">
                                <Badge color=BadgeColor::Neutral size=BadgeSize::Sm>
                                    "🚀 Planned"
                                </Badge>
                                <span class="font-semibold">"Deployment"</span>
                            </div>
                            <p class="text-sm mt-1">"Production release and monitoring"</p>
                        </TimelineItemEnd>
                    </TimelineItem>
                </Timeline>
            </Section>
        </ContentLayout>
    }
}
