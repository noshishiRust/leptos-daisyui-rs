use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn AvatarDemo() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <h1 class="text-3xl font-bold">"Avatar"</h1>
            <p class="text-base-content/70">
                "Avatars are used to show a thumbnail representation of a user"
            </p>

            <div class="space-y-4">
                <h2 class="text-xl font-semibold">"Basic Avatars"</h2>
                <div class="flex gap-4 items-center">
                    <Avatar>
                        <div class="w-24 rounded-full">
                            <img src="https://picsum.photos/100/100?random=1" alt="Avatar" />
                        </div>
                    </Avatar>

                    <Avatar>
                        <div class="w-16 rounded-full">
                            <img src="https://picsum.photos/100/100?random=2" alt="Avatar" />
                        </div>
                    </Avatar>

                    <Avatar>
                        <div class="w-12 rounded-full">
                            <img src="https://picsum.photos/100/100?random=3" alt="Avatar" />
                        </div>
                    </Avatar>
                </div>

                <h2 class="text-xl font-semibold">"Avatar with Status"</h2>
                <div class="flex gap-4 items-center">
                    <Avatar modifier=AvatarModifier::Online>
                        <div class="w-20 rounded-full">
                            <img src="https://picsum.photos/100/100?random=4" alt="Avatar" />
                        </div>
                    </Avatar>

                    <Avatar modifier=AvatarModifier::Offline>
                        <div class="w-20 rounded-full">
                            <img src="https://picsum.photos/100/100?random=5" alt="Avatar" />
                        </div>
                    </Avatar>
                </div>

                <h2 class="text-xl font-semibold">"Placeholder Avatar"</h2>
                <Avatar modifier=AvatarModifier::Placeholder>
                    <div class="bg-neutral text-neutral-content rounded-full w-16">
                        <span class="text-xl">"K"</span>
                    </div>
                </Avatar>

                <h2 class="text-xl font-semibold">"Avatar Group"</h2>
                <AvatarGroup class="-space-x-4">
                    <Avatar>
                        <div class="w-12 rounded-full">
                            <img src="https://picsum.photos/100/100?random=6" alt="Avatar" />
                        </div>
                    </Avatar>
                    <Avatar>
                        <div class="w-12 rounded-full">
                            <img src="https://picsum.photos/100/100?random=7" alt="Avatar" />
                        </div>
                    </Avatar>
                    <Avatar>
                        <div class="w-12 rounded-full">
                            <img src="https://picsum.photos/100/100?random=8" alt="Avatar" />
                        </div>
                    </Avatar>
                    <Avatar modifier=AvatarModifier::Placeholder>
                        <div class="w-12 bg-neutral text-neutral-content rounded-full">
                            <span>"+3"</span>
                        </div>
                    </Avatar>
                </AvatarGroup>
            </div>
        </div>
    }
}