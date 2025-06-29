use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn ButtonDemo() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <h1 class="text-3xl font-bold">"Button"</h1>
            <p class="text-base-content/70">
                "Buttons allow users to take actions and make choices"
            </p>

            <div class="space-y-4">
                <h2 class="text-xl font-semibold">"Colors"</h2>
                <div class="flex flex-wrap gap-2">
                    <Button>"Default"</Button>
                    <Button class:btn-neutral=true on:click=move |_| log::info!("Button clicked")>
                        <span>"Neutral"</span>
                    </Button>
                    <Button color=Signal::derive(move || ButtonColor::Primary)>"Primary"</Button>
                    <Button color=Signal::derive(move || {
                        ButtonColor::Secondary
                    })>"Secondary"</Button>
                    <Button color=Signal::derive(move || ButtonColor::Accent)>"Accent"</Button>
                    <Button color=Signal::derive(move || ButtonColor::Info)>"Info"</Button>
                    <Button color=Signal::derive(move || ButtonColor::Success)>"Success"</Button>
                    <Button color=Signal::derive(move || ButtonColor::Warning)>"Warning"</Button>
                    <Button color=Signal::derive(move || ButtonColor::Error)>"Error"</Button>
                </div>

                <h2 class="text-xl font-semibold">"Sizes"</h2>
                <div class="flex items-center gap-2">
                    <Button size=Signal::derive(move || ButtonSize::Xs)>"XS"</Button>
                    <Button size=Signal::derive(move || ButtonSize::Sm)>"SM"</Button>
                    <Button size=Signal::derive(move || ButtonSize::Md)>"MD"</Button>
                    <Button size=Signal::derive(move || ButtonSize::Lg)>"LG"</Button>
                    <Button size=Signal::derive(move || ButtonSize::Xl)>"XL"</Button>
                </div>

                <h2 class="text-xl font-semibold">"Styles"</h2>
                <div class="flex flex-wrap gap-2">
                    <Button color=Signal::derive(move || ButtonColor::Primary)>"Default"</Button>
                    <Button
                        style=Signal::derive(move || ButtonStyle::Outline)
                        color=Signal::derive(move || ButtonColor::Primary)
                    >
                        "Outline"
                    </Button>
                    <Button
                        style=Signal::derive(move || ButtonStyle::Ghost)
                        color=Signal::derive(move || ButtonColor::Primary)
                    >
                        "Ghost"
                    </Button>
                    <Button
                        style=Signal::derive(move || ButtonStyle::Link)
                        color=Signal::derive(move || ButtonColor::Primary)
                    >
                        "Link"
                    </Button>
                    <Button
                        style=Signal::derive(move || ButtonStyle::Soft)
                        color=Signal::derive(move || ButtonColor::Primary)
                    >
                        "Soft"
                    </Button>
                </div>

                <h2 class="text-xl font-semibold">"States"</h2>
                <div class="flex gap-2">
                    <Button color=Signal::derive(move || ButtonColor::Primary)>"Normal"</Button>
                    <Button
                        color=Signal::derive(move || ButtonColor::Primary)
                        disabled=Signal::derive(move || true)
                    >
                        "Disabled"
                    </Button>
                    <Button color=Signal::derive(move || ButtonColor::Primary) class="loading">
                        "Loading"
                    </Button>
                </div>

                <h2 class="text-xl font-semibold">"Shapes"</h2>
                <div class="flex items-center gap-2">
                    <Button color=Signal::derive(move || ButtonColor::Primary)>"Normal"</Button>
                    <Button color=Signal::derive(move || ButtonColor::Primary) class="btn-wide">
                        "Wide"
                    </Button>
                    <Button color=Signal::derive(move || ButtonColor::Primary) class="btn-square">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="h-5 w-5"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke="currentColor"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M6 18L18 6M6 6l12 12"
                            />
                        </svg>
                    </Button>
                    <Button color=Signal::derive(move || ButtonColor::Primary) class="btn-circle">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="h-5 w-5"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke="currentColor"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M6 18L18 6M6 6l12 12"
                            />
                        </svg>
                    </Button>
                </div>
            </div>
        </div>
    }
}