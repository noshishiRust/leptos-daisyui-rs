use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn AutoCompleteDemo() -> impl IntoView {
    // State for different autocomplete examples
    let (country_value, set_country_value) = signal(String::new());
    let (programming_value, set_programming_value) = signal(String::new());
    let (fruit_value, set_fruit_value) = signal(String::new());
    let (selected_item, set_selected_item) = signal(String::from("None"));

    // Sample data
    let countries = vec![
        "United States".to_string(),
        "United Kingdom".to_string(),
        "Canada".to_string(),
        "Australia".to_string(),
        "Germany".to_string(),
        "France".to_string(),
        "Japan".to_string(),
        "China".to_string(),
        "India".to_string(),
        "Brazil".to_string(),
        "Mexico".to_string(),
        "Spain".to_string(),
        "Italy".to_string(),
        "Netherlands".to_string(),
        "Sweden".to_string(),
    ];

    let programming_languages = vec![
        "Rust".to_string(),
        "JavaScript".to_string(),
        "TypeScript".to_string(),
        "Python".to_string(),
        "Java".to_string(),
        "C++".to_string(),
        "C#".to_string(),
        "Go".to_string(),
        "Ruby".to_string(),
        "Swift".to_string(),
        "Kotlin".to_string(),
        "PHP".to_string(),
        "Scala".to_string(),
    ];

    let fruits = vec![
        "Apple".to_string(),
        "Banana".to_string(),
        "Cherry".to_string(),
        "Date".to_string(),
        "Elderberry".to_string(),
        "Fig".to_string(),
        "Grape".to_string(),
        "Honeydew".to_string(),
        "Kiwi".to_string(),
        "Lemon".to_string(),
        "Mango".to_string(),
        "Orange".to_string(),
        "Papaya".to_string(),
        "Strawberry".to_string(),
    ];

    view! {
        <div class="space-y-8 p-8">
            <div class="space-y-4">
                <h2 class="text-2xl font-bold">"AutoComplete Component"</h2>
                <p class="text-base-content/70">
                    "Input field with auto-completion suggestions displayed in a dropdown."
                </p>
            </div>

            // Current Selection Display
            <div class="alert alert-info">
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
                <span>
                    "Last selected: "
                    <strong>{move || selected_item.get()}</strong>
                </span>
            </div>

            // Basic Example
            <div class="space-y-2">
                <h3 class="text-xl font-semibold">"Basic AutoComplete"</h3>
                <p class="text-sm text-base-content/70">"Search for countries"</p>
                <AutoComplete
                    value=country_value.into()
                    suggestions=Signal::derive(move || countries.clone())
                    placeholder="Type to search countries...".into()
                    on_select=Callback::new(move |value: String| {
                        set_country_value.set(value.clone());
                        set_selected_item.set(format!("Country: {}", value));
                    })

                    on_input=Callback::new(move |value: String| {
                        set_country_value.set(value);
                    })
                />
            </div>

            // Programming Languages Example
            <div class="space-y-2">
                <h3 class="text-xl font-semibold">"Programming Languages"</h3>
                <p class="text-sm text-base-content/70">"Search for your favorite language"</p>
                <AutoComplete
                    value=programming_value.into()
                    suggestions=Signal::derive(move || programming_languages.clone())
                    placeholder="Type to search languages...".into()
                    on_select=Callback::new(move |value: String| {
                        set_programming_value.set(value.clone());
                        set_selected_item.set(format!("Language: {}", value));
                    })

                    on_input=Callback::new(move |value: String| {
                        set_programming_value.set(value);
                    })
                />
            </div>

            // Fruits Example
            <div class="space-y-2">
                <h3 class="text-xl font-semibold">"Fruits AutoComplete"</h3>
                <p class="text-sm text-base-content/70">"Search for fruits"</p>
                <AutoComplete
                    value=fruit_value.into()
                    suggestions=Signal::derive(move || fruits.clone())
                    placeholder="Type to search fruits...".into()
                    on_select=Callback::new(move |value: String| {
                        set_fruit_value.set(value.clone());
                        set_selected_item.set(format!("Fruit: {}", value));
                    })

                    on_input=Callback::new(move |value: String| {
                        set_fruit_value.set(value);
                    })
                />
            </div>

            // Disabled State
            <div class="space-y-2">
                <h3 class="text-xl font-semibold">"Disabled State"</h3>
                <AutoComplete
                    value="Disabled".into()
                    suggestions=Signal::derive(move || vec![])
                    placeholder="This is disabled".into()
                    disabled=true.into()
                />
            </div>

            // Usage Example
            <div class="space-y-2">
                <h3 class="text-xl font-semibold">"Usage Example"</h3>
                <div class="mockup-code text-sm">
                    <pre data-prefix=">">
                        <code>"let (value, set_value) = signal(String::new());"</code>
                    </pre>
                    <pre data-prefix=">">
                        <code>"let suggestions = vec![\"Option 1\".to_string(), ...];"</code>
                    </pre>
                    <pre data-prefix="">
                        <code></code>
                    </pre>
                    <pre data-prefix=">">
                        <code>
                            "<AutoComplete"
                        </code>
                    </pre>
                    <pre data-prefix=">">
                        <code>"  value=value.into()"</code>
                    </pre>
                    <pre data-prefix=">">
                        <code>"  suggestions=Signal::derive(move || suggestions.clone())"</code>
                    </pre>
                    <pre data-prefix=">">
                        <code>"  on_select=Callback::new(move |val| set_value.set(val))"</code>
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
                    <li>"Real-time filtering based on input"</li>
                    <li>"Maximum 10 suggestions displayed"</li>
                    <li>"Case-insensitive matching"</li>
                    <li>"Keyboard navigation support"</li>
                    <li>"Click outside to close"</li>
                    <li>"Customizable callbacks for selection and input changes"</li>
                </ul>
            </div>
        </div>
    }
}
