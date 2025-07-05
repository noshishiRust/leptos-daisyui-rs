use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn DividerDemo() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <h1 class="text-3xl font-bold">"Divider"</h1>
            <p class="text-base-content/70">
                "Divider visually separates content in lists or groups"
            </p>

            <div class="space-y-4">
                <h2 class="text-xl font-semibold">"Basic Divider"</h2>
                <div class="flex w-full flex-col">
                    <div class="grid h-20 flex-grow card bg-base-300 rounded-box place-items-center">
                        "Content above"
                    </div>
                    <div class="divider"></div>
                    <div class="grid h-20 flex-grow card bg-base-300 rounded-box place-items-center">
                        "Content below"
                    </div>
                </div>

                <h2 class="text-xl font-semibold">"Divider with Text"</h2>
                <div class="flex w-full flex-col">
                    <div class="grid h-20 flex-grow card bg-base-300 rounded-box place-items-center">
                        "Content above"
                    </div>
                    <div class="divider">"OR"</div>
                    <div class="grid h-20 flex-grow card bg-base-300 rounded-box place-items-center">
                        "Content below"
                    </div>
                </div>

                <h2 class="text-xl font-semibold">"Vertical Divider"</h2>
                <div class="flex w-full">
                    <div class="grid h-20 flex-grow card bg-base-300 rounded-box place-items-center">
                        "Content left"
                    </div>
                    <div class="divider divider-horizontal"></div>
                    <div class="grid h-20 flex-grow card bg-base-300 rounded-box place-items-center">
                        "Content right"
                    </div>
                </div>

                <h2 class="text-xl font-semibold">"Vertical Divider with Text"</h2>
                <div class="flex w-full">
                    <div class="grid h-20 flex-grow card bg-base-300 rounded-box place-items-center">
                        "Content left"
                    </div>
                    <div class="divider divider-horizontal">"OR"</div>
                    <div class="grid h-20 flex-grow card bg-base-300 rounded-box place-items-center">
                        "Content right"
                    </div>
                </div>

                <h2 class="text-xl font-semibold">"Divider Colors"</h2>
                <div class="space-y-4">
                    <div class="flex w-full flex-col">
                        <div class="grid h-16 flex-grow card bg-base-300 rounded-box place-items-center">
                            "Section 1"
                        </div>
                        <div class="divider divider-primary">"PRIMARY"</div>
                        <div class="grid h-16 flex-grow card bg-base-300 rounded-box place-items-center">
                            "Section 2"
                        </div>
                    </div>

                    <div class="flex w-full flex-col">
                        <div class="grid h-16 flex-grow card bg-base-300 rounded-box place-items-center">
                            "Section 1"
                        </div>
                        <div class="divider divider-secondary">"SECONDARY"</div>
                        <div class="grid h-16 flex-grow card bg-base-300 rounded-box place-items-center">
                            "Section 2"
                        </div>
                    </div>

                    <div class="flex w-full flex-col">
                        <div class="grid h-16 flex-grow card bg-base-300 rounded-box place-items-center">
                            "Section 1"
                        </div>
                        <div class="divider divider-accent">"ACCENT"</div>
                        <div class="grid h-16 flex-grow card bg-base-300 rounded-box place-items-center">
                            "Section 2"
                        </div>
                    </div>
                </div>

                <h2 class="text-xl font-semibold">"Form Section Dividers"</h2>
                <Card class="bg-base-100 shadow-xl">
                    <CardBody>
                        <h2 class="card-title">"User Registration"</h2>

                        <div class="space-y-4">
                            <div class="form-control">
                                <label class="label">
                                    <span class="label-text">"Full Name"</span>
                                </label>
                                <Input placeholder="Enter your full name" class="w-full" />
                            </div>

                            <div class="form-control">
                                <label class="label">
                                    <span class="label-text">"Email"</span>
                                </label>
                                <Input placeholder="Enter your email" class="w-full" />
                            </div>
                        </div>

                        <div class="divider">"Account Details"</div>

                        <div class="space-y-4">
                            <div class="form-control">
                                <label class="label">
                                    <span class="label-text">"Username"</span>
                                </label>
                                <Input placeholder="Choose a username" class="w-full" />
                            </div>

                            <div class="form-control">
                                <label class="label">
                                    <span class="label-text">"Password"</span>
                                </label>
                                <input
                                    type="password"
                                    placeholder="Create a password"
                                    class="input input-bordered w-full"
                                />
                            </div>
                        </div>

                        <div class="divider">"Preferences"</div>

                        <div class="space-y-2">
                            <div class="form-control">
                                <label class="label cursor-pointer justify-start gap-2">
                                    <Checkbox />
                                    <span class="label-text">"Send me newsletters"</span>
                                </label>
                            </div>
                            <div class="form-control">
                                <label class="label cursor-pointer justify-start gap-2">
                                    <Checkbox />
                                    <span class="label-text">"Enable notifications"</span>
                                </label>
                            </div>
                        </div>

                        <div class="card-actions justify-end mt-6">
                            <Button color=ButtonColor::Primary>"Register"</Button>
                        </div>
                    </CardBody>
                </Card>

                <h2 class="text-xl font-semibold">"Content Layout with Dividers"</h2>
                <div class="space-y-0">
                    <div class="p-4 bg-base-200 rounded-t-lg">
                        <h3 class="font-semibold">"Article Title"</h3>
                        <p class="text-sm text-base-content/70">
                            "This is the introduction section of the article."
                        </p>
                    </div>

                    <div class="divider my-0">"Main Content"</div>

                    <div class="p-4 bg-base-100">
                        <p class="text-sm">
                            "This is the main body of the article with detailed information and explanations."
                        </p>
                    </div>

                    <div class="divider my-0">"Comments"</div>

                    <div class="p-4 bg-base-200 rounded-b-lg">
                        <p class="text-sm text-base-content/70">
                            "User comments and discussions would appear here."
                        </p>
                    </div>
                </div>
            </div>
        </div>
    }
}