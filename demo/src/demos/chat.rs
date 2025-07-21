use crate::core::{ContentLayout, Section};
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn ChatDemo() -> impl IntoView {
    let selected_color = RwSignal::new(ChatBubbleColor::default());
    let message_text = RwSignal::new("Hello World!".to_string());
    let show_avatar = RwSignal::new(true);
    let show_timestamp = RwSignal::new(true);

    view! {
        <ContentLayout
            title="Chat"
            description="Chat bubbles are used to show conversations between multiple people"
        >
            <Section title="Basic Usage" col=true>
                <Chat placement=ChatPlacement::Start>
                    <ChatBubble>"It's over Anakin, I have the high ground."</ChatBubble>
                </Chat>
                <Chat placement=ChatPlacement::End>
                    <ChatBubble>"You underestimate my power!"</ChatBubble>
                </Chat>
            </Section>

            <Section title="Interactive Message" col=true>
                <div class="form-control">
                    <label class="label cursor-pointer">
                        <span class="label-text">"Show Timestamp"</span>
                        <Checkbox
                            prop:checked=move || show_timestamp.get()
                            on:change=move |e| show_timestamp.set(event_target_checked(&e))
                        />
                    </label>
                </div>

                <Chat placement=ChatPlacement::Start>
                    {move || {
                        show_avatar
                            .get()
                            .then(|| {
                                view! {
                                    <ChatImage class="avatar">
                                        <div class="w-10 rounded-full">
                                            <img
                                                src="https://picsum.photos/40/40?random=1"
                                                alt="User"
                                            />
                                        </div>
                                    </ChatImage>
                                }
                            })
                    }}
                    {move || {
                        show_timestamp
                            .get()
                            .then(|| {
                                view! {
                                    <ChatHeader>
                                        "User" <time class="text-xs opacity-50">"12:45"</time>
                                    </ChatHeader>
                                }
                            })
                    }} <ChatBubble color=selected_color>{move || message_text.get()}</ChatBubble>
                </Chat>
            </Section>

            <Section title="Color Variants" row=true>
                <Chat placement=ChatPlacement::Start>
                    <ChatBubble color=ChatBubbleColor::Default>"Default"</ChatBubble>
                </Chat>
                <Chat placement=ChatPlacement::Start>
                    <ChatBubble color=ChatBubbleColor::Primary>"Primary"</ChatBubble>
                </Chat>
                <Chat placement=ChatPlacement::Start>
                    <ChatBubble color=ChatBubbleColor::Secondary>"Secondary"</ChatBubble>
                </Chat>
                <Chat placement=ChatPlacement::Start>
                    <ChatBubble color=ChatBubbleColor::Success>"Success"</ChatBubble>
                </Chat>
                <Chat placement=ChatPlacement::Start>
                    <ChatBubble color=ChatBubbleColor::Error>"Error"</ChatBubble>
                </Chat>
            </Section>

            <Section title="Complete Chat Example" col=true>
                <div class="h-64 overflow-y-auto bg-base-200 p-4 rounded-lg space-y-2">
                    <Chat placement=ChatPlacement::Start>
                        <ChatImage class="avatar">
                            <div class="w-10 rounded-full">
                                <img src="https://picsum.photos/40/40?random=3" alt="Alice" />
                            </div>
                        </ChatImage>
                        <ChatHeader>
                            "Alice" <time class="text-xs opacity-50">"2 hours ago"</time>
                        </ChatHeader>
                        <ChatBubble>"Hey! How are you doing?"</ChatBubble>
                    </Chat>

                    <Chat placement=ChatPlacement::End>
                        <ChatImage class="avatar">
                            <div class="w-10 rounded-full">
                                <img src="https://picsum.photos/40/40?random=4" alt="Bob" />
                            </div>
                        </ChatImage>
                        <ChatHeader>
                            "Bob" <time class="text-xs opacity-50">"2 hours ago"</time>
                        </ChatHeader>
                        <ChatBubble color=ChatBubbleColor::Primary>
                            "I'm doing great! Just working on some Leptos components."
                        </ChatBubble>
                    </Chat>

                    <Chat placement=ChatPlacement::Start>
                        <ChatImage class="avatar">
                            <div class="w-10 rounded-full">
                                <img src="https://picsum.photos/40/40?random=3" alt="Alice" />
                            </div>
                        </ChatImage>
                        <ChatBubble>"That sounds awesome! Are you using daisyUI?"</ChatBubble>
                    </Chat>

                    <Chat placement=ChatPlacement::End>
                        <ChatImage class="avatar">
                            <div class="w-10 rounded-full">
                                <img src="https://picsum.photos/40/40?random=4" alt="Bob" />
                            </div>
                        </ChatImage>
                        <ChatBubble color=ChatBubbleColor::Success>
                            "Yes! The leptos-daisyui-rs library is fantastic."
                        </ChatBubble>
                        <ChatFooter class="opacity-50">"Delivered"</ChatFooter>
                    </Chat>
                </div>
            </Section>
        </ContentLayout>
    }
}
