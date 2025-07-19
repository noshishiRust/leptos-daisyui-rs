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
                    <Button color=ButtonColor::Primary>"Primary"</Button>
                    <Button color=ButtonColor::Secondary>"Secondary"</Button>
                    <Button color=ButtonColor::Accent>"Accent"</Button>
                    <Button color=ButtonColor::Info>"Info"</Button>
                    <Button color=ButtonColor::Success>"Success"</Button>
                    <Button color=ButtonColor::Warning>"Warning"</Button>
                    <Button color=ButtonColor::Error>"Error"</Button>
                </div>

                <h2 class="text-xl font-semibold">"Sizes"</h2>
                <div class="flex items-center gap-2">
                    <Button size=ButtonSize::Xs>"XS"</Button>
                    <Button size=ButtonSize::Sm>"SM"</Button>
                    <Button size=ButtonSize::Md>"MD"</Button>
                    <Button size=ButtonSize::Lg>"LG"</Button>
                    <Button size=ButtonSize::Xl>"XL"</Button>
                </div>

                <h2 class="text-xl font-semibold">"Styles"</h2>
                <div class="flex flex-wrap gap-2">
                    <Button style=ButtonStyle::Outline>"Outline"</Button>
                    <Button style=ButtonStyle::Ghost>"Ghost"</Button>
                    <Button style=ButtonStyle::Link>"Link"</Button>
                    <Button style=ButtonStyle::Soft>"Soft"</Button>
                </div>

                <h2 class="text-xl font-semibold">"States"</h2>
                <div class="flex gap-2">
                    <Button color=ButtonColor::Primary>"Normal"</Button>
                    <Button color=ButtonColor::Primary disabled=true>
                        "Disabled"
                    </Button>
                    <Button color=ButtonColor::Primary disabled=true>
                        <Loading />
                        "Loading"
                    </Button>
                </div>

                <h2 class="text-xl font-semibold">"Shapes"</h2>
                <div class="flex items-center gap-2">
                    <Button color=ButtonColor::Primary shape=ButtonShape::Wide>
                        "Wide"
                    </Button>
                    <Button color=ButtonColor::Primary shape=ButtonShape::Square>
                        "Square"
                    </Button>
                    <Button color=ButtonColor::Primary shape=ButtonShape::Circle>
                        "Circle"
                    </Button>
                    <Button color=ButtonColor::Primary shape=ButtonShape::Block>
                        "Block"
                    </Button>
                </div>
            </div>
        </div>
    }
}
