use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn AlertDemo() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <h1 class="text-3xl font-bold">"Alert"</h1>
            <p class="text-base-content/70">"Alert informs users about important events"</p>

            <div class="space-y-4">
                <h2 class="text-xl font-semibold">"Different Colors"</h2>
                <div class="space-y-3">
                    <Alert color=AlertColor::Info>
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            fill="none"
                            viewBox="0 0 24 24"
                            class="stroke-current shrink-0 w-6 h-6"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                            ></path>
                        </svg>
                        <span>"Info alert with icon"</span>
                    </Alert>

                    <Alert color=AlertColor::Success>
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="stroke-current shrink-0 h-6 w-6"
                            fill="none"
                            viewBox="0 0 24 24"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
                            />
                        </svg>
                        <span>"Success alert!"</span>
                    </Alert>

                    <Alert color=AlertColor::Warning>
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="stroke-current shrink-0 h-6 w-6"
                            fill="none"
                            viewBox="0 0 24 24"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 15c-.77.833.192 2.5 1.732 2.5z"
                            />
                        </svg>
                        <span>"Warning alert!"</span>
                    </Alert>

                    <Alert color=AlertColor::Error>
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="stroke-current shrink-0 h-6 w-6"
                            fill="none"
                            viewBox="0 0 24 24"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
                            />
                        </svg>
                        <span>"Error alert!"</span>
                    </Alert>
                </div>

                <h2 class="text-xl font-semibold">"Different Styles"</h2>
                <div class="space-y-3">
                    <Alert style=AlertStyle::Outline color=AlertColor::Info>
                        <span>"Outline style alert"</span>
                    </Alert>

                    <Alert style=AlertStyle::Soft color=AlertColor::Info>
                        <span>"Soft style alert"</span>
                    </Alert>
                </div>
            </div>
        </div>
    }
}