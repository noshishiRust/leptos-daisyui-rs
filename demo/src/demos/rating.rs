use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn RatingDemo() -> impl IntoView {
    let (rating1, set_rating1) = signal(3);
    let (rating2, set_rating2) = signal(4);

    view! {
        <div class="space-y-6">
            <h1 class="text-3xl font-bold">"Rating"</h1>
            <p class="text-base-content/70">"Rating component allows users to rate something"</p>

            <div class="space-y-4">
                <h2 class="text-xl font-semibold">"Basic Rating"</h2>
                <div class="rating">
                    <input type="radio" name="rating-1" class="mask mask-star" />
                    <input type="radio" name="rating-1" class="mask mask-star" />
                    <input type="radio" name="rating-1" class="mask mask-star" checked />
                    <input type="radio" name="rating-1" class="mask mask-star" />
                    <input type="radio" name="rating-1" class="mask mask-star" />
                </div>

                <h2 class="text-xl font-semibold">"Rating with Border"</h2>
                <div class="rating">
                    <input type="radio" name="rating-2" class="mask mask-star-2 bg-orange-400" />
                    <input
                        type="radio"
                        name="rating-2"
                        class="mask mask-star-2 bg-orange-400"
                        checked
                    />
                    <input type="radio" name="rating-2" class="mask mask-star-2 bg-orange-400" />
                    <input type="radio" name="rating-2" class="mask mask-star-2 bg-orange-400" />
                    <input type="radio" name="rating-2" class="mask mask-star-2 bg-orange-400" />
                </div>

                <h2 class="text-xl font-semibold">"Rating Sizes"</h2>
                <div class="space-y-2">
                    <div>
                        <span class="text-sm mr-2">"Extra Small:"</span>
                        <div class="rating rating-xs">
                            <input
                                type="radio"
                                name="rating-3"
                                class="mask mask-star-2 bg-orange-400"
                            />
                            <input
                                type="radio"
                                name="rating-3"
                                class="mask mask-star-2 bg-orange-400"
                            />
                            <input
                                type="radio"
                                name="rating-3"
                                class="mask mask-star-2 bg-orange-400"
                                checked
                            />
                            <input
                                type="radio"
                                name="rating-3"
                                class="mask mask-star-2 bg-orange-400"
                            />
                            <input
                                type="radio"
                                name="rating-3"
                                class="mask mask-star-2 bg-orange-400"
                            />
                        </div>
                    </div>
                    <div>
                        <span class="text-sm mr-2">"Small:"</span>
                        <div class="rating rating-sm">
                            <input
                                type="radio"
                                name="rating-4"
                                class="mask mask-star-2 bg-orange-400"
                            />
                            <input
                                type="radio"
                                name="rating-4"
                                class="mask mask-star-2 bg-orange-400"
                            />
                            <input
                                type="radio"
                                name="rating-4"
                                class="mask mask-star-2 bg-orange-400"
                            />
                            <input
                                type="radio"
                                name="rating-4"
                                class="mask mask-star-2 bg-orange-400"
                                checked
                            />
                            <input
                                type="radio"
                                name="rating-4"
                                class="mask mask-star-2 bg-orange-400"
                            />
                        </div>
                    </div>
                    <div>
                        <span class="text-sm mr-2">"Medium:"</span>
                        <div class="rating rating-md">
                            <input
                                type="radio"
                                name="rating-5"
                                class="mask mask-star-2 bg-orange-400"
                            />
                            <input
                                type="radio"
                                name="rating-5"
                                class="mask mask-star-2 bg-orange-400"
                            />
                            <input
                                type="radio"
                                name="rating-5"
                                class="mask mask-star-2 bg-orange-400"
                            />
                            <input
                                type="radio"
                                name="rating-5"
                                class="mask mask-star-2 bg-orange-400"
                            />
                            <input
                                type="radio"
                                name="rating-5"
                                class="mask mask-star-2 bg-orange-400"
                                checked
                            />
                        </div>
                    </div>
                    <div>
                        <span class="text-sm mr-2">"Large:"</span>
                        <div class="rating rating-lg">
                            <input
                                type="radio"
                                name="rating-6"
                                class="mask mask-star-2 bg-orange-400"
                            />
                            <input
                                type="radio"
                                name="rating-6"
                                class="mask mask-star-2 bg-orange-400"
                                checked
                            />
                            <input
                                type="radio"
                                name="rating-6"
                                class="mask mask-star-2 bg-orange-400"
                            />
                            <input
                                type="radio"
                                name="rating-6"
                                class="mask mask-star-2 bg-orange-400"
                            />
                            <input
                                type="radio"
                                name="rating-6"
                                class="mask mask-star-2 bg-orange-400"
                            />
                        </div>
                    </div>
                </div>

                <h2 class="text-xl font-semibold">"Half Rating"</h2>
                <div class="rating rating-half">
                    <input type="radio" name="rating-10" class="rating-hidden" />
                    <input
                        type="radio"
                        name="rating-10"
                        class="mask mask-star-2 mask-half-1 bg-green-500"
                    />
                    <input
                        type="radio"
                        name="rating-10"
                        class="mask mask-star-2 mask-half-2 bg-green-500"
                    />
                    <input
                        type="radio"
                        name="rating-10"
                        class="mask mask-star-2 mask-half-1 bg-green-500"
                        checked
                    />
                    <input
                        type="radio"
                        name="rating-10"
                        class="mask mask-star-2 mask-half-2 bg-green-500"
                    />
                    <input
                        type="radio"
                        name="rating-10"
                        class="mask mask-star-2 mask-half-1 bg-green-500"
                    />
                    <input
                        type="radio"
                        name="rating-10"
                        class="mask mask-star-2 mask-half-2 bg-green-500"
                    />
                    <input
                        type="radio"
                        name="rating-10"
                        class="mask mask-star-2 mask-half-1 bg-green-500"
                    />
                    <input
                        type="radio"
                        name="rating-10"
                        class="mask mask-star-2 mask-half-2 bg-green-500"
                    />
                    <input
                        type="radio"
                        name="rating-10"
                        class="mask mask-star-2 mask-half-1 bg-green-500"
                    />
                    <input
                        type="radio"
                        name="rating-10"
                        class="mask mask-star-2 mask-half-2 bg-green-500"
                    />
                </div>

                <h2 class="text-xl font-semibold">"Interactive Rating"</h2>
                <div class="flex items-center gap-4">
                    <span class="text-sm">"Your rating:"</span>
                    <div class="rating">
                        {(1..=5)
                            .map(|i| {
                                let star_value = i;
                                view! {
                                    <input
                                        type="radio"
                                        name="rating-interactive"
                                        class="mask mask-star-2 bg-orange-400"
                                        checked=move || rating1.get() == star_value
                                        on:change=move |_| set_rating1.set(star_value)
                                    />
                                }
                            })
                            .collect::<Vec<_>>()}
                    </div>
                    <span class="text-sm">{move || format!("{}/5 stars", rating1.get())}</span>
                </div>

                <h2 class="text-xl font-semibold">"Heart Rating"</h2>
                <div class="rating">
                    <input type="radio" name="rating-7" class="mask mask-heart bg-red-400" />
                    <input
                        type="radio"
                        name="rating-7"
                        class="mask mask-heart bg-red-400"
                        checked
                    />
                    <input type="radio" name="rating-7" class="mask mask-heart bg-red-400" />
                    <input type="radio" name="rating-7" class="mask mask-heart bg-red-400" />
                    <input type="radio" name="rating-7" class="mask mask-heart bg-red-400" />
                </div>

                <h2 class="text-xl font-semibold">"Custom Rating Shapes"</h2>
                <div class="space-y-3">
                    <div>
                        <span class="text-sm mr-2">"Circle:"</span>
                        <div class="rating">
                            <input
                                type="radio"
                                name="rating-8"
                                class="mask mask-circle bg-blue-400"
                            />
                            <input
                                type="radio"
                                name="rating-8"
                                class="mask mask-circle bg-blue-400"
                            />
                            <input
                                type="radio"
                                name="rating-8"
                                class="mask mask-circle bg-blue-400"
                                checked
                            />
                            <input
                                type="radio"
                                name="rating-8"
                                class="mask mask-circle bg-blue-400"
                            />
                            <input
                                type="radio"
                                name="rating-8"
                                class="mask mask-circle bg-blue-400"
                            />
                        </div>
                    </div>
                    <div>
                        <span class="text-sm mr-2">"Triangle:"</span>
                        <div class="rating">
                            <input
                                type="radio"
                                name="rating-9"
                                class="mask mask-triangle bg-purple-400"
                            />
                            <input
                                type="radio"
                                name="rating-9"
                                class="mask mask-triangle bg-purple-400"
                            />
                            <input
                                type="radio"
                                name="rating-9"
                                class="mask mask-triangle bg-purple-400"
                            />
                            <input
                                type="radio"
                                name="rating-9"
                                class="mask mask-triangle bg-purple-400"
                                checked
                            />
                            <input
                                type="radio"
                                name="rating-9"
                                class="mask mask-triangle bg-purple-400"
                            />
                        </div>
                    </div>
                </div>

                <h2 class="text-xl font-semibold">"Product Rating Example"</h2>
                <Card class="bg-base-100 shadow-xl">
                    <CardBody>
                        <h2 class="card-title">"Rate this product"</h2>
                        <div class="flex items-center gap-4 my-4">
                            <span class="text-lg">"Quality:"</span>
                            <div class="rating">
                                {(1..=5)
                                    .map(|i| {
                                        let star_value = i;
                                        view! {
                                            <input
                                                type="radio"
                                                name="product-rating"
                                                class="mask mask-star-2 bg-yellow-400"
                                                checked=move || rating2.get() == star_value
                                                on:change=move |_| set_rating2.set(star_value)
                                            />
                                        }
                                    })
                                    .collect::<Vec<_>>()}
                            </div>
                            <span class="text-sm text-base-content/70">
                                {move || {
                                    match rating2.get() {
                                        1 => "Poor",
                                        2 => "Fair",
                                        3 => "Good",
                                        4 => "Very Good",
                                        5 => "Excellent",
                                        _ => "Not rated",
                                    }
                                }}
                            </span>
                        </div>
                        <div class="card-actions justify-end">
                            <Button
                                color=ButtonColor::Primary
                                disabled=Signal::derive(move || rating2.get() == 0)
                            >
                                "Submit Rating"
                            </Button>
                        </div>
                    </CardBody>
                </Card>

                <h2 class="text-xl font-semibold">"Disabled Rating"</h2>
                <div class="rating">
                    <input
                        type="radio"
                        name="rating-disabled"
                        class="mask mask-star-2 bg-gray-300"
                        disabled
                    />
                    <input
                        type="radio"
                        name="rating-disabled"
                        class="mask mask-star-2 bg-gray-300"
                        disabled
                    />
                    <input
                        type="radio"
                        name="rating-disabled"
                        class="mask mask-star-2 bg-gray-300"
                        disabled
                        checked
                    />
                    <input
                        type="radio"
                        name="rating-disabled"
                        class="mask mask-star-2 bg-gray-300"
                        disabled
                    />
                    <input
                        type="radio"
                        name="rating-disabled"
                        class="mask mask-star-2 bg-gray-300"
                        disabled
                    />
                </div>
            </div>
        </div>
    }
}