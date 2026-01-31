use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn ColorPickerDemo() -> impl IntoView {
    // State for different color pickers
    let (primary_color, set_primary_color) = signal("#3b82f6".to_string());
    let (secondary_color, set_secondary_color) = signal("#f59e0b".to_string());
    let (background_color, set_background_color) = signal("#1f2937".to_string());
    let (text_color, set_text_color) = signal("#f3f4f6".to_string());

    view! {
        <div class="space-y-8 p-8">
            <div class="space-y-4">
                <h2 class="text-2xl font-bold">"ColorPicker Component"</h2>
                <p class="text-base-content/70">
                    "Color selection interface using HTML5 color input with daisyUI styling."
                </p>
            </div>

            // Live Preview Card
            <div class="card bg-base-200 shadow-xl">
                <div class="card-body">
                    <h3 class="card-title">"Live Color Preview"</h3>
                    <div
                        class="p-8 rounded-lg transition-colors duration-300"
                        style:background-color=move || background_color.get()
                    >
                        <h4
                            class="text-2xl font-bold mb-4 transition-colors duration-300"
                            style:color=move || primary_color.get()
                        >
                            "Sample Heading"
                        </h4>
                        <p
                            class="mb-4 transition-colors duration-300"
                            style:color=move || text_color.get()
                        >
                            "This is sample text that demonstrates the selected colors. "
                            "Change the color pickers below to see live updates."
                        </p>
                        <button
                            class="btn transition-colors duration-300"
                            style:background-color=move || secondary_color.get()
                            style:border-color=move || secondary_color.get()
                            style:color=move || text_color.get()
                        >
                            "Sample Button"
                        </button>
                    </div>
                </div>
            </div>

            // Primary Color Picker
            <div class="space-y-2">
                <h3 class="text-xl font-semibold">"Primary Color"</h3>
                <p class="text-sm text-base-content/70">"Select the primary color for headings"</p>
                <ColorPicker
                    value=Signal::derive(move || primary_color.get())
                    on_change=Callback::new(move |color: String| {
                        set_primary_color.set(color);
                    })

                    show_value=true
                />
            </div>

            // Secondary Color Picker
            <div class="space-y-2">
                <h3 class="text-xl font-semibold">"Secondary Color"</h3>
                <p class="text-sm text-base-content/70">"Select the secondary color for buttons"</p>
                <ColorPicker
                    value=Signal::derive(move || secondary_color.get())
                    on_change=Callback::new(move |color: String| {
                        set_secondary_color.set(color);
                    })

                    show_value=true
                />
            </div>

            // Background Color Picker
            <div class="space-y-2">
                <h3 class="text-xl font-semibold">"Background Color"</h3>
                <p class="text-sm text-base-content/70">"Select the background color"</p>
                <ColorPicker
                    value=Signal::derive(move || background_color.get())
                    on_change=Callback::new(move |color: String| {
                        set_background_color.set(color);
                    })

                    show_value=true
                />
            </div>

            // Text Color Picker
            <div class="space-y-2">
                <h3 class="text-xl font-semibold">"Text Color"</h3>
                <p class="text-sm text-base-content/70">"Select the text color"</p>
                <ColorPicker
                    value=Signal::derive(move || text_color.get())
                    on_change=Callback::new(move |color: String| {
                        set_text_color.set(color);
                    })

                    show_value=true
                />
            </div>

            // Without Value Display
            <div class="space-y-2">
                <h3 class="text-xl font-semibold">"Without Value Display"</h3>
                <ColorPicker
                    value=Signal::derive(|| "#06b6d4".to_string())
                    show_value=false
                    on_change=Callback::new(move |_color: String| {})
                />
            </div>

            // Disabled State
            <div class="space-y-2">
                <h3 class="text-xl font-semibold">"Disabled State"</h3>
                <ColorPicker
                    value=Signal::derive(|| "#9ca3af".to_string())
                    disabled=Signal::derive(|| true)
                    show_value=true
                />
            </div>

            // Color Palette Examples
            <div class="space-y-2">
                <h3 class="text-xl font-semibold">"Common Color Palettes"</h3>
                <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
                    <div class="space-y-2">
                        <p class="text-sm font-semibold">"Blue Palette"</p>
                        <div class="flex gap-2">
                            <div class="w-10 h-10 rounded" style="background-color: #3b82f6"></div>
                            <div class="w-10 h-10 rounded" style="background-color: #60a5fa"></div>
                            <div class="w-10 h-10 rounded" style="background-color: #93c5fd"></div>
                        </div>
                    </div>

                    <div class="space-y-2">
                        <p class="text-sm font-semibold">"Green Palette"</p>
                        <div class="flex gap-2">
                            <div class="w-10 h-10 rounded" style="background-color: #10b981"></div>
                            <div class="w-10 h-10 rounded" style="background-color: #34d399"></div>
                            <div class="w-10 h-10 rounded" style="background-color: #6ee7b7"></div>
                        </div>
                    </div>

                    <div class="space-y-2">
                        <p class="text-sm font-semibold">"Purple Palette"</p>
                        <div class="flex gap-2">
                            <div class="w-10 h-10 rounded" style="background-color: #8b5cf6"></div>
                            <div class="w-10 h-10 rounded" style="background-color: #a78bfa"></div>
                            <div class="w-10 h-10 rounded" style="background-color: #c4b5fd"></div>
                        </div>
                    </div>

                    <div class="space-y-2">
                        <p class="text-sm font-semibold">"Red Palette"</p>
                        <div class="flex gap-2">
                            <div class="w-10 h-10 rounded" style="background-color: #ef4444"></div>
                            <div class="w-10 h-10 rounded" style="background-color: #f87171"></div>
                            <div class="w-10 h-10 rounded" style="background-color: #fca5a5"></div>
                        </div>
                    </div>
                </div>
            </div>

            // Reset Button
            <div class="space-y-2">
                <h3 class="text-xl font-semibold">"Reset to Defaults"</h3>
                <button
                    class="btn btn-primary"
                    on:click=move |_| {
                        set_primary_color.set("#3b82f6".to_string());
                        set_secondary_color.set("#f59e0b".to_string());
                        set_background_color.set("#1f2937".to_string());
                        set_text_color.set("#f3f4f6".to_string());
                    }
                >

                    "Reset All Colors"
                </button>
            </div>

            // Usage Example
            <div class="space-y-2">
                <h3 class="text-xl font-semibold">"Usage Example"</h3>
                <div class="mockup-code text-sm">
                    <pre data-prefix=">">
                        <code>"let (color, set_color) = signal(\"#3b82f6\".to_string());"</code>
                    </pre>
                    <pre data-prefix="">
                        <code></code>
                    </pre>
                    <pre data-prefix=">">
                        <code>"<ColorPicker"</code>
                    </pre>
                    <pre data-prefix=">">
                        <code>"  value=color"</code>
                    </pre>
                    <pre data-prefix=">">
                        <code>"  on_change=Callback::new(move |c| set_color.set(c))"</code>
                    </pre>
                    <pre data-prefix=">">
                        <code>"  show_value=true"</code>
                    </pre>
                    <pre data-prefix=">">
                        <code>"/>"</code>
                    </pre>
                </div>
            </div>

            // Features List
            <div class="space-y-2">
                <h3 class="text-xl font-semibold">"Features"</h3>
                <ul class="list-disc list-inside space-y-1 text-base-content/70">
                    <li>"Native HTML5 color input for broad browser support"</li>
                    <li>"Optional hex value display"</li>
                    <li>"Disabled state support"</li>
                    <li>"Customizable with additional CSS classes"</li>
                    <li>"Callback support for color changes"</li>
                    <li>"Clean daisyUI integration"</li>
                </ul>
            </div>
        </div>
    }
}
