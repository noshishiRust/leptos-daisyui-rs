use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn KbdDemo() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <h1 class="text-3xl font-bold">"Kbd"</h1>
            <p class="text-base-content/70">
                "Kbd is used to display keyboard shortcuts or key combinations"
            </p>

            <div class="space-y-4">
                <h2 class="text-xl font-semibold">"Basic Keyboard Keys"</h2>
                <div class="flex gap-2 flex-wrap">
                    <kbd class="kbd">"A"</kbd>
                    <kbd class="kbd">"B"</kbd>
                    <kbd class="kbd">"C"</kbd>
                    <kbd class="kbd">"D"</kbd>
                    <kbd class="kbd">"E"</kbd>
                </div>

                <h2 class="text-xl font-semibold">"Special Keys"</h2>
                <div class="flex gap-2 flex-wrap">
                    <kbd class="kbd">"Tab"</kbd>
                    <kbd class="kbd">"Shift"</kbd>
                    <kbd class="kbd">"Ctrl"</kbd>
                    <kbd class="kbd">"Alt"</kbd>
                    <kbd class="kbd">"Cmd"</kbd>
                    <kbd class="kbd">"Space"</kbd>
                    <kbd class="kbd">"Enter"</kbd>
                    <kbd class="kbd">"Esc"</kbd>
                </div>

                <h2 class="text-xl font-semibold">"Arrow Keys"</h2>
                <div class="flex gap-2">
                    <kbd class="kbd">"▲"</kbd>
                    <kbd class="kbd">"▼"</kbd>
                    <kbd class="kbd">"◀︎"</kbd>
                    <kbd class="kbd">"▶︎"</kbd>
                </div>

                <h2 class="text-xl font-semibold">"Function Keys"</h2>
                <div class="flex gap-2 flex-wrap">
                    <kbd class="kbd">"F1"</kbd>
                    <kbd class="kbd">"F2"</kbd>
                    <kbd class="kbd">"F3"</kbd>
                    <kbd class="kbd">"F4"</kbd>
                    <kbd class="kbd">"F5"</kbd>
                    <kbd class="kbd">"F6"</kbd>
                    <kbd class="kbd">"F7"</kbd>
                    <kbd class="kbd">"F8"</kbd>
                    <kbd class="kbd">"F9"</kbd>
                    <kbd class="kbd">"F10"</kbd>
                    <kbd class="kbd">"F11"</kbd>
                    <kbd class="kbd">"F12"</kbd>
                </div>

                <h2 class="text-xl font-semibold">"Number Keys"</h2>
                <div class="flex gap-2 flex-wrap">
                    <kbd class="kbd">"1"</kbd>
                    <kbd class="kbd">"2"</kbd>
                    <kbd class="kbd">"3"</kbd>
                    <kbd class="kbd">"4"</kbd>
                    <kbd class="kbd">"5"</kbd>
                    <kbd class="kbd">"6"</kbd>
                    <kbd class="kbd">"7"</kbd>
                    <kbd class="kbd">"8"</kbd>
                    <kbd class="kbd">"9"</kbd>
                    <kbd class="kbd">"0"</kbd>
                </div>

                <h2 class="text-xl font-semibold">"Key Combinations"</h2>
                <div class="space-y-3">
                    <div class="flex items-center gap-2">
                        <span class="text-sm">"Copy:"</span>
                        <kbd class="kbd kbd-sm">"Ctrl"</kbd>
                        <span>"+"</span>
                        <kbd class="kbd kbd-sm">"C"</kbd>
                    </div>
                    <div class="flex items-center gap-2">
                        <span class="text-sm">"Paste:"</span>
                        <kbd class="kbd kbd-sm">"Ctrl"</kbd>
                        <span>"+"</span>
                        <kbd class="kbd kbd-sm">"V"</kbd>
                    </div>
                    <div class="flex items-center gap-2">
                        <span class="text-sm">"Select All:"</span>
                        <kbd class="kbd kbd-sm">"Ctrl"</kbd>
                        <span>"+"</span>
                        <kbd class="kbd kbd-sm">"A"</kbd>
                    </div>
                    <div class="flex items-center gap-2">
                        <span class="text-sm">"Undo:"</span>
                        <kbd class="kbd kbd-sm">"Ctrl"</kbd>
                        <span>"+"</span>
                        <kbd class="kbd kbd-sm">"Z"</kbd>
                    </div>
                    <div class="flex items-center gap-2">
                        <span class="text-sm">"Redo:"</span>
                        <kbd class="kbd kbd-sm">"Ctrl"</kbd>
                        <span>"+"</span>
                        <kbd class="kbd kbd-sm">"Y"</kbd>
                    </div>
                </div>

                <h2 class="text-xl font-semibold">"Sizes"</h2>
                <div class="flex items-center gap-2">
                    <kbd class="kbd kbd-xs">"XS"</kbd>
                    <kbd class="kbd kbd-sm">"SM"</kbd>
                    <kbd class="kbd kbd-md">"MD"</kbd>
                    <kbd class="kbd kbd-lg">"LG"</kbd>
                </div>

                <h2 class="text-xl font-semibold">"In Documentation"</h2>
                <Card class="bg-base-100 shadow-xl">
                    <CardBody>
                        <h2 class="card-title">"Keyboard Shortcuts"</h2>
                        <div class="space-y-3">
                            <div class="flex justify-between items-center">
                                <span>"Open search"</span>
                                <div class="flex gap-1">
                                    <kbd class="kbd kbd-sm">"Ctrl"</kbd>
                                    <span>"+"</span>
                                    <kbd class="kbd kbd-sm">"K"</kbd>
                                </div>
                            </div>
                            <div class="flex justify-between items-center">
                                <span>"Toggle sidebar"</span>
                                <kbd class="kbd kbd-sm">"["</kbd>
                            </div>
                            <div class="flex justify-between items-center">
                                <span>"Go to next page"</span>
                                <div class="flex gap-1">
                                    <kbd class="kbd kbd-sm">"Ctrl"</kbd>
                                    <span>"+"</span>
                                    <kbd class="kbd kbd-sm">"→"</kbd>
                                </div>
                            </div>
                            <div class="flex justify-between items-center">
                                <span>"Go to previous page"</span>
                                <div class="flex gap-1">
                                    <kbd class="kbd kbd-sm">"Ctrl"</kbd>
                                    <span>"+"</span>
                                    <kbd class="kbd kbd-sm">"←"</kbd>
                                </div>
                            </div>
                        </div>
                    </CardBody>
                </Card>

                <h2 class="text-xl font-semibold">"Platform Specific"</h2>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    <Card class="bg-base-100 shadow-lg">
                        <CardBody>
                            <h3 class="card-title text-lg">"Windows/Linux"</h3>
                            <div class="space-y-2">
                                <div class="flex justify-between">
                                    <span class="text-sm">"New Tab:"</span>
                                    <div class="flex gap-1">
                                        <kbd class="kbd kbd-xs">"Ctrl"</kbd>
                                        <span>"+"</span>
                                        <kbd class="kbd kbd-xs">"T"</kbd>
                                    </div>
                                </div>
                                <div class="flex justify-between">
                                    <span class="text-sm">"Close Tab:"</span>
                                    <div class="flex gap-1">
                                        <kbd class="kbd kbd-xs">"Ctrl"</kbd>
                                        <span>"+"</span>
                                        <kbd class="kbd kbd-xs">"W"</kbd>
                                    </div>
                                </div>
                                <div class="flex justify-between">
                                    <span class="text-sm">"Refresh:"</span>
                                    <kbd class="kbd kbd-xs">"F5"</kbd>
                                </div>
                            </div>
                        </CardBody>
                    </Card>

                    <Card class="bg-base-100 shadow-lg">
                        <CardBody>
                            <h3 class="card-title text-lg">"macOS"</h3>
                            <div class="space-y-2">
                                <div class="flex justify-between">
                                    <span class="text-sm">"New Tab:"</span>
                                    <div class="flex gap-1">
                                        <kbd class="kbd kbd-xs">"⌘"</kbd>
                                        <span>"+"</span>
                                        <kbd class="kbd kbd-xs">"T"</kbd>
                                    </div>
                                </div>
                                <div class="flex justify-between">
                                    <span class="text-sm">"Close Tab:"</span>
                                    <div class="flex gap-1">
                                        <kbd class="kbd kbd-xs">"⌘"</kbd>
                                        <span>"+"</span>
                                        <kbd class="kbd kbd-xs">"W"</kbd>
                                    </div>
                                </div>
                                <div class="flex justify-between">
                                    <span class="text-sm">"Refresh:"</span>
                                    <div class="flex gap-1">
                                        <kbd class="kbd kbd-xs">"⌘"</kbd>
                                        <span>"+"</span>
                                        <kbd class="kbd kbd-xs">"R"</kbd>
                                    </div>
                                </div>
                            </div>
                        </CardBody>
                    </Card>
                </div>

                <h2 class="text-xl font-semibold">"Game Controls"</h2>
                <Card class="bg-base-100 shadow-xl">
                    <CardBody>
                        <h2 class="card-title">"FPS Game Controls"</h2>
                        <div class="grid grid-cols-2 gap-4">
                            <div>
                                <h4 class="font-semibold mb-2">"Movement"</h4>
                                <div class="space-y-1">
                                    <div class="flex justify-between text-sm">
                                        <span>"Forward:"</span>
                                        <kbd class="kbd kbd-xs">"W"</kbd>
                                    </div>
                                    <div class="flex justify-between text-sm">
                                        <span>"Back:"</span>
                                        <kbd class="kbd kbd-xs">"S"</kbd>
                                    </div>
                                    <div class="flex justify-between text-sm">
                                        <span>"Left:"</span>
                                        <kbd class="kbd kbd-xs">"A"</kbd>
                                    </div>
                                    <div class="flex justify-between text-sm">
                                        <span>"Right:"</span>
                                        <kbd class="kbd kbd-xs">"D"</kbd>
                                    </div>
                                </div>
                            </div>
                            <div>
                                <h4 class="font-semibold mb-2">"Actions"</h4>
                                <div class="space-y-1">
                                    <div class="flex justify-between text-sm">
                                        <span>"Jump:"</span>
                                        <kbd class="kbd kbd-xs">"Space"</kbd>
                                    </div>
                                    <div class="flex justify-between text-sm">
                                        <span>"Crouch:"</span>
                                        <kbd class="kbd kbd-xs">"Ctrl"</kbd>
                                    </div>
                                    <div class="flex justify-between text-sm">
                                        <span>"Reload:"</span>
                                        <kbd class="kbd kbd-xs">"R"</kbd>
                                    </div>
                                    <div class="flex justify-between text-sm">
                                        <span>"Use:"</span>
                                        <kbd class="kbd kbd-xs">"E"</kbd>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </CardBody>
                </Card>
            </div>
        </div>
    }
}
