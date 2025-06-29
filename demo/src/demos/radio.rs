use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn RadioDemo() -> impl IntoView {
    let (selected_option, set_selected_option) = signal("option1".to_string());
    let (selected_color, set_selected_color) = signal("primary".to_string());
    let (selected_size, set_selected_size) = signal("md".to_string());

    view! {
        <div class="space-y-6">
            <h1 class="text-3xl font-bold">"Radio"</h1>
            <p class="text-base-content/70">
                "Radio buttons allow users to select one option from a set"
            </p>

            <div class="space-y-4">
                <h2 class="text-xl font-semibold">"Basic Radio Buttons"</h2>
                <div class="flex flex-col gap-2">
                    <div class="form-control">
                        <label class="label cursor-pointer justify-start gap-2">
                            <input 
                                type="radio" 
                                name="radio-basic" 
                                class="radio" 
                                value="option1"
                                checked=move || selected_option.get() == "option1"
                                on:change=move |_| set_selected_option.set("option1".to_string())
                            />
                            <span class="label-text">"Option 1"</span>
                        </label>
                    </div>
                    <div class="form-control">
                        <label class="label cursor-pointer justify-start gap-2">
                            <input 
                                type="radio" 
                                name="radio-basic" 
                                class="radio" 
                                value="option2"
                                checked=move || selected_option.get() == "option2"
                                on:change=move |_| set_selected_option.set("option2".to_string())
                            />
                            <span class="label-text">"Option 2"</span>
                        </label>
                    </div>
                    <div class="form-control">
                        <label class="label cursor-pointer justify-start gap-2">
                            <input 
                                type="radio" 
                                name="radio-basic" 
                                class="radio" 
                                value="option3"
                                checked=move || selected_option.get() == "option3"
                                on:change=move |_| set_selected_option.set("option3".to_string())
                            />
                            <span class="label-text">"Option 3"</span>
                        </label>
                    </div>
                </div>
                <p class="text-sm text-base-content/70">
                    "Selected: " {move || selected_option.get()}
                </p>

                <h2 class="text-xl font-semibold">"Radio Button Colors"</h2>
                <div class="flex flex-wrap gap-4">
                    <div class="form-control">
                        <label class="label cursor-pointer justify-start gap-2">
                            <input 
                                type="radio" 
                                name="radio-color" 
                                class="radio" 
                                value="default"
                                checked=move || selected_color.get() == "default"
                                on:change=move |_| set_selected_color.set("default".to_string())
                            />
                            <span class="label-text">"Default"</span>
                        </label>
                    </div>
                    <div class="form-control">
                        <label class="label cursor-pointer justify-start gap-2">
                            <input 
                                type="radio" 
                                name="radio-color" 
                                class="radio radio-primary" 
                                value="primary"
                                checked=move || selected_color.get() == "primary"
                                on:change=move |_| set_selected_color.set("primary".to_string())
                            />
                            <span class="label-text">"Primary"</span>
                        </label>
                    </div>
                    <div class="form-control">
                        <label class="label cursor-pointer justify-start gap-2">
                            <input 
                                type="radio" 
                                name="radio-color" 
                                class="radio radio-secondary" 
                                value="secondary"
                                checked=move || selected_color.get() == "secondary"
                                on:change=move |_| set_selected_color.set("secondary".to_string())
                            />
                            <span class="label-text">"Secondary"</span>
                        </label>
                    </div>
                    <div class="form-control">
                        <label class="label cursor-pointer justify-start gap-2">
                            <input 
                                type="radio" 
                                name="radio-color" 
                                class="radio radio-accent" 
                                value="accent"
                                checked=move || selected_color.get() == "accent"
                                on:change=move |_| set_selected_color.set("accent".to_string())
                            />
                            <span class="label-text">"Accent"</span>
                        </label>
                    </div>
                    <div class="form-control">
                        <label class="label cursor-pointer justify-start gap-2">
                            <input 
                                type="radio" 
                                name="radio-color" 
                                class="radio radio-success" 
                                value="success"
                                checked=move || selected_color.get() == "success"
                                on:change=move |_| set_selected_color.set("success".to_string())
                            />
                            <span class="label-text">"Success"</span>
                        </label>
                    </div>
                    <div class="form-control">
                        <label class="label cursor-pointer justify-start gap-2">
                            <input 
                                type="radio" 
                                name="radio-color" 
                                class="radio radio-warning" 
                                value="warning"
                                checked=move || selected_color.get() == "warning"
                                on:change=move |_| set_selected_color.set("warning".to_string())
                            />
                            <span class="label-text">"Warning"</span>
                        </label>
                    </div>
                    <div class="form-control">
                        <label class="label cursor-pointer justify-start gap-2">
                            <input 
                                type="radio" 
                                name="radio-color" 
                                class="radio radio-info" 
                                value="info"
                                checked=move || selected_color.get() == "info"
                                on:change=move |_| set_selected_color.set("info".to_string())
                            />
                            <span class="label-text">"Info"</span>
                        </label>
                    </div>
                    <div class="form-control">
                        <label class="label cursor-pointer justify-start gap-2">
                            <input 
                                type="radio" 
                                name="radio-color" 
                                class="radio radio-error" 
                                value="error"
                                checked=move || selected_color.get() == "error"
                                on:change=move |_| set_selected_color.set("error".to_string())
                            />
                            <span class="label-text">"Error"</span>
                        </label>
                    </div>
                </div>

                <h2 class="text-xl font-semibold">"Radio Button Sizes"</h2>
                <div class="flex items-center gap-4">
                    <div class="form-control">
                        <label class="label cursor-pointer justify-start gap-2">
                            <input 
                                type="radio" 
                                name="radio-size" 
                                class="radio radio-xs" 
                                value="xs"
                                checked=move || selected_size.get() == "xs"
                                on:change=move |_| set_selected_size.set("xs".to_string())
                            />
                            <span class="label-text text-xs">"XS"</span>
                        </label>
                    </div>
                    <div class="form-control">
                        <label class="label cursor-pointer justify-start gap-2">
                            <input 
                                type="radio" 
                                name="radio-size" 
                                class="radio radio-sm" 
                                value="sm"
                                checked=move || selected_size.get() == "sm"
                                on:change=move |_| set_selected_size.set("sm".to_string())
                            />
                            <span class="label-text text-sm">"SM"</span>
                        </label>
                    </div>
                    <div class="form-control">
                        <label class="label cursor-pointer justify-start gap-2">
                            <input 
                                type="radio" 
                                name="radio-size" 
                                class="radio" 
                                value="md"
                                checked=move || selected_size.get() == "md"
                                on:change=move |_| set_selected_size.set("md".to_string())
                            />
                            <span class="label-text">"MD"</span>
                        </label>
                    </div>
                    <div class="form-control">
                        <label class="label cursor-pointer justify-start gap-2">
                            <input 
                                type="radio" 
                                name="radio-size" 
                                class="radio radio-lg" 
                                value="lg"
                                checked=move || selected_size.get() == "lg"
                                on:change=move |_| set_selected_size.set("lg".to_string())
                            />
                            <span class="label-text text-lg">"LG"</span>
                        </label>
                    </div>
                </div>

                <h2 class="text-xl font-semibold">"Disabled State"</h2>
                <div class="flex gap-4">
                    <div class="form-control">
                        <label class="label cursor-pointer justify-start gap-2">
                            <input type="radio" name="radio-disabled" class="radio" disabled />
                            <span class="label-text">"Disabled unchecked"</span>
                        </label>
                    </div>
                    <div class="form-control">
                        <label class="label cursor-pointer justify-start gap-2">
                            <input type="radio" name="radio-disabled" class="radio" disabled checked />
                            <span class="label-text">"Disabled checked"</span>
                        </label>
                    </div>
                </div>

                <h2 class="text-xl font-semibold">"Radio Group Example"</h2>
                <Card class="bg-base-100 shadow-xl">
                    <CardBody>
                        <h2 class="card-title">"Choose your favorite framework"</h2>
                        <div class="space-y-2">
                            <div class="form-control">
                                <label class="label cursor-pointer justify-start gap-2">
                                    <input type="radio" name="framework" class="radio radio-primary" />
                                    <span class="label-text">"Leptos (Rust)"</span>
                                </label>
                            </div>
                            <div class="form-control">
                                <label class="label cursor-pointer justify-start gap-2">
                                    <input type="radio" name="framework" class="radio radio-primary" />
                                    <span class="label-text">"React (JavaScript)"</span>
                                </label>
                            </div>
                            <div class="form-control">
                                <label class="label cursor-pointer justify-start gap-2">
                                    <input type="radio" name="framework" class="radio radio-primary" />
                                    <span class="label-text">"Vue (JavaScript)"</span>
                                </label>
                            </div>
                            <div class="form-control">
                                <label class="label cursor-pointer justify-start gap-2">
                                    <input type="radio" name="framework" class="radio radio-primary" />
                                    <span class="label-text">"Svelte (JavaScript)"</span>
                                </label>
                            </div>
                        </div>
                        <div class="card-actions justify-end mt-4">
                            <Button color=Signal::derive(|| ButtonColor::Primary)>"Submit"</Button>
                        </div>
                    </CardBody>
                </Card>
            </div>
        </div>
    }
}