use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn RangeDemo() -> impl IntoView {
    let (value1, set_value1) = signal(25.0);
    let (value2, set_value2) = signal(40.0);
    let (value3, set_value3) = signal(60.0);

    view! {
        <div class="space-y-6">
            <h1 class="text-3xl font-bold">"Range"</h1>
            <p class="text-base-content/70">
                "Range slider is used to select a value by sliding a handle"
            </p>

            <div class="space-y-4">
                <h2 class="text-xl font-semibold">"Basic Range"</h2>
                <input type="range" min="0" max="100" value="50" class="range" />

                <h2 class="text-xl font-semibold">"Range with Steps and Labels"</h2>
                <input type="range" min="0" max="100" value="25" class="range" step="25" />
                <div class="w-full flex justify-between text-xs px-2">
                    <span>"|"</span>
                    <span>"|"</span>
                    <span>"|"</span>
                    <span>"|"</span>
                    <span>"|"</span>
                </div>

                <h2 class="text-xl font-semibold">"Range Colors"</h2>
                <div class="space-y-3">
                    <div>
                        <label class="text-sm">"Default"</label>
                        <input type="range" min="0" max="100" value="50" class="range" />
                    </div>
                    <div>
                        <label class="text-sm">"Primary"</label>
                        <input
                            type="range"
                            min="0"
                            max="100"
                            value="50"
                            class="range range-primary"
                        />
                    </div>
                    <div>
                        <label class="text-sm">"Secondary"</label>
                        <input
                            type="range"
                            min="0"
                            max="100"
                            value="50"
                            class="range range-secondary"
                        />
                    </div>
                    <div>
                        <label class="text-sm">"Accent"</label>
                        <input
                            type="range"
                            min="0"
                            max="100"
                            value="50"
                            class="range range-accent"
                        />
                    </div>
                    <div>
                        <label class="text-sm">"Success"</label>
                        <input
                            type="range"
                            min="0"
                            max="100"
                            value="50"
                            class="range range-success"
                        />
                    </div>
                    <div>
                        <label class="text-sm">"Warning"</label>
                        <input
                            type="range"
                            min="0"
                            max="100"
                            value="50"
                            class="range range-warning"
                        />
                    </div>
                    <div>
                        <label class="text-sm">"Info"</label>
                        <input type="range" min="0" max="100" value="50" class="range range-info" />
                    </div>
                    <div>
                        <label class="text-sm">"Error"</label>
                        <input
                            type="range"
                            min="0"
                            max="100"
                            value="50"
                            class="range range-error"
                        />
                    </div>
                </div>

                <h2 class="text-xl font-semibold">"Range Sizes"</h2>
                <div class="space-y-3">
                    <div>
                        <label class="text-sm">"Extra Small"</label>
                        <input type="range" min="0" max="100" value="20" class="range range-xs" />
                    </div>
                    <div>
                        <label class="text-sm">"Small"</label>
                        <input type="range" min="0" max="100" value="30" class="range range-sm" />
                    </div>
                    <div>
                        <label class="text-sm">"Medium (Default)"</label>
                        <input type="range" min="0" max="100" value="40" class="range range-md" />
                    </div>
                    <div>
                        <label class="text-sm">"Large"</label>
                        <input type="range" min="0" max="100" value="50" class="range range-lg" />
                    </div>
                </div>

                <h2 class="text-xl font-semibold">"Interactive Range"</h2>
                <div class="space-y-4">
                    <div>
                        <label class="label">
                            <span class="label-text">
                                "Volume: " {move || (value1.get() as f64).round() as i32} "%"
                            </span>
                        </label>
                        <input
                            type="range"
                            min="0"
                            max="100"
                            value=move || value1.get()
                            class="range range-primary"
                            on:input=move |ev| {
                                let value = event_target_value(&ev).parse::<f64>().unwrap_or(0.0);
                                set_value1.set(value);
                            }
                        />
                    </div>

                    <div>
                        <label class="label">
                            <span class="label-text">
                                "Brightness: " {move || (value2.get() as f64).round() as i32} "%"
                            </span>
                        </label>
                        <input
                            type="range"
                            min="0"
                            max="100"
                            value=move || value2.get()
                            class="range range-secondary"
                            on:input=move |ev| {
                                let value = event_target_value(&ev).parse::<f64>().unwrap_or(0.0);
                                set_value2.set(value);
                            }
                        />
                    </div>

                    <div>
                        <label class="label">
                            <span class="label-text">
                                "Temperature: " {move || (value3.get() as f64).round() as i32} "°"
                            </span>
                        </label>
                        <input
                            type="range"
                            min="0"
                            max="100"
                            value=move || value3.get()
                            class="range range-accent"
                            on:input=move |ev| {
                                let value = event_target_value(&ev).parse::<f64>().unwrap_or(0.0);
                                set_value3.set(value);
                            }
                        />
                    </div>
                </div>

                <h2 class="text-xl font-semibold">"Disabled Range"</h2>
                <input type="range" min="0" max="100" value="75" class="range" disabled />

                <h2 class="text-xl font-semibold">"Custom Step Range"</h2>
                <div>
                    <label class="text-sm">"Step by 10"</label>
                    <input
                        type="range"
                        min="0"
                        max="100"
                        value="30"
                        class="range range-primary"
                        step="10"
                    />
                    <div class="w-full flex justify-between text-xs px-2">
                        <span>"0"</span>
                        <span>"25"</span>
                        <span>"50"</span>
                        <span>"75"</span>
                        <span>"100"</span>
                    </div>
                </div>

                <h2 class="text-xl font-semibold">"Range in Form"</h2>
                <Card class="bg-base-100 shadow-xl">
                    <CardBody>
                        <h2 class="card-title">"Audio Settings"</h2>
                        <div class="space-y-4">
                            <div class="form-control">
                                <label class="label">
                                    <span class="label-text">"Master Volume"</span>
                                    <span class="label-text-alt">"75%"</span>
                                </label>
                                <input
                                    type="range"
                                    min="0"
                                    max="100"
                                    value="75"
                                    class="range range-primary"
                                />
                            </div>
                            <div class="form-control">
                                <label class="label">
                                    <span class="label-text">"Bass"</span>
                                    <span class="label-text-alt">"50%"</span>
                                </label>
                                <input
                                    type="range"
                                    min="0"
                                    max="100"
                                    value="50"
                                    class="range range-secondary"
                                />
                            </div>
                            <div class="form-control">
                                <label class="label">
                                    <span class="label-text">"Treble"</span>
                                    <span class="label-text-alt">"60%"</span>
                                </label>
                                <input
                                    type="range"
                                    min="0"
                                    max="100"
                                    value="60"
                                    class="range range-accent"
                                />
                            </div>
                        </div>
                        <div class="card-actions justify-end mt-4">
                            <Button color=ButtonColor::Primary>"Apply"</Button>
                        </div>
                    </CardBody>
                </Card>
            </div>
        </div>
    }
}