use crate::core::{ContentLayout, Section};
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;
use leptos_icons::Icon;

#[component]
pub fn MockupPhoneDemo() -> impl IntoView {
    view! {
        <ContentLayout
            title="Mockup Phone"
            description="Mobile phone frame for displaying mobile interfaces"
        >
            <Section title="Basic Phone Mockup">
                <div class="flex justify-center">
                    <MockupPhone>
                        <div class="bg-primary text-primary-content p-4">
                            <div class="text-center">
                                <h2 class="text-2xl font-bold mb-4">"Mobile App"</h2>
                                <p class="mb-6">"Welcome to our mobile application!"</p>
                                <Button color=ButtonColor::Secondary size=ButtonSize::Sm>
                                    "Get Started"
                                </Button>
                            </div>
                        </div>
                    </MockupPhone>
                </div>
            </Section>

            <Section title="Chat Interface">
                <div class="flex justify-center">
                    <MockupPhone>
                        <div class="h-full bg-base-100 flex flex-col">
                            <div class="bg-primary text-primary-content p-3 flex items-center justify-between">
                                <div class="flex items-center gap-3">
                                    <Avatar>
                                        <div class="w-8 rounded-full bg-primary-content/20">
                                            <Icon
                                                icon=icondata::BiUserRegular
                                                attr:class="w-full h-full p-1"
                                            />
                                        </div>
                                    </Avatar>
                                    <div>
                                        <div class="font-semibold">"Alice"</div>
                                        <div class="text-xs opacity-70">"Online"</div>
                                    </div>
                                </div>
                                <Icon icon=icondata::AiPhoneFilled />
                            </div>

                            <div class="flex-1 p-3 space-y-3 overflow-y-auto">
                                <Chat>
                                    <ChatImage>
                                        <Avatar>
                                            <div class="w-8 rounded-full bg-base-300">
                                                <Icon
                                                    icon=icondata::BiUserRegular
                                                    attr:class="w-full h-full p-1"
                                                />
                                            </div>
                                        </Avatar>
                                    </ChatImage>
                                    <ChatBubble color=ChatBubbleColor::Primary>
                                        "Hey there! How are you doing?"
                                    </ChatBubble>
                                </Chat>

                                <Chat placement=ChatPlacement::End>
                                    <ChatBubble color=ChatBubbleColor::Success>
                                        "Hi! I'm doing great, thanks for asking 😊"
                                    </ChatBubble>
                                </Chat>

                                <Chat>
                                    <ChatImage>
                                        <Avatar>
                                            <div class="w-8 rounded-full bg-base-300">
                                                <Icon
                                                    icon=icondata::BiUserRegular
                                                    attr:class="w-full h-full p-1"
                                                />
                                            </div>
                                        </Avatar>
                                    </ChatImage>
                                    <ChatBubble color=ChatBubbleColor::Primary>
                                        "That's wonderful to hear!"
                                    </ChatBubble>
                                </Chat>
                            </div>

                            <div class="p-3 border-t border-base-300">
                                <div class="flex gap-2">
                                    <Input attr:placeholder="Type a message..." class="flex-1" />
                                    <Button color=ButtonColor::Primary size=ButtonSize::Sm>
                                        <Icon icon=icondata::AiSendOutlined />
                                    </Button>
                                </div>
                            </div>
                        </div>
                    </MockupPhone>
                </div>
            </Section>

            <Section title="Profile Screen">
                <div class="flex justify-center">
                    <MockupPhoneCamera>
                        <div class="h-full bg-gradient-to-b from-purple-500 to-pink-500 text-white">
                            <div class="p-6 text-center">
                                <Avatar class="mb-4">
                                    <div class="w-20 h-20 rounded-full bg-white/20 flex items-center justify-center">
                                        <span class="text-2xl font-bold">"JS"</span>
                                    </div>
                                </Avatar>
                                <h3 class="text-xl font-bold mb-1">"Jane Smith"</h3>
                                <p class="text-purple-100 mb-4">"@janesmith"</p>

                                <div class="grid grid-cols-3 gap-4 mb-6 text-center">
                                    <div>
                                        <div class="text-lg font-bold">"1.2K"</div>
                                        <div class="text-xs text-purple-200">"Followers"</div>
                                    </div>
                                    <div>
                                        <div class="text-lg font-bold">"856"</div>
                                        <div class="text-xs text-purple-200">"Following"</div>
                                    </div>
                                    <div>
                                        <div class="text-lg font-bold">"42"</div>
                                        <div class="text-xs text-purple-200">"Posts"</div>
                                    </div>
                                </div>

                                <div class="flex gap-2 justify-center">
                                    <Button
                                        size=ButtonSize::Sm
                                        style=ButtonStyle::Outline
                                        class="border-white text-white hover:bg-white hover:text-purple-500"
                                    >
                                        "Follow"
                                    </Button>
                                    <Button
                                        size=ButtonSize::Sm
                                        style=ButtonStyle::Outline
                                        class="border-white text-white hover:bg-white hover:text-purple-500"
                                    >
                                        "Message"
                                    </Button>
                                </div>
                            </div>
                        </div>
                    </MockupPhoneCamera>
                </div>
            </Section>
        </ContentLayout>
    }
}
