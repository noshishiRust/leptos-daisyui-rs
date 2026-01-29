use leptos::prelude::*;
use leptos_daisyui_rs::components::{Swap, SwapOff, SwapOn, SwapRotate};

#[component]
pub fn SwapDemo() -> impl IntoView {
    let (active1, set_active1) = signal(false);
    let (active2, set_active2) = signal(false);
    let (active3, set_active3) = signal(false);

    view! {
        <div class="space-y-8 p-8">
            <div class="space-y-4">
                <h2 class="text-2xl font-bold">"Swap Component"</h2>
                <p class="text-base-content/70">
                    "Toggle between two elements with different animation styles."
                </p>
            </div>

            // Basic Swap
            <div class="space-y-2">
                <h3 class="text-xl font-semibold">"Basic Swap"</h3>
                <div class="flex gap-4 items-center">
                    <Swap active=active1 on:click=move |_| set_active1.update(|v| *v = !*v)>
                        <SwapOn>
                            <div class="text-6xl">"😄"</div>
                        </SwapOn>
                        <SwapOff>
                            <div class="text-6xl">"😭"</div>
                        </SwapOff>
                    </Swap>
                    <span class="text-sm text-base-content/70">"Click to toggle"</span>
                </div>
            </div>

            // Swap with Rotate Animation
            <div class="space-y-2">
                <h3 class="text-xl font-semibold">"Rotate Animation"</h3>
                <div class="flex gap-4 items-center">
                    <Swap
                        rotate=SwapRotate::Rotate
                        active=active2
                        on:click=move |_| set_active2.update(|v| *v = !*v)
                    >
                        <SwapOn>
                            <svg
                                class="fill-current w-10 h-10"
                                xmlns="http://www.w3.org/2000/svg"
                                viewBox="0 0 24 24"
                            >
                                <path d="M14,3.23V5.29C16.89,6.15 19,8.83 19,12C19,15.17 16.89,17.84 14,18.7V20.77C18,19.86 21,16.28 21,12C21,7.72 18,4.14 14,3.23M16.5,12C16.5,10.23 15.5,8.71 14,7.97V16C15.5,15.29 16.5,13.76 16.5,12M3,9V15H7L12,20V4L7,9H3Z" />
                            </svg>
                        </SwapOn>
                        <SwapOff>
                            <svg
                                class="fill-current w-10 h-10"
                                xmlns="http://www.w3.org/2000/svg"
                                viewBox="0 0 24 24"
                            >
                                <path d="M3,9H7L12,4V20L7,15H3V9M16.59,12L14,9.41L15.41,8L18,10.59L20.59,8L22,9.41L19.41,12L22,14.59L20.59,16L18,13.41L15.41,16L14,14.59L16.59,12Z" />
                            </svg>
                        </SwapOff>
                    </Swap>
                    <span class="text-sm text-base-content/70">"Volume control with rotation"</span>
                </div>
            </div>

            // Swap with Flip Animation
            <div class="space-y-2">
                <h3 class="text-xl font-semibold">"Flip Animation"</h3>
                <div class="flex gap-4 items-center">
                    <Swap
                        rotate=SwapRotate::Flip
                        active=active3
                        on:click=move |_| set_active3.update(|v| *v = !*v)
                        class="text-6xl"
                    >
                        <SwapOn>"🌙"</SwapOn>
                        <SwapOff>"☀️"</SwapOff>
                    </Swap>
                    <span class="text-sm text-base-content/70">"Day/Night toggle with flip"</span>
                </div>
            </div>

            // Multiple Swaps in a Row
            <div class="space-y-2">
                <h3 class="text-xl font-semibold">"Multiple Swaps"</h3>
                <div class="flex gap-8">
                    {move || {
                        (0..5)
                            .map(|_i| {
                                let (active, set_active) = signal(false);
                                view! {
                                    <Swap
                                        rotate=SwapRotate::Rotate
                                        active=active
                                        on:click=move |_| set_active.update(|v| *v = !*v)
                                        class="text-4xl"
                                    >
                                        <SwapOn>"⭐"</SwapOn>
                                        <SwapOff>"☆"</SwapOff>
                                    </Swap>
                                }
                            })
                            .collect_view()
                    }}

                </div>
            </div>

            // Code Example
            <div class="space-y-2">
                <h3 class="text-xl font-semibold">"Usage Example"</h3>
                <div class="mockup-code text-sm">
                    <pre data-prefix="$">
                        <code>
                            "let (active, set_active) = signal(false);"
                        </code>
                    </pre>
                    <pre data-prefix=">">
                        <code></code>
                    </pre>
                    <pre data-prefix=">">
                        <code>
                            {"<Swap active=active on:click=move |_| set_active.update(|v| *v = !*v)>"}
                        </code>
                    </pre>
                    <pre data-prefix=">">
                        <code>"  <SwapOn><div>\"ON\"</div></SwapOn>"</code>
                    </pre>
                    <pre data-prefix=">">
                        <code>"  <SwapOff><div>\"OFF\"</div></SwapOff>"</code>
                    </pre>
                    <pre data-prefix=">">
                        <code>"</Swap>"</code>
                    </pre>
                </div>
            </div>

            // Animation Variants
            <div class="space-y-2">
                <h3 class="text-xl font-semibold">"Animation Variants"</h3>
                <div class="overflow-x-auto">
                    <table class="table table-zebra">
                        <thead>
                            <tr>
                                <th>"Variant"</th>
                                <th>"Description"</th>
                                <th>"Usage"</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td><code>"SwapRotate::None"</code></td>
                                <td>"No animation (default)"</td>
                                <td><code>"rotate=SwapRotate::None"</code></td>
                            </tr>
                            <tr>
                                <td><code>"SwapRotate::Rotate"</code></td>
                                <td>"180° rotation animation"</td>
                                <td><code>"rotate=SwapRotate::Rotate"</code></td>
                            </tr>
                            <tr>
                                <td><code>"SwapRotate::Flip"</code></td>
                                <td>"3D flip animation"</td>
                                <td><code>"rotate=SwapRotate::Flip"</code></td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>
        </div>
    }
}
