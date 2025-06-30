use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn StepsDemo() -> impl IntoView {
    let (current_step, set_current_step) = signal(1);

    view! {
        <div class="space-y-6">
            <h1 class="text-3xl font-bold">"Steps"</h1>
            <p class="text-base-content/70">
                "Steps component shows a sequence of steps in a process"
            </p>

            <div class="space-y-4">
                <h2 class="text-xl font-semibold">"Basic Steps"</h2>
                <ul class="steps">
                    <li class="step step-primary">"Register"</li>
                    <li class="step step-primary">"Choose plan"</li>
                    <li class="step">"Purchase"</li>
                    <li class="step">"Receive Product"</li>
                </ul>

                <h2 class="text-xl font-semibold">"Vertical Steps"</h2>
                <ul class="steps steps-vertical">
                    <li class="step step-primary">"Step 1"</li>
                    <li class="step step-primary">"Step 2"</li>
                    <li class="step">"Step 3"</li>
                    <li class="step">"Step 4"</li>
                </ul>

                <h2 class="text-xl font-semibold">"Responsive Steps"</h2>
                <ul class="steps steps-vertical lg:steps-horizontal">
                    <li class="step step-primary">"Desktop"</li>
                    <li class="step step-primary">"Laptop"</li>
                    <li class="step">"Tablet"</li>
                    <li class="step">"Mobile"</li>
                </ul>

                <h2 class="text-xl font-semibold">"Steps with Custom Content"</h2>
                <ul class="steps">
                    <li class="step step-primary" data-content="1">
                        "Start"
                    </li>
                    <li class="step step-primary" data-content="2">
                        "Processing"
                    </li>
                    <li class="step" data-content="3">
                        "Review"
                    </li>
                    <li class="step" data-content="4">
                        "Complete"
                    </li>
                </ul>

                <h2 class="text-xl font-semibold">"Steps with Icons"</h2>
                <ul class="steps">
                    <li class="step step-primary" data-content="📝">
                        "Write"
                    </li>
                    <li class="step step-primary" data-content="👀">
                        "Review"
                    </li>
                    <li class="step" data-content="📤">
                        "Submit"
                    </li>
                    <li class="step" data-content="✅">
                        "Approve"
                    </li>
                </ul>

                <h2 class="text-xl font-semibold">"Interactive Steps"</h2>
                <div class="space-y-4">
                    <ul class="steps">
                        <li class=move || {
                            if current_step.get() >= 1 { "step step-primary" } else { "step" }
                        }>"Choose Product"</li>
                        <li class=move || {
                            if current_step.get() >= 2 { "step step-primary" } else { "step" }
                        }>"Add to Cart"</li>
                        <li class=move || {
                            if current_step.get() >= 3 { "step step-primary" } else { "step" }
                        }>"Checkout"</li>
                        <li class=move || {
                            if current_step.get() >= 4 { "step step-primary" } else { "step" }
                        }>"Payment"</li>
                        <li class=move || {
                            if current_step.get() >= 5 { "step step-primary" } else { "step" }
                        }>"Confirmation"</li>
                    </ul>

                    <div class="flex gap-2">
                        <Button
                            size=Signal::derive(|| ButtonSize::Sm)
                            style=Signal::derive(|| ButtonStyle::Outline)
                            disabled=Signal::derive(move || current_step.get() <= 1)
                            on:click=move |_| {
                                set_current_step
                                    .update(|step| {
                                        if *step > 1 {
                                            *step -= 1;
                                        }
                                    })
                            }
                        >
                            "Previous"
                        </Button>
                        <Button
                            size=Signal::derive(|| ButtonSize::Sm)
                            color=Signal::derive(|| ButtonColor::Primary)
                            disabled=Signal::derive(move || current_step.get() >= 5)
                            on:click=move |_| {
                                set_current_step
                                    .update(|step| {
                                        if *step < 5 {
                                            *step += 1;
                                        }
                                    })
                            }
                        >
                            "Next"
                        </Button>
                        <Button
                            size=Signal::derive(|| ButtonSize::Sm)
                            style=Signal::derive(|| ButtonStyle::Ghost)
                            on:click=move |_| set_current_step.set(1)
                        >
                            "Reset"
                        </Button>
                    </div>

                    <div class="bg-base-100 p-4 rounded-lg border">
                        <h3 class="font-semibold mb-2">
                            "Current Step: " {move || current_step.get()} " of 5"
                        </h3>
                        {move || match current_step.get() {
                            1 => {
                                view! {
                                    <p>
                                        "Browse our products and select the one you want to purchase."
                                    </p>
                                }
                                    .into_any()
                            }
                            2 => {
                                view! { <p>"Add your selected product to the shopping cart."</p> }
                                    .into_any()
                            }
                            3 => {
                                view! { <p>"Review your cart items and proceed to checkout."</p> }
                                    .into_any()
                            }
                            4 => {
                                view! {
                                    <p>
                                        "Enter your payment information and complete the purchase."
                                    </p>
                                }
                                    .into_any()
                            }
                            5 => {
                                view! {
                                    <p>
                                        "Order confirmed! You will receive a confirmation email shortly."
                                    </p>
                                }
                                    .into_any()
                            }
                            _ => view! { <p>"Unknown step"</p> }.into_any(),
                        }}
                    </div>
                </div>

                <h2 class="text-xl font-semibold">"Multi-color Steps"</h2>
                <ul class="steps">
                    <li class="step step-info">"Info"</li>
                    <li class="step step-success">"Success"</li>
                    <li class="step step-warning">"Warning"</li>
                    <li class="step step-error">"Error"</li>
                </ul>

                <h2 class="text-xl font-semibold">"Form Wizard Example"</h2>
                <Card class="bg-base-100 shadow-xl">
                    <CardBody>
                        <h2 class="card-title">"User Registration Wizard"</h2>

                        <ul class="steps mb-6">
                            <li class="step step-primary" data-content="1">
                                "Personal Info"
                            </li>
                            <li class="step step-primary" data-content="2">
                                "Account Details"
                            </li>
                            <li class="step" data-content="3">
                                "Verification"
                            </li>
                            <li class="step" data-content="4">
                                "Complete"
                            </li>
                        </ul>

                        <div class="space-y-4">
                            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                                <div class="form-control">
                                    <label class="label">
                                        <span class="label-text">"First Name"</span>
                                    </label>
                                    <Input placeholder="Enter first name" class="w-full" />
                                </div>
                                <div class="form-control">
                                    <label class="label">
                                        <span class="label-text">"Last Name"</span>
                                    </label>
                                    <Input placeholder="Enter last name" class="w-full" />
                                </div>
                            </div>

                            <div class="form-control">
                                <label class="label">
                                    <span class="label-text">"Email Address"</span>
                                </label>
                                <input
                                    type="email"
                                    placeholder="Enter email"
                                    class="input input-bordered w-full"
                                />
                            </div>

                            <div class="form-control">
                                <label class="label">
                                    <span class="label-text">"Phone Number"</span>
                                </label>
                                <input
                                    type="tel"
                                    placeholder="Enter phone number"
                                    class="input input-bordered w-full"
                                />
                            </div>
                        </div>

                        <div class="card-actions justify-between mt-6">
                            <Button style=Signal::derive(|| ButtonStyle::Ghost)>"Cancel"</Button>
                            <Button color=Signal::derive(|| {
                                ButtonColor::Primary
                            })>"Continue"</Button>
                        </div>
                    </CardBody>
                </Card>

                <h2 class="text-xl font-semibold">"Project Timeline Steps"</h2>
                <div class="space-y-4">
                    <ul class="steps steps-vertical w-full">
                        <li class="step step-primary">
                            <div class="text-left">
                                <div class="font-semibold">"Project Planning"</div>
                                <div class="text-sm text-base-content/70">
                                    "Define requirements and scope"
                                </div>
                            </div>
                        </li>
                        <li class="step step-primary">
                            <div class="text-left">
                                <div class="font-semibold">"Design Phase"</div>
                                <div class="text-sm text-base-content/70">
                                    "Create wireframes and mockups"
                                </div>
                            </div>
                        </li>
                        <li class="step step-primary">
                            <div class="text-left">
                                <div class="font-semibold">"Development"</div>
                                <div class="text-sm text-base-content/70">
                                    "Build the application"
                                </div>
                            </div>
                        </li>
                        <li class="step">
                            <div class="text-left">
                                <div class="font-semibold">"Testing"</div>
                                <div class="text-sm text-base-content/70">
                                    "Quality assurance and bug fixes"
                                </div>
                            </div>
                        </li>
                        <li class="step">
                            <div class="text-left">
                                <div class="font-semibold">"Deployment"</div>
                                <div class="text-sm text-base-content/70">
                                    "Launch to production"
                                </div>
                            </div>
                        </li>
                    </ul>
                </div>

                <h2 class="text-xl font-semibold">"Learning Progress"</h2>
                <Card class="bg-base-100 shadow-xl">
                    <CardBody>
                        <h2 class="card-title">"Rust Learning Path"</h2>

                        <ul class="steps steps-vertical">
                            <li class="step step-success" data-content="✓">
                                "Basics"
                            </li>
                            <li class="step step-success" data-content="✓">
                                "Ownership"
                            </li>
                            <li class="step step-primary" data-content="🔄">
                                "Structs & Enums"
                            </li>
                            <li class="step" data-content="📚">
                                "Collections"
                            </li>
                            <li class="step" data-content="📚">
                                "Error Handling"
                            </li>
                            <li class="step" data-content="📚">
                                "Traits"
                            </li>
                            <li class="step" data-content="📚">
                                "Lifetimes"
                            </li>
                            <li class="step" data-content="📚">
                                "Concurrency"
                            </li>
                        </ul>

                        <div class="mt-4">
                            <div class="text-sm text-base-content/70">
                                "Progress: 3/8 completed"
                            </div>
                            <Progress
                                value=37.5
                                max=100.0
                                color=Signal::derive(|| ProgressColor::Primary)
                                class="w-full mt-2"
                            />
                        </div>
                    </CardBody>
                </Card>
            </div>
        </div>
    }
}