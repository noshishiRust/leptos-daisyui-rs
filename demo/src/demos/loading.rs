use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn LoadingDemo() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <h1 class="text-3xl font-bold">"Loading"</h1>
            <p class="text-base-content/70">
                "Loading indicators show that something is being processed"
            </p>

            <div class="space-y-4">
                <h2 class="text-xl font-semibold">"Basic Loading"</h2>
                <div class="flex gap-4">
                    <Loading />
                    <Loading class="loading-spinner" />
                    <Loading class="loading-dots" />
                    <Loading class="loading-ring" />
                    <Loading class="loading-ball" />
                    <Loading class="loading-bars" />
                    <Loading class="loading-infinity" />
                </div>

                <h2 class="text-xl font-semibold">"Sizes"</h2>
                <div class="flex items-center gap-4">
                    <Loading size=LoadingSize::Xs />
                    <Loading size=LoadingSize::Sm />
                    <Loading size=LoadingSize::Md />
                    <Loading size=LoadingSize::Lg />
                </div>

                <h2 class="text-xl font-semibold">"Colors"</h2>
                <div class="flex gap-4">
                    <Loading color=LoadingColor::Primary />
                    <Loading color=LoadingColor::Secondary />
                    <Loading color=LoadingColor::Accent />
                    <Loading color=LoadingColor::Info />
                    <Loading color=LoadingColor::Success />
                    <Loading color=LoadingColor::Warning />
                    <Loading color=LoadingColor::Error />
                </div>
            </div>
        </div>
    }
}