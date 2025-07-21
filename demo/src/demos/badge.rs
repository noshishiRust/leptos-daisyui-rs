use crate::core::{ContentLayout, Section};
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn BadgeDemo() -> impl IntoView {
    let (count, set_count) = signal(3);
    let (online, set_online) = signal(true);

    view! {
        <ContentLayout
            title="Badge"
            description="Badges are used to inform the user of the status of specific data"
        >
            <Section row=true title="Colors">
                <Badge>"Default"</Badge>
                <Badge color=BadgeColor::Neutral>"Neutral"</Badge>
                <Badge color=BadgeColor::Primary>"Primary"</Badge>
                <Badge color=BadgeColor::Secondary>"Secondary"</Badge>
                <Badge color=BadgeColor::Accent>"Accent"</Badge>
                <Badge color=BadgeColor::Info>"Info"</Badge>
                <Badge color=BadgeColor::Success>"Success"</Badge>
                <Badge color=BadgeColor::Warning>"Warning"</Badge>
                <Badge color=BadgeColor::Error>"Error"</Badge>
            </Section>

            <Section row=true title="Sizes">
                <Badge size=BadgeSize::Xs>"XS"</Badge>
                <Badge size=BadgeSize::Sm>"SM"</Badge>
                <Badge size=BadgeSize::Md>"MD"</Badge>
                <Badge size=BadgeSize::Lg>"LG"</Badge>
                <Badge size=BadgeSize::Xl>"XL"</Badge>
            </Section>

            <Section row=true title="Styles">
                <Badge color=BadgeColor::Primary>"Default"</Badge>
                <Badge style=BadgeStyle::Outline color=BadgeColor::Primary>
                    "Outline"
                </Badge>
                <Badge style=BadgeStyle::Ghost color=BadgeColor::Primary>
                    "Ghost"
                </Badge>
                <Badge style=BadgeStyle::Soft color=BadgeColor::Primary>
                    "Soft"
                </Badge>
                <Badge style=BadgeStyle::Dash color=BadgeColor::Primary>
                    "Dash"
                </Badge>
            </Section>

            <Section title="Reactive Counter">
                <div class="flex items-center gap-4">
                    <div class="flex items-center gap-2">
                        "Messages " <Badge color=BadgeColor::Error>{move || count.get()}</Badge>
                    </div>
                    <div class="flex gap-2">
                        <Button size=ButtonSize::Sm on:click=move |_| set_count.update(|c| *c += 1)>
                            "Add Message"
                        </Button>
                        <Button
                            size=ButtonSize::Sm
                            color=ButtonColor::Neutral
                            on:click=move |_| set_count.set(0)
                        >
                            "Clear"
                        </Button>
                    </div>
                </div>
            </Section>

            <Section title="Status Indicator">
                <div class="flex items-center gap-4">
                    <div class="flex items-center gap-2">
                        "Server Status"
                        <Badge color=Signal::derive(move || {
                            if online.get() { BadgeColor::Success } else { BadgeColor::Error }
                        })>{move || if online.get() { "Online" } else { "Offline" }}</Badge>
                    </div>
                    <Button size=ButtonSize::Sm on:click=move |_| set_online.update(|s| *s = !*s)>
                        "Toggle Status"
                    </Button>
                </div>
            </Section>

            <Section title="Usage Examples">
                <div class="space-y-3">
                    <div class="text-lg">
                        "Inbox " <Badge color=BadgeColor::Secondary>"3"</Badge>
                    </div>
                    <div class="flex items-center gap-2">
                        "Notifications " <Badge color=BadgeColor::Error class="w-3 h-3 p-0">
                            " "
                        </Badge>
                    </div>
                    <div class="flex gap-2">
                        <Button class="relative">
                            "Profile"
                            <Badge
                                color=BadgeColor::Warning
                                size=BadgeSize::Sm
                                class="absolute -top-2 -right-2"
                            >
                                "NEW"
                            </Badge>
                        </Button>
                    </div>
                </div>
            </Section>
        </ContentLayout>
    }
}
