use crate::core::{ContentLayout, Section};
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;
use leptos_icons::Icon;

#[component]
pub fn AvatarDemo() -> impl IntoView {
    let (online_status, set_online_status) = signal(AvatarModifier::Online);

    view! {
        <ContentLayout
            title="Avatar"
            description="Avatars are used to show a thumbnail representation of a user"
        >
            <Section title="Sizes">
                <div class="flex gap-4 items-center">
                    <Avatar>
                        <div class="w-8 rounded-full">
                            <img src="https://picsum.photos/100/100?random=1" alt="Small" />
                        </div>
                    </Avatar>
                    <Avatar>
                        <div class="w-12 rounded-full">
                            <img src="https://picsum.photos/100/100?random=2" alt="Medium" />
                        </div>
                    </Avatar>
                    <Avatar>
                        <div class="w-16 rounded-full">
                            <img src="https://picsum.photos/100/100?random=3" alt="Large" />
                        </div>
                    </Avatar>
                    <Avatar>
                        <div class="w-24 rounded-full">
                            <img src="https://picsum.photos/100/100?random=4" alt="XLarge" />
                        </div>
                    </Avatar>
                </div>
            </Section>

            <Section title="Status Indicators">
                <div class="flex gap-4 items-center">
                    <Avatar modifier=AvatarModifier::Online>
                        <div class="w-16 rounded-full">
                            <img src="https://picsum.photos/100/100?random=5" alt="Online" />
                        </div>
                    </Avatar>
                    <Avatar modifier=AvatarModifier::Offline>
                        <div class="w-16 rounded-full">
                            <img src="https://picsum.photos/100/100?random=6" alt="Offline" />
                        </div>
                    </Avatar>
                </div>
            </Section>

            <Section title="Placeholder Avatars">
                <div class="flex gap-4 items-center">
                    <Avatar modifier=AvatarModifier::Placeholder>
                        <div class="bg-neutral text-neutral-content rounded-full w-16">
                            <span class="text-xl">"JD"</span>
                        </div>
                    </Avatar>
                    <Avatar modifier=AvatarModifier::Placeholder>
                        <div class="bg-primary text-primary-content rounded-full w-16">
                            <Icon icon=icondata::AiUserOutlined />
                        </div>
                    </Avatar>
                </div>
            </Section>

            <Section title="Avatar Group">
                <AvatarGroup>
                    <Avatar>
                        <div class="w-12 rounded-full">
                            <img src="https://picsum.photos/100/100?random=7" alt="User 1" />
                        </div>
                    </Avatar>
                    <Avatar>
                        <div class="w-12 rounded-full">
                            <img src="https://picsum.photos/100/100?random=8" alt="User 2" />
                        </div>
                    </Avatar>
                    <Avatar>
                        <div class="w-12 rounded-full">
                            <img src="https://picsum.photos/100/100?random=9" alt="User 3" />
                        </div>
                    </Avatar>
                    <Avatar modifier=AvatarModifier::Placeholder>
                        <div class="bg-neutral text-neutral-content rounded-full w-12">
                            <span>"+99"</span>
                        </div>
                    </Avatar>
                </AvatarGroup>
            </Section>

            <Section title="Reactive Status">
                <div class="flex gap-4 items-center">
                    <Avatar modifier=online_status>
                        <div class="w-16 rounded-full">
                            <img src="https://picsum.photos/100/100?random=10" alt="Dynamic" />
                        </div>
                    </Avatar>
                    <div class="flex gap-2">
                        <button
                            class="btn btn-sm btn-success"
                            on:click=move |_| set_online_status.set(AvatarModifier::Online)
                        >
                            "Online"
                        </button>
                        <button
                            class="btn btn-sm btn-neutral"
                            on:click=move |_| set_online_status.set(AvatarModifier::Offline)
                        >
                            "Offline"
                        </button>
                    </div>
                </div>
            </Section>
        </ContentLayout>
    }
}