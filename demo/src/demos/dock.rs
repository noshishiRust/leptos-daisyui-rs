use crate::core::{ContentLayout, Section};
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;
use leptos_icons::Icon;

#[component]
pub fn DockDemo() -> impl IntoView {
    let (active_item, set_active_item) = signal(0);

    view! {
        <ContentLayout title="Dock" description="Bottom navigation bar for quick access">
            <Section title="Basic Dock">
                <div class="flex justify-center">
                    <Dock>
                        <DockItem active=Signal::derive(move || active_item.get() == 0)>
                            <Icon icon=icondata::AiHomeFilled />
                            <DockLabel>"Home"</DockLabel>
                        </DockItem>
                        <DockItem active=Signal::derive(move || active_item.get() == 1)>
                            <Icon icon=icondata::AiMessageFilled />
                            <DockLabel>"Messages"</DockLabel>
                        </DockItem>
                        <DockItem active=Signal::derive(move || active_item.get() == 3)>
                            <Icon icon=icondata::AiSettingFilled />
                            <DockLabel>"Settings"</DockLabel>
                        </DockItem>
                    </Dock>
                </div>
            </Section>

            <Section title="Interactive Dock">
                <div class="flex justify-center">
                    <Dock>
                        <DockItem
                            active=Signal::derive(move || active_item.get() == 0)
                            on:click=move |_| set_active_item.set(0)
                        >
                            <Icon icon=icondata::AiHomeFilled />
                            <DockLabel>"Home"</DockLabel>
                        </DockItem>
                        <DockItem
                            active=Signal::derive(move || active_item.get() == 1)
                            on:click=move |_| set_active_item.set(1)
                        >
                            <Icon icon=icondata::AiMessageFilled />
                            <DockLabel>"Messages"</DockLabel>
                        </DockItem>
                        <DockItem
                            active=Signal::derive(move || active_item.get() == 3)
                            on:click=move |_| set_active_item.set(3)
                        >
                            <Icon icon=icondata::AiSettingFilled />
                            <DockLabel>"Settings"</DockLabel>
                        </DockItem>
                    </Dock>
                </div>
            </Section>

            <Section title="Dock Sizes">

                <div class="flex justify-center">
                    <Dock size=DockSize::Xs>
                        <DockItem>
                            <Icon icon=icondata::AiHomeFilled />
                        </DockItem>
                        <DockItem>
                            <Icon icon=icondata::AiMessageFilled />
                        </DockItem>
                    </Dock>
                </div>
                <div class="flex justify-center">
                    <Dock size=DockSize::Sm>
                        <DockItem>
                            <Icon icon=icondata::AiHomeFilled />
                        </DockItem>
                        <DockItem>
                            <Icon icon=icondata::AiMessageFilled />
                        </DockItem>
                    </Dock>
                </div>
                <div class="flex justify-center">
                    <Dock size=DockSize::Md>
                        <DockItem>
                            <Icon icon=icondata::AiHomeFilled />
                        </DockItem>
                        <DockItem>
                            <Icon icon=icondata::AiMessageFilled />
                        </DockItem>
                    </Dock>
                </div>
                <div class="flex justify-center">
                    <Dock size=DockSize::Lg>
                        <DockItem>
                            <Icon icon=icondata::AiHomeFilled />
                        </DockItem>
                        <DockItem>
                            <Icon icon=icondata::AiMessageFilled />
                        </DockItem>
                    </Dock>
                </div>
            </Section>
        </ContentLayout>
    }
}
