use crate::core::{ContentLayout, Section};
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;
use leptos_icons::Icon;

#[component]
pub fn DockDemo() -> impl IntoView {
    let (active_item, set_active_item) = signal(0);
    let (message_count, set_message_count) = signal(3);

    view! {
        <ContentLayout title="Dock" description="Bottom navigation bar for quick access to frequently used actions">
            <Section title="Basic Dock with Labels">
                <p class="text-sm text-base-content/70 mb-4">
                    "A simple dock with icons and labels. Items can show their active state visually."
                </p>
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

            <Section title="Interactive Dock with Click Handlers">
                <p class="text-sm text-base-content/70 mb-4">
                    "Click on any item to mark it as active. The active state is managed via signals and reflects the current navigation."
                </p>
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
                            active=Signal::derive(move || active_item.get() == 2)
                            on:click=move |_| set_active_item.set(2)
                        >
                            <Icon icon=icondata::AiNotificationFilled />
                            <DockLabel>"Alerts"</DockLabel>
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

            <Section title="Dock with Badges and Notifications">
                <p class="text-sm text-base-content/70 mb-4">
                    "Combine dock items with badges to show notification counts or status indicators. Useful for messaging apps or activity tracking."
                </p>
                <div class="flex justify-center">
                    <Dock>
                        <DockItem on:click=move |_| set_active_item.set(0)>
                            <Icon icon=icondata::AiHomeFilled />
                            <DockLabel>"Home"</DockLabel>
                        </DockItem>
                        <DockItem on:click=move |_| {
                            set_active_item.set(1);
                            set_message_count.set(0);
                        }>
                            <div class="indicator">
                                {move || {
                                    if message_count.get() > 0 {
                                        view! {
                                            <Badge
                                                color=BadgeColor::Error
                                                size=BadgeSize::Xs
                                                class="indicator-item"
                                            >
                                                {move || message_count.get().to_string()}
                                            </Badge>
                                        }
                                            .into_any()
                                    } else {
                                        ().into_any()
                                    }
                                }}

                                <Icon icon=icondata::AiMessageFilled />
                            </div>
                            <DockLabel>"Messages"</DockLabel>
                        </DockItem>
                        <DockItem on:click=move |_| set_active_item.set(2)>
                            <div class="indicator">
                                <Badge color=BadgeColor::Primary size=BadgeSize::Xs class="indicator-item">
                                    ""
                                </Badge>
                                <Icon icon=icondata::AiNotificationFilled />
                            </div>
                            <DockLabel>"Alerts"</DockLabel>
                        </DockItem>
                        <DockItem on:click=move |_| set_active_item.set(3)>
                            <Icon icon=icondata::AiSettingFilled />
                            <DockLabel>"Settings"</DockLabel>
                        </DockItem>
                    </Dock>
                </div>
            </Section>

            <Section title="Dock with Tooltips">
                <p class="text-sm text-base-content/70 mb-4">
                    "Add tooltips to dock items for additional context. Hover over icons to see helpful descriptions."
                </p>
                <div class="flex justify-center">
                    <Dock>
                        <Tooltip tip="Go to homepage" position=TooltipPosition::Top>
                            <DockItem on:click=move |_| set_active_item.set(0)>
                                <Icon icon=icondata::AiHomeFilled />
                                <DockLabel>"Home"</DockLabel>
                            </DockItem>
                        </Tooltip>
                        <Tooltip tip="View your messages" position=TooltipPosition::Top>
                            <DockItem on:click=move |_| set_active_item.set(1)>
                                <Icon icon=icondata::AiMessageFilled />
                                <DockLabel>"Messages"</DockLabel>
                            </DockItem>
                        </Tooltip>
                        <Tooltip tip="Check notifications" position=TooltipPosition::Top>
                            <DockItem on:click=move |_| set_active_item.set(2)>
                                <Icon icon=icondata::AiNotificationFilled />
                                <DockLabel>"Alerts"</DockLabel>
                            </DockItem>
                        </Tooltip>
                        <Tooltip tip="Configure settings" position=TooltipPosition::Top>
                            <DockItem on:click=move |_| set_active_item.set(3)>
                                <Icon icon=icondata::AiSettingFilled />
                                <DockLabel>"Settings"</DockLabel>
                            </DockItem>
                        </Tooltip>
                    </Dock>
                </div>
            </Section>

            <Section title="Various Icon Sets">
                <p class="text-sm text-base-content/70 mb-4">
                    "Mix and match icons from different icon sets (AI, Bootstrap, Font Awesome, etc.) to create unique dock designs."
                </p>
                <div class="flex justify-center">
                    <Dock>
                        <DockItem>
                            <Icon icon=icondata::BiGridAltSolid />
                            <DockLabel>"Dashboard"</DockLabel>
                        </DockItem>
                        <DockItem>
                            <Icon icon=icondata::BsBarChartFill />
                            <DockLabel>"Analytics"</DockLabel>
                        </DockItem>
                        <DockItem>
                            <Icon icon=icondata::FaUsersSolid />
                            <DockLabel>"Team"</DockLabel>
                        </DockItem>
                        <DockItem>
                            <Icon icon=icondata::IoDocumentTextSharp />
                            <DockLabel>"Documents"</DockLabel>
                        </DockItem>
                        <DockItem>
                            <Icon icon=icondata::AiFolderFilled />
                            <DockLabel>"Files"</DockLabel>
                        </DockItem>
                    </Dock>
                </div>
            </Section>

            <Section title="Dock Size Variants">
                <p class="text-sm text-base-content/70 mb-4">
                    "Control the size of your dock with predefined size variants. Choose from extra-small to large based on your design needs."
                </p>

                <div class="space-y-4">
                    <div>
                        <p class="text-xs font-semibold mb-2">"Extra Small (xs)"</p>
                        <div class="flex justify-center">
                            <Dock size=DockSize::Xs>
                                <DockItem>
                                    <Icon icon=icondata::AiHomeFilled />
                                </DockItem>
                                <DockItem>
                                    <Icon icon=icondata::AiMessageFilled />
                                </DockItem>
                                <DockItem>
                                    <Icon icon=icondata::AiSettingFilled />
                                </DockItem>
                            </Dock>
                        </div>
                    </div>

                    <div>
                        <p class="text-xs font-semibold mb-2">"Small (sm)"</p>
                        <div class="flex justify-center">
                            <Dock size=DockSize::Sm>
                                <DockItem>
                                    <Icon icon=icondata::AiHomeFilled />
                                </DockItem>
                                <DockItem>
                                    <Icon icon=icondata::AiMessageFilled />
                                </DockItem>
                                <DockItem>
                                    <Icon icon=icondata::AiSettingFilled />
                                </DockItem>
                            </Dock>
                        </div>
                    </div>

                    <div>
                        <p class="text-xs font-semibold mb-2">"Medium (md) - Default"</p>
                        <div class="flex justify-center">
                            <Dock size=DockSize::Md>
                                <DockItem>
                                    <Icon icon=icondata::AiHomeFilled />
                                </DockItem>
                                <DockItem>
                                    <Icon icon=icondata::AiMessageFilled />
                                </DockItem>
                                <DockItem>
                                    <Icon icon=icondata::AiSettingFilled />
                                </DockItem>
                            </Dock>
                        </div>
                    </div>

                    <div>
                        <p class="text-xs font-semibold mb-2">"Large (lg)"</p>
                        <div class="flex justify-center">
                            <Dock size=DockSize::Lg>
                                <DockItem>
                                    <Icon icon=icondata::AiHomeFilled />
                                </DockItem>
                                <DockItem>
                                    <Icon icon=icondata::AiMessageFilled />
                                </DockItem>
                                <DockItem>
                                    <Icon icon=icondata::AiSettingFilled />
                                </DockItem>
                            </Dock>
                        </div>
                    </div>
                </div>
            </Section>
        </ContentLayout>
    }
}
