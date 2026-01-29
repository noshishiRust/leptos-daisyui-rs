use crate::core::{ContentLayout, Section};
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;
use leptos_icons::Icon;

#[component]
pub fn ListDemo() -> impl IntoView {
    view! {
        <ContentLayout
            title="List"
            description="List component is used to display a series of items"
        >
            <Section title="Basic List">
                <div class="list list-col-wrap bg-base-200 rounded-box p-4">
                    <div class="px-4 py-2">"Item 1"</div>
                    <div class="px-4 py-2">"Item 2"</div>
                    <div class="px-4 py-2">"Item 3"</div>
                    <div class="px-4 py-2">"Item 4"</div>
                </div>
            </Section>

            <Section title="List with Icons">
                <div class="list list-col-wrap bg-base-200 rounded-box p-4">
                    <div class="flex items-center gap-3 px-4 py-2">
                        <Icon icon=icondata::AiHomeOutlined />
                        "Home"
                    </div>
                    <div class="flex items-center gap-3 px-4 py-2">
                        <Icon icon=icondata::AiUserOutlined />
                        "Profile"
                    </div>
                    <div class="flex items-center gap-3 px-4 py-2">
                        <Icon icon=icondata::AiSettingOutlined />
                        "Settings"
                    </div>
                    <div class="flex items-center gap-3 px-4 py-2">
                        <Icon icon=icondata::AiQuestionCircleOutlined />
                        "Help"
                    </div>
                </div>
            </Section>

            <Section title="List with Actions">
                <div class="list list-col-wrap bg-base-200 rounded-box p-4">
                    <div class="flex items-center justify-between px-4 py-2">
                        <div class="flex items-center gap-3">
                            <Icon icon=icondata::AiFileTextOutlined />
                            "Document.pdf"
                        </div>
                        <div class="flex gap-2">
                            <Button size=ButtonSize::Sm style=ButtonStyle::Ghost>
                                <Icon icon=icondata::AiDownloadOutlined />
                            </Button>
                            <Button size=ButtonSize::Sm style=ButtonStyle::Ghost>
                                <Icon icon=icondata::AiDeleteOutlined />
                            </Button>
                        </div>
                    </div>
                    <div class="flex items-center justify-between px-4 py-2">
                        <div class="flex items-center gap-3">
                            <Icon icon=icondata::AiFileImageOutlined />
                            "Image.jpg"
                        </div>
                        <div class="flex gap-2">
                            <Button size=ButtonSize::Sm style=ButtonStyle::Ghost>
                                <Icon icon=icondata::AiEyeOutlined />
                            </Button>
                            <Button size=ButtonSize::Sm style=ButtonStyle::Ghost>
                                <Icon icon=icondata::AiDownloadOutlined />
                            </Button>
                        </div>
                    </div>
                    <div class="flex items-center justify-between px-4 py-2">
                        <div class="flex items-center gap-3">
                            <Icon icon=icondata::AiFileOutlined />
                            "Spreadsheet.xlsx"
                        </div>
                        <div class="flex gap-2">
                            <Button size=ButtonSize::Sm style=ButtonStyle::Ghost>
                                <Icon icon=icondata::AiEditOutlined />
                            </Button>
                            <Button size=ButtonSize::Sm style=ButtonStyle::Ghost>
                                <Icon icon=icondata::AiShareAltOutlined />
                            </Button>
                        </div>
                    </div>
                </div>
            </Section>

            <Section title="Contact List">
                <div class="list list-col-wrap bg-base-200 rounded-box p-4">
                    <div class="flex items-center gap-3 px-4 py-2">
                        <Avatar>
                            <div class="w-10 h-10 rounded-full">
                                <img src="https://picsum.photos/40/40?random=1" alt="Contact" />
                            </div>
                        </Avatar>
                        <div>
                            <div class="font-bold">"John Doe"</div>
                            <div class="text-sm opacity-50">"john@example.com"</div>
                        </div>
                    </div>
                    <div class="flex items-center gap-3 px-4 py-2">
                        <Avatar>
                            <div class="w-10 h-10 rounded-full">
                                <img src="https://picsum.photos/40/40?random=2" alt="Contact" />
                            </div>
                        </Avatar>
                        <div>
                            <div class="font-bold">"Jane Smith"</div>
                            <div class="text-sm opacity-70">"jane@example.com"</div>
                        </div>
                    </div>
                    <div class="flex items-center gap-3 px-4 py-2">
                        <Avatar>
                            <div class="w-10 h-10 rounded-full">
                                <img src="https://picsum.photos/40/40?random=3" alt="Contact" />
                            </div>
                        </Avatar>
                        <div>
                            <div class="font-bold">"Bob Johnson"</div>
                            <div class="text-sm opacity-70">"bob@example.com"</div>
                        </div>
                    </div>
                </div>
            </Section>

            <Section title="Notification List">
                <div class="list list-col-wrap bg-base-200 rounded-box p-4">
                    <div class="flex items-start gap-3 w-full px-4 py-2">
                        <Icon icon=icondata::AiBellOutlined />
                        <div class="flex-1">
                            <div class="font-semibold">"New message received"</div>
                            <div class="text-sm opacity-70">"You have a new message from John Doe"</div>
                            <div class="text-xs opacity-50 mt-1">"2 minutes ago"</div>
                        </div>
                        <Badge color=BadgeColor::Primary size=BadgeSize::Sm>"New"</Badge>
                    </div>
                    <div class="flex items-start gap-3 w-full px-4 py-2">
                        <Icon icon=icondata::AiCheckCircleOutlined />
                        <div class="flex-1">
                            <div class="font-semibold">"Task completed"</div>
                            <div class="text-sm opacity-70">"Project documentation has been completed"</div>
                            <div class="text-xs opacity-50 mt-1">"1 hour ago"</div>
                        </div>
                    </div>
                    <div class="flex items-start gap-3 w-full px-4 py-2">
                        <Icon icon=icondata::AiExclamationCircleOutlined />
                        <div class="flex-1">
                            <div class="font-semibold">"System maintenance"</div>
                            <div class="text-sm opacity-70">"Scheduled maintenance will begin at 2:00 AM"</div>
                            <div class="text-xs opacity-50 mt-1">"3 hours ago"</div>
                        </div>
                    </div>
                </div>
            </Section>

            <Section title="Settings List">
                <div class="list list-col-wrap bg-base-200 rounded-box p-4">
                    <div class="flex items-center justify-between px-4 py-2">
                        <div class="flex items-center gap-3">
                            <Icon icon=icondata::AiBellOutlined />
                            <div>
                                <div class="font-semibold">"Push Notifications"</div>
                                <div class="text-sm opacity-70">"Receive push notifications"</div>
                            </div>
                        </div>
                        <Toggle attr:checked=true />
                    </div>
                    <div class="flex items-center justify-between px-4 py-2">
                        <div class="flex items-center gap-3">
                            <Icon icon=icondata::AiMailOutlined />
                            <div>
                                <div class="font-semibold">"Email Notifications"</div>
                                <div class="text-sm opacity-70">"Receive email updates"</div>
                            </div>
                        </div>
                        <Toggle attr:checked=false />
                    </div>
                    <div class="flex items-center justify-between px-4 py-2">
                        <div class="flex items-center gap-3">
                            <Icon icon=icondata::AiLockOutlined />
                            <div>
                                <div class="font-semibold">"Two-Factor Authentication"</div>
                                <div class="text-sm opacity-70">"Enhanced security for your account"</div>
                            </div>
                        </div>
                        <Toggle attr:checked=true />
                    </div>
                </div>
            </Section>

            <Section title="Task List">
                <div class="list list-col-wrap bg-base-200 rounded-box p-4">
                    <div class="flex items-center gap-3 w-full px-4 py-2">
                        <Checkbox attr:checked=true />
                        <div class="flex-1">
                            <div class="line-through opacity-50">"Complete project documentation"</div>
                            <div class="text-sm opacity-50">"Due: Yesterday"</div>
                        </div>
                        <Badge color=BadgeColor::Success size=BadgeSize::Sm>"Done"</Badge>
                    </div>
                    <div class="flex items-center gap-3 w-full px-4 py-2">
                        <Checkbox attr:checked=false />
                        <div class="flex-1">
                            <div class="font-semibold">"Review pull requests"</div>
                            <div class="text-sm opacity-70">"Due: Today"</div>
                        </div>
                        <Badge color=BadgeColor::Warning size=BadgeSize::Sm>"In Progress"</Badge>
                    </div>
                    <div class="flex items-center gap-3 w-full px-4 py-2">
                        <Checkbox attr:checked=false />
                        <div class="flex-1">
                            <div class="font-semibold">"Prepare presentation"</div>
                            <div class="text-sm opacity-70">"Due: Tomorrow"</div>
                        </div>
                        <Badge color=BadgeColor::Info size=BadgeSize::Sm>"Pending"</Badge>
                    </div>
                </div>
            </Section>

            <Section title="Product List">
                <div class="list list-col-wrap bg-base-200 rounded-box p-4">
                    <div class="flex items-center gap-4 w-full px-4 py-2">
                        <div class="w-16 h-16 bg-base-300 rounded-lg flex items-center justify-center">
                            <Icon icon=icondata::AiShoppingOutlined />
                        </div>
                        <div class="flex-1">
                            <div class="font-bold">"Wireless Headphones"</div>
                            <div class="text-sm opacity-70">"High-quality sound with noise cancellation"</div>
                            <div class="flex items-center gap-2 mt-1">
                                <Badge color=BadgeColor::Primary size=BadgeSize::Sm>"Electronics"</Badge>
                                <span class="text-lg font-bold text-primary">"$99.99"</span>
                            </div>
                        </div>
                        <Button color=ButtonColor::Primary size=ButtonSize::Sm>
                            "Add to Cart"
                        </Button>
                    </div>
                    <div class="flex items-center gap-4 w-full px-4 py-2">
                        <div class="w-16 h-16 bg-base-300 rounded-lg flex items-center justify-center">
                            <Icon icon=icondata::AiLaptopOutlined />
                        </div>
                        <div class="flex-1">
                            <div class="font-bold">"Gaming Laptop"</div>
                            <div class="text-sm opacity-70">"High-performance laptop for gaming and work"</div>
                            <div class="flex items-center gap-2 mt-1">
                                <Badge color=BadgeColor::Secondary size=BadgeSize::Sm>"Computers"</Badge>
                                <span class="text-lg font-bold text-primary">"$1,299.99"</span>
                            </div>
                        </div>
                        <Button color=ButtonColor::Primary size=ButtonSize::Sm>
                            "Add to Cart"
                        </Button>
                    </div>
                </div>
            </Section>

            <Section title="Chat List">
                <div class="list list-col-wrap bg-base-200 rounded-box p-4">
                    <div class="flex items-center gap-3 w-full px-4 py-2">
                        <div class="relative">
                            <Avatar>
                                <div class="w-12 h-12 rounded-full">
                                    <img src="https://picsum.photos/48/48?random=10" alt="User" />
                                </div>
                            </Avatar>
                            <div class="absolute bottom-0 right-0 w-3 h-3 bg-success rounded-full border-2 border-base-100"></div>
                        </div>
                        <div class="flex-1 min-w-0">
                            <div class="font-semibold truncate">"Alice Cooper"</div>
                            <div class="text-sm opacity-70 truncate">"Hey, how are you doing?"</div>
                        </div>
                        <div class="text-right">
                            <div class="text-xs opacity-50">"2:30 PM"</div>
                            <Badge color=BadgeColor::Primary size=BadgeSize::Sm class="mt-1">"3"</Badge>
                        </div>
                    </div>
                    <div class="flex items-center gap-3 w-full px-4 py-2">
                        <div class="relative">
                            <Avatar>
                                <div class="w-12 h-12 rounded-full">
                                    <img src="https://picsum.photos/48/48?random=11" alt="User" />
                                </div>
                            </Avatar>
                            <div class="absolute bottom-0 right-0 w-3 h-3 bg-base-300 rounded-full border-2 border-base-100"></div>
                        </div>
                        <div class="flex-1 min-w-0">
                            <div class="font-semibold truncate">"Bob Wilson"</div>
                            <div class="text-sm opacity-70 truncate">"Can we schedule a meeting for tomorrow?"</div>
                        </div>
                        <div class="text-right">
                            <div class="text-xs opacity-50">"1:15 PM"</div>
                        </div>
                    </div>
                    <div class="flex items-center gap-3 w-full px-4 py-2">
                        <div class="relative">
                            <Avatar>
                                <div class="w-12 h-12 rounded-full">
                                    <img src="https://picsum.photos/48/48?random=12" alt="User" />
                                </div>
                            </Avatar>
                            <div class="absolute bottom-0 right-0 w-3 h-3 bg-warning rounded-full border-2 border-base-100"></div>
                        </div>
                        <div class="flex-1 min-w-0">
                            <div class="font-semibold truncate">"Carol Davis"</div>
                            <div class="text-sm opacity-70 truncate">"The project looks great! 👏"</div>
                        </div>
                        <div class="text-right">
                            <div class="text-xs opacity-50">"11:45 AM"</div>
                            <Badge color=BadgeColor::Success size=BadgeSize::Sm class="mt-1">"1"</Badge>
                        </div>
                    </div>
                </div>
            </Section>
        </ContentLayout>
    }
}
