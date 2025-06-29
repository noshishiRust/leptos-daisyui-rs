use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn CardDemo() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <h1 class="text-3xl font-bold">"Card"</h1>
            <p class="text-base-content/70">
                "Cards are used to group related information in a flexible and extensible container"
            </p>

            <div class="space-y-4">
                <h2 class="text-xl font-semibold">"Basic Card"</h2>
                <Card class="w-96 bg-base-100 shadow-xl">
                    <CardBody>
                        <h2 class="card-title">"Card title!"</h2>
                        <p>"If a dog chews shoes whose shoes does he choose?"</p>
                        <div class="card-actions justify-end">
                            <Button color=Signal::derive(move || {
                                ButtonColor::Primary
                            })>"Buy Now"</Button>
                        </div>
                    </CardBody>
                </Card>

                <h2 class="text-xl font-semibold">"Card with Image"</h2>
                <Card class="w-96 bg-base-100 shadow-xl">
                    <figure>
                        <img src="https://picsum.photos/400/200?random=1" alt="Demo image" />
                    </figure>
                    <CardBody>
                        <h2 class="card-title">"Shoes!"</h2>
                        <p>"If a dog chews shoes whose shoes does he choose?"</p>
                        <div class="card-actions justify-end">
                            <Button color=Signal::derive(move || {
                                ButtonColor::Primary
                            })>"Buy Now"</Button>
                        </div>
                    </CardBody>
                </Card>

                <h2 class="text-xl font-semibold">"Compact Card"</h2>
                <Card class="card-compact w-96 bg-base-100 shadow-xl">
                    <figure>
                        <img src="https://picsum.photos/400/200?random=2" alt="Demo image" />
                    </figure>
                    <CardBody>
                        <h2 class="card-title">"Compact card!"</h2>
                        <p>"This card uses compact style"</p>
                        <div class="card-actions justify-end">
                            <Button
                                size=Signal::derive(move || ButtonSize::Sm)
                                color=Signal::derive(move || ButtonColor::Primary)
                            >
                                "Buy Now"
                            </Button>
                        </div>
                    </CardBody>
                </Card>
            </div>
        </div>
    }
}