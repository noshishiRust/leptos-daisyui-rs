use crate::core::{ContentLayout, Section};
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn Hover3dDemo() -> impl IntoView {
    view! {
        <ContentLayout
            title="Hover 3D"
            description="3D tilt effect on mouse hover for interactive displays"
        >
            <Section title="Basic Card Example">
                <div class="flex justify-center">
                    <Hover3d>
                        <Card class="w-96 bg-gradient-to-br from-primary to-secondary text-primary-content shadow-xl">
                            <CardBody>
                                <h2 class="card-title">"Hover over me!"</h2>
                                <p>"Move your mouse around to see the 3D tilt effect."</p>
                            </CardBody>
                        </Card>
                    </Hover3d>
                </div>
            </Section>

            <Section title="Credit Card Style">
                <div class="flex justify-center">
                    <Hover3d>
                        <div class="w-96 h-56 bg-gradient-to-br from-blue-600 to-purple-700 rounded-xl shadow-2xl p-6 text-white">
                            <div class="flex justify-between items-start mb-8">
                                <div class="text-2xl font-bold">"BANK"</div>
                                <div class="w-12 h-8 bg-yellow-400 rounded"></div>
                            </div>
                            <div class="mb-4">
                                <div class="text-sm opacity-75">"Card Number"</div>
                                <div class="text-xl tracking-wider font-mono">
                                    "4532 •••• •••• 8765"
                                </div>
                            </div>
                            <div class="flex justify-between">
                                <div>
                                    <div class="text-xs opacity-75">"Card Holder"</div>
                                    <div class="text-sm font-semibold">"JOHN DOE"</div>
                                </div>
                                <div>
                                    <div class="text-xs opacity-75">"Expires"</div>
                                    <div class="text-sm font-semibold">"12/26"</div>
                                </div>
                            </div>
                        </div>
                    </Hover3d>
                </div>
            </Section>

            <Section title="Product Showcase">
                <div class="flex justify-center gap-6 flex-wrap">
                    <Hover3d>
                        <Card class="w-64 bg-base-200 shadow-xl">
                            <figure class="px-10 pt-10">
                                <div class="w-full h-32 bg-primary rounded-lg flex items-center justify-center text-primary-content text-4xl">
                                    "📱"
                                </div>
                            </figure>
                            <CardBody class="items-center text-center">
                                <h2 class="card-title">"Product 1"</h2>
                                <p>"Amazing features"</p>
                            </CardBody>
                        </Card>
                    </Hover3d>

                    <Hover3d>
                        <Card class="w-64 bg-base-200 shadow-xl">
                            <figure class="px-10 pt-10">
                                <div class="w-full h-32 bg-secondary rounded-lg flex items-center justify-center text-secondary-content text-4xl">
                                    "💻"
                                </div>
                            </figure>
                            <CardBody class="items-center text-center">
                                <h2 class="card-title">"Product 2"</h2>
                                <p>"Great performance"</p>
                            </CardBody>
                        </Card>
                    </Hover3d>

                    <Hover3d>
                        <Card class="w-64 bg-base-200 shadow-xl">
                            <figure class="px-10 pt-10">
                                <div class="w-full h-32 bg-accent rounded-lg flex items-center justify-center text-accent-content text-4xl">
                                    "🎧"
                                </div>
                            </figure>
                            <CardBody class="items-center text-center">
                                <h2 class="card-title">"Product 3"</h2>
                                <p>"Premium quality"</p>
                            </CardBody>
                        </Card>
                    </Hover3d>
                </div>
            </Section>

            <Section title="Image Gallery">
                <div class="flex justify-center gap-6 flex-wrap">
                    <Hover3d>
                        <div class="w-48 h-48 bg-gradient-to-br from-pink-500 to-rose-500 rounded-xl flex items-center justify-center text-white text-6xl shadow-xl">
                            "🌸"
                        </div>
                    </Hover3d>
                    <Hover3d>
                        <div class="w-48 h-48 bg-gradient-to-br from-blue-500 to-cyan-500 rounded-xl flex items-center justify-center text-white text-6xl shadow-xl">
                            "🌊"
                        </div>
                    </Hover3d>
                    <Hover3d>
                        <div class="w-48 h-48 bg-gradient-to-br from-green-500 to-emerald-500 rounded-xl flex items-center justify-center text-white text-6xl shadow-xl">
                            "🌲"
                        </div>
                    </Hover3d>
                </div>
            </Section>

            <Section title="Important Note">
                <div class="alert alert-warning">
                    <div>
                        <strong>"Warning:"</strong>
                        " Only use non-interactive content inside Hover3d. "
                        "Do not place buttons or links inside as the hover zones will prevent interaction. "
                        "If you need the entire card to be clickable, wrap the Hover3d component in a link."
                    </div>
                </div>
            </Section>
        </ContentLayout>
    }
}
