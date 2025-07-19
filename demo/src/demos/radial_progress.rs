use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn RadialProgressDemo() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <h1 class="text-3xl font-bold">"Radial Progress"</h1>
            <p class="text-base-content/70">
                "Radial progress shows the progress in a circle format"
            </p>

            <div class="space-y-4">
                <h2 class="text-xl font-semibold">"Basic Radial Progress"</h2>
                <div class="flex flex-wrap gap-4">
                    <RadialProgress value=0.0 class="text-primary" />
                    <RadialProgress value=20.0 class="text-primary" />
                    <RadialProgress value=60.0 class="text-primary" />
                    <RadialProgress value=80.0 class="text-primary" />
                    <RadialProgress value=100.0 class="text-primary" />
                </div>

                <h2 class="text-xl font-semibold">"With Text"</h2>
                <div class="flex flex-wrap gap-4">
                    <RadialProgress value=70.0 class="text-primary">
                        "70%"
                    </RadialProgress>
                    <RadialProgress value=85.0 class="text-secondary">
                        "85%"
                    </RadialProgress>
                    <RadialProgress value=90.0 class="text-accent">
                        "90%"
                    </RadialProgress>
                </div>

                <h2 class="text-xl font-semibold">"Colors"</h2>
                <div class="flex flex-wrap gap-4">
                    <RadialProgress value=70.0 color=RadialProgressColor::Primary>
                        "70%"
                    </RadialProgress>
                    <RadialProgress value=70.0 color=RadialProgressColor::Secondary>
                        "70%"
                    </RadialProgress>
                    <RadialProgress value=70.0 color=RadialProgressColor::Accent>
                        "70%"
                    </RadialProgress>
                    <RadialProgress value=70.0 color=RadialProgressColor::Info>
                        "70%"
                    </RadialProgress>
                    <RadialProgress value=70.0 color=RadialProgressColor::Success>
                        "70%"
                    </RadialProgress>
                    <RadialProgress value=70.0 color=RadialProgressColor::Warning>
                        "70%"
                    </RadialProgress>
                    <RadialProgress value=70.0 color=RadialProgressColor::Error>
                        "70%"
                    </RadialProgress>
                </div>

                <h2 class="text-xl font-semibold">"Sizes"</h2>
                <div class="flex flex-wrap items-center gap-4">
                    <RadialProgress value=70.0 size=RadialProgressSize::Xs class="text-primary">
                        "70%"
                    </RadialProgress>
                    <RadialProgress value=70.0 size=RadialProgressSize::Sm class="text-primary">
                        "70%"
                    </RadialProgress>
                    <RadialProgress value=70.0 size=RadialProgressSize::Md class="text-primary">
                        "70%"
                    </RadialProgress>
                    <RadialProgress value=70.0 size=RadialProgressSize::Lg class="text-primary">
                        "70%"
                    </RadialProgress>
                    <RadialProgress value=70.0 size=RadialProgressSize::Xl class="text-primary">
                        "70%"
                    </RadialProgress>
                </div>

                <h2 class="text-xl font-semibold">"With Different Values"</h2>
                <div class="flex flex-wrap gap-4">
                    <RadialProgress value=25.0 class="text-error">
                        "25%"
                    </RadialProgress>
                    <RadialProgress value=50.0 class="text-warning">
                        "50%"
                    </RadialProgress>
                    <RadialProgress value=75.0 class="text-info">
                        "75%"
                    </RadialProgress>
                    <RadialProgress value=100.0 class="text-success">
                        "100%"
                    </RadialProgress>
                </div>

                <h2 class="text-xl font-semibold">"With Custom Content"</h2>
                <div class="flex flex-wrap gap-4">
                    <RadialProgress value=75.0 class="text-primary">
                        <div class="text-xs">"3/4"</div>
                    </RadialProgress>
                    <RadialProgress value=66.0 class="text-secondary">
                        <div class="text-xs">"2 of 3"</div>
                    </RadialProgress>
                    <RadialProgress value=88.0 class="text-accent">
                        <div class="text-xs">"8.8"</div>
                    </RadialProgress>
                </div>
            </div>
        </div>
    }
}