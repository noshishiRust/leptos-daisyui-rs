use leptos::prelude::*;

#[component]
pub fn ChatDemo() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <h1 class="text-3xl font-bold">"Chat"</h1>
            <p class="text-base-content/70">
                "Chat bubbles are used to show conversations between multiple people"
            </p>

            <div class="space-y-4">
                <h2 class="text-xl font-semibold">"Basic Chat"</h2>
                <div class="chat chat-start">
                    <div class="chat-bubble">"It's over Anakin, I have the high ground."</div>
                </div>
                <div class="chat chat-end">
                    <div class="chat-bubble">"You underestimate my power!"</div>
                </div>

                <h2 class="text-xl font-semibold">"Chat with Header and Footer"</h2>
                <div class="chat chat-start">
                    <div class="chat-image avatar">
                        <div class="w-10 rounded-full">
                            <img alt="Tailwind CSS chat bubble component" src="https://picsum.photos/40/40?random=1" />
                        </div>
                    </div>
                    <div class="chat-header">
                        "Obi-Wan Kenobi"
                        <time class="text-xs opacity-50">"12:45"</time>
                    </div>
                    <div class="chat-bubble">"You were the Chosen One!"</div>
                    <div class="chat-footer opacity-50">"Delivered"</div>
                </div>
                <div class="chat chat-end">
                    <div class="chat-image avatar">
                        <div class="w-10 rounded-full">
                            <img alt="Tailwind CSS chat bubble component" src="https://picsum.photos/40/40?random=2" />
                        </div>
                    </div>
                    <div class="chat-header">
                        "Anakin"
                        <time class="text-xs opacity-50">"12:46"</time>
                    </div>
                    <div class="chat-bubble">"I hate you!"</div>
                    <div class="chat-footer opacity-50">"Seen at 12:46"</div>
                </div>

                <h2 class="text-xl font-semibold">"Chat Bubble Colors"</h2>
                <div class="chat chat-start">
                    <div class="chat-bubble">"Default color"</div>
                </div>
                <div class="chat chat-start">
                    <div class="chat-bubble chat-bubble-primary">"Primary color"</div>
                </div>
                <div class="chat chat-start">
                    <div class="chat-bubble chat-bubble-secondary">"Secondary color"</div>
                </div>
                <div class="chat chat-start">
                    <div class="chat-bubble chat-bubble-accent">"Accent color"</div>
                </div>
                <div class="chat chat-start">
                    <div class="chat-bubble chat-bubble-info">"Info color"</div>
                </div>
                <div class="chat chat-start">
                    <div class="chat-bubble chat-bubble-success">"Success color"</div>
                </div>
                <div class="chat chat-start">
                    <div class="chat-bubble chat-bubble-warning">"Warning color"</div>
                </div>
                <div class="chat chat-start">
                    <div class="chat-bubble chat-bubble-error">"Error color"</div>
                </div>

                <h2 class="text-xl font-semibold">"Chat Conversation"</h2>
                <div class="h-64 overflow-y-auto bg-base-200 p-4 rounded-lg">
                    <div class="chat chat-start">
                        <div class="chat-image avatar">
                            <div class="w-10 rounded-full">
                                <img src="https://picsum.photos/40/40?random=3" alt="User 1" />
                            </div>
                        </div>
                        <div class="chat-header">
                            "Alice"
                            <time class="text-xs opacity-50">"2 hours ago"</time>
                        </div>
                        <div class="chat-bubble">"Hey! How are you doing?"</div>
                    </div>

                    <div class="chat chat-end">
                        <div class="chat-image avatar">
                            <div class="w-10 rounded-full">
                                <img src="https://picsum.photos/40/40?random=4" alt="User 2" />
                            </div>
                        </div>
                        <div class="chat-header">
                            "Bob"
                            <time class="text-xs opacity-50">"2 hours ago"</time>
                        </div>
                        <div class="chat-bubble chat-bubble-primary">"I'm doing great! Just working on some Leptos components."</div>
                    </div>

                    <div class="chat chat-start">
                        <div class="chat-image avatar">
                            <div class="w-10 rounded-full">
                                <img src="https://picsum.photos/40/40?random=3" alt="User 1" />
                            </div>
                        </div>
                        <div class="chat-bubble">"That sounds awesome! Are you using daisyUI?"</div>
                    </div>

                    <div class="chat chat-end">
                        <div class="chat-image avatar">
                            <div class="w-10 rounded-full">
                                <img src="https://picsum.photos/40/40?random=4" alt="User 2" />
                            </div>
                        </div>
                        <div class="chat-bubble chat-bubble-primary">"Yes! The leptos-daisyui-rs library is fantastic. The components are so easy to use."</div>
                    </div>

                    <div class="chat chat-start">
                        <div class="chat-image avatar">
                            <div class="w-10 rounded-full">
                                <img src="https://picsum.photos/40/40?random=3" alt="User 1" />
                            </div>
                        </div>
                        <div class="chat-bubble">"I should check that out. Thanks for the recommendation!"</div>
                        <div class="chat-footer opacity-50">"Delivered"</div>
                    </div>

                    <div class="chat chat-end">
                        <div class="chat-image avatar">
                            <div class="w-10 rounded-full">
                                <img src="https://picsum.photos/40/40?random=4" alt="User 2" />
                            </div>
                        </div>
                        <div class="chat-bubble chat-bubble-success">"You definitely should! Happy coding! 🚀"</div>
                        <div class="chat-footer opacity-50">"Seen"</div>
                    </div>
                </div>
            </div>
        </div>
    }
}