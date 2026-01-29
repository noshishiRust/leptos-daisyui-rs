use crate::core::{ContentLayout, Section};
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;
use leptos_icons::Icon;

#[component]
pub fn StackDemo() -> impl IntoView {
    view! {
        <ContentLayout
            title="Stack"
            description="Stack puts child elements on top of each other"
        >
            <Section title="Basic Stack">
                <div class="flex flex-wrap gap-8">
                    <Stack class="text-primary-content">
                        <div class="grid w-32 h-20 bg-primary place-content-center">
                            "1"
                        </div>
                        <div class="grid w-28 h-16 bg-accent place-content-center">
                            "2"
                        </div>
                        <div class="grid w-24 h-12 bg-secondary place-content-center">
                            "3"
                        </div>
                    </Stack>
                </div>
            </Section>

            <Section title="Profile Card with Stack">
                <div class="flex flex-wrap gap-8">
                    <Stack>
                        <div class="grid w-64 h-20 bg-primary text-primary-content place-content-center">
                            "Background"
                        </div>
                        <div class="grid w-60 h-16 bg-accent text-accent-content place-content-center">
                            "Middle layer"
                        </div>
                        <div class="grid w-56 h-12 bg-secondary text-secondary-content place-content-center">
                            "Top layer"
                        </div>
                    </Stack>
                </div>
            </Section>

            <Section title="Avatar with Badge Stack">
                <div class="flex flex-wrap gap-8">
                    <Stack>
                        <Avatar>
                            <div class="w-24 h-24 rounded-full">
                                <img src="https://picsum.photos/96/96?random=1" alt="User avatar" />
                            </div>
                        </Avatar>
                        <Badge
                            color=BadgeColor::Success
                            size=BadgeSize::Xs
                            class="indicator-item indicator-bottom indicator-end"
                        >
                            "Online"
                        </Badge>
                    </Stack>
                </div>
            </Section>

            <Section title="Image with Overlay Stack">
                <div class="flex flex-wrap gap-8">
                    <Stack>
                        <img
                            src="https://picsum.photos/300/200?random=5"
                            alt="Background"
                            class="rounded-lg w-72"
                        />
                        <div class="bg-black bg-opacity-50 text-white p-4 rounded-lg mx-6 my-4">
                            <h3 class="font-bold text-lg">"Image Title"</h3>
                            <p>"Description text overlay"</p>
                        </div>
                    </Stack>
                </div>
            </Section>

            <Section title="Button with Notification Stack">
                <div class="flex flex-wrap gap-8">
                    <Stack>
                        <Button color=ButtonColor::Primary size=ButtonSize::Lg>
                            <Icon icon=icondata::AiBellOutlined />
                            "Notifications"
                        </Button>
                        <Badge
                            color=BadgeColor::Error
                            size=BadgeSize::Sm
                            class="indicator-item indicator-top indicator-end"
                        >
                            "99+"
                        </Badge>
                    </Stack>

                    <Stack>
                        <Button style=ButtonStyle::Ghost size=ButtonSize::Lg>
                            <Icon icon=icondata::AiMessageOutlined />
                            "Messages"
                        </Button>
                        <Badge
                            color=BadgeColor::Warning
                            size=BadgeSize::Sm
                            class="indicator-item indicator-top indicator-end"
                        >
                            "3"
                        </Badge>
                    </Stack>
                </div>
            </Section>

            <Section title="Card Stack Layout">
                <div class="flex flex-wrap gap-8">
                    <Stack>
                        <Card class="w-80 bg-base-100 shadow-xl">
                            <CardBody>
                                <h2 class="card-title">"Base Card"</h2>
                                <p>"This is the base layer of the stack"</p>
                            </CardBody>
                        </Card>
                        <Card class="w-72 bg-primary text-primary-content shadow-xl mt-4 ml-4">
                            <CardBody>
                                <h2 class="card-title">"Stacked Card"</h2>
                                <p>"This card is stacked on top"</p>
                            </CardBody>
                        </Card>
                    </Stack>
                </div>
            </Section>

            <Section title="Creative Stack Examples">
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                    <div>
                        <h3 class="font-semibold mb-2">"Badge Collection"</h3>
                        <Stack>
                            <Badge color=BadgeColor::Primary class="badge-lg">
                                "Primary"
                            </Badge>
                            <Badge color=BadgeColor::Secondary class="badge-lg ml-2 mt-1">
                                "Secondary"
                            </Badge>
                            <Badge color=BadgeColor::Accent class="badge-lg ml-4 mt-2">
                                "Accent"
                            </Badge>
                        </Stack>
                    </div>

                    <div>
                        <h3 class="font-semibold mb-2">"Progress Stack"</h3>
                        <Stack>
                            <Progress
                                attr:value=100.0
                                attr:max=100.0
                                color=ProgressColor::Primary
                                class="w-56 h-6"
                            />
                            <Progress
                                attr:value=75.0
                                attr:max=100.0
                                color=ProgressColor::Secondary
                                class="w-52 h-5 mt-1"
                            />
                            <Progress
                                attr:value=50.0
                                attr:max=100.0
                                color=ProgressColor::Accent
                                class="w-48 h-4 mt-2"
                            />
                        </Stack>
                    </div>

                    <div>
                        <h3 class="font-semibold mb-2">"Loading Stack"</h3>
                        <Stack>
                            <div class="w-24 h-24 bg-base-300 rounded-full flex items-center justify-center">
                                "Content"
                            </div>
                            <Loading size=LoadingSize::Lg color=LoadingColor::Primary />
                        </Stack>
                    </div>
                </div>
            </Section>

            <Section title="Complex Stack Example">
                <div class="flex justify-center">
                    <Stack>
                        <Card class="w-96 bg-base-100 shadow-2xl">
                            <CardBody>
                                <h2 class="card-title">"User Profile"</h2>
                                <div class="flex items-center gap-4 my-4">
                                    <Stack>
                                        <Avatar>
                                            <div class="w-20 h-20 rounded-full">
                                                <img src="https://picsum.photos/80/80?random=10" alt="User" />
                                            </div>
                                        </Avatar>
                                        <Badge
                                            color=BadgeColor::Success
                                            size=BadgeSize::Sm
                                            class="indicator-item indicator-bottom indicator-end"
                                        >
                                            ""
                                        </Badge>
                                    </Stack>
                                    <div>
                                        <h3 class="font-bold">"John Doe"</h3>
                                        <p class="text-sm opacity-60">"Software Engineer"</p>
                                        <Stack class="mt-2">
                                            <Badge color=BadgeColor::Primary size=BadgeSize::Sm>
                                                "React"
                                            </Badge>
                                            <Badge
                                                color=BadgeColor::Secondary
                                                size=BadgeSize::Sm
                                                class="ml-1"
                                            >
                                                "TypeScript"
                                            </Badge>
                                            <Badge
                                                color=BadgeColor::Accent
                                                size=BadgeSize::Sm
                                                class="ml-2"
                                            >
                                                "Rust"
                                            </Badge>
                                        </Stack>
                                    </div>
                                </div>
                                <div class="card-actions justify-end">
                                    <Button color=ButtonColor::Primary size=ButtonSize::Sm>
                                        "Follow"
                                    </Button>
                                    <Button
                                        style=ButtonStyle::Ghost
                                        size=ButtonSize::Sm
                                    >
                                        "Message"
                                    </Button>
                                </div>
                            </CardBody>
                        </Card>
                        <div class="absolute top-0 right-0 p-2">
                            <Badge color=BadgeColor::Warning size=BadgeSize::Sm>
                                "VIP"
                            </Badge>
                        </div>
                    </Stack>
                </div>
            </Section>
        </ContentLayout>
    }
}
