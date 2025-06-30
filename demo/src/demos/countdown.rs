use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn CountdownDemo() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <h1 class="text-3xl font-bold">"Countdown"</h1>
            <p class="text-base-content/70">
                "Countdown component shows a timer that counts down to zero"
            </p>

            <div class="space-y-4">
                <h2 class="text-xl font-semibold">"Basic Countdown"</h2>
                <div class="grid grid-flow-col gap-5 text-center auto-cols-max">
                    <div class="flex flex-col">
                        <span class="countdown font-mono text-5xl">
                            <span style="--value:15;"></span>
                        </span>
                        "days"
                    </div>
                    <div class="flex flex-col">
                        <span class="countdown font-mono text-5xl">
                            <span style="--value:10;"></span>
                        </span>
                        "hours"
                    </div>
                    <div class="flex flex-col">
                        <span class="countdown font-mono text-5xl">
                            <span style="--value:24;"></span>
                        </span>
                        "min"
                    </div>
                    <div class="flex flex-col">
                        <span class="countdown font-mono text-5xl">
                            <span style="--value:47;"></span>
                        </span>
                        "sec"
                    </div>
                </div>

                <h2 class="text-xl font-semibold">"Large Countdown"</h2>
                <div class="grid grid-flow-col gap-5 text-center auto-cols-max">
                    <div class="flex flex-col p-2 bg-neutral rounded-box text-neutral-content">
                        <span class="countdown font-mono text-5xl">
                            <span style="--value:99;"></span>
                        </span>
                        "days"
                    </div>
                    <div class="flex flex-col p-2 bg-neutral rounded-box text-neutral-content">
                        <span class="countdown font-mono text-5xl">
                            <span style="--value:23;"></span>
                        </span>
                        "hours"
                    </div>
                    <div class="flex flex-col p-2 bg-neutral rounded-box text-neutral-content">
                        <span class="countdown font-mono text-5xl">
                            <span style="--value:59;"></span>
                        </span>
                        "min"
                    </div>
                    <div class="flex flex-col p-2 bg-neutral rounded-box text-neutral-content">
                        <span class="countdown font-mono text-5xl">
                            <span style="--value:35;"></span>
                        </span>
                        "sec"
                    </div>
                </div>

                <h2 class="text-xl font-semibold">"Colored Countdown"</h2>
                <div class="grid grid-flow-col gap-5 text-center auto-cols-max">
                    <div class="flex flex-col p-2 bg-primary rounded-box text-primary-content">
                        <span class="countdown font-mono text-5xl">
                            <span style="--value:30;"></span>
                        </span>
                        "days"
                    </div>
                    <div class="flex flex-col p-2 bg-secondary rounded-box text-secondary-content">
                        <span class="countdown font-mono text-5xl">
                            <span style="--value:12;"></span>
                        </span>
                        "hours"
                    </div>
                    <div class="flex flex-col p-2 bg-accent rounded-box text-accent-content">
                        <span class="countdown font-mono text-5xl">
                            <span style="--value:45;"></span>
                        </span>
                        "min"
                    </div>
                    <div class="flex flex-col p-2 bg-info rounded-box text-info-content">
                        <span class="countdown font-mono text-5xl">
                            <span style="--value:08;"></span>
                        </span>
                        "sec"
                    </div>
                </div>

                <h2 class="text-xl font-semibold">"Small Countdown"</h2>
                <div class="grid grid-flow-col gap-2 text-center auto-cols-max">
                    <div class="flex flex-col">
                        <span class="countdown font-mono text-2xl">
                            <span style="--value:7;"></span>
                        </span>
                        <span class="text-xs">"days"</span>
                    </div>
                    <div class="flex flex-col">
                        <span class="countdown font-mono text-2xl">
                            <span style="--value:18;"></span>
                        </span>
                        <span class="text-xs">"hours"</span>
                    </div>
                    <div class="flex flex-col">
                        <span class="countdown font-mono text-2xl">
                            <span style="--value:32;"></span>
                        </span>
                        <span class="text-xs">"min"</span>
                    </div>
                    <div class="flex flex-col">
                        <span class="countdown font-mono text-2xl">
                            <span style="--value:55;"></span>
                        </span>
                        <span class="text-xs">"sec"</span>
                    </div>
                </div>

                <h2 class="text-xl font-semibold">"Event Countdown Cards"</h2>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    <Card class="bg-gradient-to-r from-primary to-secondary text-primary-content">
                        <CardBody class="text-center">
                            <h2 class="card-title justify-center">"Product Launch"</h2>
                            <p>"Our exciting new product launches in:"</p>
                            <div class="grid grid-flow-col gap-3 text-center auto-cols-max justify-center mt-4">
                                <div class="flex flex-col p-2 bg-base-100 rounded-box text-base-content">
                                    <span class="countdown font-mono text-3xl">
                                        <span style="--value:45;"></span>
                                    </span>
                                    "days"
                                </div>
                                <div class="flex flex-col p-2 bg-base-100 rounded-box text-base-content">
                                    <span class="countdown font-mono text-3xl">
                                        <span style="--value:8;"></span>
                                    </span>
                                    "hours"
                                </div>
                                <div class="flex flex-col p-2 bg-base-100 rounded-box text-base-content">
                                    <span class="countdown font-mono text-3xl">
                                        <span style="--value:27;"></span>
                                    </span>
                                    "min"
                                </div>
                            </div>
                            <div class="card-actions justify-center mt-4">
                                <Button color=Signal::derive(|| {
                                    ButtonColor::Accent
                                })>"Notify Me"</Button>
                            </div>
                        </CardBody>
                    </Card>

                    <Card class="bg-gradient-to-r from-success to-accent text-success-content">
                        <CardBody class="text-center">
                            <h2 class="card-title justify-center">"Summer Sale"</h2>
                            <p>"50% off everything! Sale ends in:"</p>
                            <div class="grid grid-flow-col gap-3 text-center auto-cols-max justify-center mt-4">
                                <div class="flex flex-col p-2 bg-base-100 rounded-box text-base-content">
                                    <span class="countdown font-mono text-3xl">
                                        <span style="--value:2;"></span>
                                    </span>
                                    "days"
                                </div>
                                <div class="flex flex-col p-2 bg-base-100 rounded-box text-base-content">
                                    <span class="countdown font-mono text-3xl">
                                        <span style="--value:14;"></span>
                                    </span>
                                    "hours"
                                </div>
                                <div class="flex flex-col p-2 bg-base-100 rounded-box text-base-content">
                                    <span class="countdown font-mono text-3xl">
                                        <span style="--value:35;"></span>
                                    </span>
                                    "min"
                                </div>
                            </div>
                            <div class="card-actions justify-center mt-4">
                                <Button color=Signal::derive(|| {
                                    ButtonColor::Warning
                                })>"Shop Now"</Button>
                            </div>
                        </CardBody>
                    </Card>
                </div>

                <h2 class="text-xl font-semibold">"Live Event Countdown"</h2>
                <Card class="bg-base-200">
                    <CardBody>
                        <div class="text-center">
                            <div class="flex items-center justify-center gap-2 mb-4">
                                <div class="w-3 h-3 bg-error rounded-full animate-pulse"></div>
                                <span class="text-lg font-semibold">
                                    "LIVE EVENT STARTING SOON"
                                </span>
                            </div>

                            <h3 class="text-2xl font-bold mb-2">"Web Development Masterclass"</h3>
                            <p class="text-base-content/70 mb-6">
                                "Join us for an intensive 3-hour coding session"
                            </p>

                            <div class="grid grid-flow-col gap-4 text-center auto-cols-max justify-center">
                                <div class="flex flex-col p-4 bg-error rounded-box text-error-content">
                                    <span class="countdown font-mono text-4xl">
                                        <span style="--value:0;"></span>
                                    </span>
                                    "hours"
                                </div>
                                <div class="flex flex-col p-4 bg-warning rounded-box text-warning-content">
                                    <span class="countdown font-mono text-4xl">
                                        <span style="--value:15;"></span>
                                    </span>
                                    "min"
                                </div>
                                <div class="flex flex-col p-4 bg-success rounded-box text-success-content">
                                    <span class="countdown font-mono text-4xl">
                                        <span style="--value:30;"></span>
                                    </span>
                                    "sec"
                                </div>
                            </div>

                            <div class="mt-6 flex gap-2 justify-center">
                                <Button color=Signal::derive(|| {
                                    ButtonColor::Primary
                                })>"Join Live Stream"</Button>
                                <Button style=Signal::derive(|| {
                                    ButtonStyle::Outline
                                })>"Set Reminder"</Button>
                            </div>
                        </div>
                    </CardBody>
                </Card>

                <h2 class="text-xl font-semibold">"Birthday Countdown"</h2>
                <Card class="bg-gradient-to-br from-pink-500 to-purple-600 text-white">
                    <CardBody class="text-center">
                        <div class="text-6xl mb-4">"🎂"</div>
                        <h2 class="card-title justify-center text-2xl">"Sarah's Birthday"</h2>
                        <p class="opacity-90 mb-6">"The big day is almost here!"</p>

                        <div class="grid grid-flow-col gap-3 text-center auto-cols-max justify-center">
                            <div class="flex flex-col p-3 bg-white/20 rounded-box backdrop-blur">
                                <span class="countdown font-mono text-3xl">
                                    <span style="--value:12;"></span>
                                </span>
                                "days"
                            </div>
                            <div class="flex flex-col p-3 bg-white/20 rounded-box backdrop-blur">
                                <span class="countdown font-mono text-3xl">
                                    <span style="--value:6;"></span>
                                </span>
                                "hours"
                            </div>
                            <div class="flex flex-col p-3 bg-white/20 rounded-box backdrop-blur">
                                <span class="countdown font-mono text-3xl">
                                    <span style="--value:42;"></span>
                                </span>
                                "min"
                            </div>
                        </div>

                        <div class="card-actions justify-center mt-6">
                            <Button class="bg-white text-purple-600 hover:bg-gray-100">
                                "Plan Surprise"
                            </Button>
                        </div>
                    </CardBody>
                </Card>
            </div>
        </div>
    }
}