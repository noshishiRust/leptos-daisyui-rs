use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn HeroDemo() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <h1 class="text-3xl font-bold">"Hero"</h1>
            <p class="text-base-content/70">
                "Hero section is a large banner usually used at the top of a page"
            </p>

            <div class="space-y-8">
                <h2 class="text-xl font-semibold">"Basic Hero"</h2>
                <div class="hero min-h-screen bg-base-200">
                    <div class="hero-content text-center">
                        <div class="max-w-md">
                            <h1 class="text-5xl font-bold">"Hello there"</h1>
                            <p class="py-6">
                                "Provident cupiditate voluptatem et in. Quaerat fugiat ut assumenda excepturi exercitationem quasi. In deleniti eaque aut repudiandae et a id nisi."
                            </p>
                            <Button color=ButtonColor::Primary>"Get Started"</Button>
                        </div>
                    </div>
                </div>

                <h2 class="text-xl font-semibold">"Hero with Background Image"</h2>
                <div
                    class="hero min-h-screen"
                    style="background-image: url(https://picsum.photos/1200/800?random=1);"
                >
                    <div class="hero-overlay bg-opacity-60"></div>
                    <div class="hero-content text-center text-neutral-content">
                        <div class="max-w-md">
                            <h1 class="mb-5 text-5xl font-bold">"Hello there"</h1>
                            <p class="mb-5">
                                "Provident cupiditate voluptatem et in. Quaerat fugiat ut assumenda excepturi exercitationem quasi. In deleniti eaque aut repudiandae et a id nisi."
                            </p>
                            <Button color=ButtonColor::Primary>"Get Started"</Button>
                        </div>
                    </div>
                </div>

                <h2 class="text-xl font-semibold">"Hero with Form"</h2>
                <div class="hero min-h-screen bg-base-200">
                    <div class="hero-content flex-col lg:flex-row-reverse">
                        <div class="text-center lg:text-left">
                            <h1 class="text-5xl font-bold">"Login now!"</h1>
                            <p class="py-6">
                                "Provident cupiditate voluptatem et in. Quaerat fugiat ut assumenda excepturi exercitationem quasi. In deleniti eaque aut repudiandae et a id nisi."
                            </p>
                        </div>
                        <div class="card shrink-0 w-full max-w-sm shadow-2xl bg-base-100">
                            <div class="card-body">
                                <div class="form-control">
                                    <label class="label">
                                        <span class="label-text">"Email"</span>
                                    </label>
                                    <Input placeholder="email" class="w-full" />
                                </div>
                                <div class="form-control">
                                    <label class="label">
                                        <span class="label-text">"Password"</span>
                                    </label>
                                    <input
                                        type="password"
                                        placeholder="password"
                                        class="input input-bordered w-full"
                                    />
                                    <label class="label">
                                        <a href="#" class="label-text-alt link link-hover">
                                            "Forgot password?"
                                        </a>
                                    </label>
                                </div>
                                <div class="form-control mt-6">
                                    <Button color=ButtonColor::Primary>"Login"</Button>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>

                <h2 class="text-xl font-semibold">"Hero with Figure"</h2>
                <div class="hero min-h-screen bg-base-200">
                    <div class="hero-content flex-col lg:flex-row">
                        <img
                            src="https://picsum.photos/400/600?random=2"
                            class="max-w-sm rounded-lg shadow-2xl"
                            alt="Hero illustration"
                        />
                        <div>
                            <h1 class="text-5xl font-bold">"Box Office News!"</h1>
                            <p class="py-6">
                                "Provident cupiditate voluptatem et in. Quaerat fugiat ut assumenda excepturi exercitationem quasi. In deleniti eaque aut repudiandae et a id nisi."
                            </p>
                            <Button color=ButtonColor::Primary>"Get Started"</Button>
                        </div>
                    </div>
                </div>

                <h2 class="text-xl font-semibold">"Hero with Glass Effect"</h2>
                <div
                    class="hero min-h-screen"
                    style="background-image: url(https://picsum.photos/1200/800?random=3);"
                >
                    <div class="hero-overlay bg-opacity-60"></div>
                    <div class="hero-content text-center text-neutral-content">
                        <div class="max-w-md bg-black bg-opacity-20 backdrop-blur-sm p-8 rounded-2xl">
                            <h1 class="mb-5 text-5xl font-bold">"Glass Hero"</h1>
                            <p class="mb-5">
                                "Modern glassmorphism design with backdrop blur effect. Perfect for creating stunning landing pages."
                            </p>
                            <div class="flex gap-2 justify-center">
                                <Button style=ButtonStyle::Outline class="glass">
                                    "Learn More"
                                </Button>
                                <Button color=ButtonColor::Primary class="glass">
                                    "Get Started"
                                </Button>
                            </div>
                        </div>
                    </div>
                </div>

                <h2 class="text-xl font-semibold">"Minimal Hero"</h2>
                <div class="hero min-h-96 bg-base-100">
                    <div class="hero-content text-center">
                        <div class="max-w-lg">
                            <h1 class="text-4xl font-light text-base-content">"Simple & Clean"</h1>
                            <p class="py-4 text-base-content/70">
                                "Sometimes less is more. A minimalist approach to hero sections."
                            </p>
                            <div class="flex gap-2 justify-center">
                                <Button style=ButtonStyle::Ghost>"Learn More"</Button>
                                <Button color=ButtonColor::Primary>"Contact"</Button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
