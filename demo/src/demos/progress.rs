use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn ProgressDemo() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <h1 class="text-3xl font-bold">"Progress"</h1>
            <p class="text-base-content/70">
                "Progress bars show the progress of a task or show the loading state"
            </p>

            <div class="space-y-4">
                <h2 class="text-xl font-semibold">"Basic Progress"</h2>
                <Progress value=70.0 max=100.0 class="w-56" />

                <h2 class="text-xl font-semibold">"Colors"</h2>
                <div class="space-y-2">
                    <Progress value=70.0 max=100.0 class="w-56" />
                    <Progress
                        color=Signal::derive(move || ProgressColor::Primary)
                        value=70.0
                        max=100.0
                        class="w-56"
                    />
                    <Progress
                        color=Signal::derive(move || ProgressColor::Secondary)
                        value=70.0
                        max=100.0
                        class="w-56"
                    />
                    <Progress
                        color=Signal::derive(move || ProgressColor::Accent)
                        value=70.0
                        max=100.0
                        class="w-56"
                    />
                    <Progress
                        color=Signal::derive(move || ProgressColor::Info)
                        value=70.0
                        max=100.0
                        class="w-56"
                    />
                    <Progress
                        color=Signal::derive(move || ProgressColor::Success)
                        value=70.0
                        max=100.0
                        class="w-56"
                    />
                    <Progress
                        color=Signal::derive(move || ProgressColor::Warning)
                        value=70.0
                        max=100.0
                        class="w-56"
                    />
                    <Progress
                        color=Signal::derive(move || ProgressColor::Error)
                        value=70.0
                        max=100.0
                        class="w-56"
                    />
                </div>

                <h2 class="text-xl font-semibold">"Indeterminate Progress"</h2>
                <Progress class="w-56" />
            </div>
        </div>
    }
}