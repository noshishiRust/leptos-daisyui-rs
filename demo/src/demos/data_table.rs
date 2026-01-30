use crate::core::{ContentLayout, Section};
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;
use std::collections::HashMap;

#[component]
pub fn DataTableDemo() -> impl IntoView {
    // Sample user data
    let generate_users = |count: usize| -> Vec<HashMap<&'static str, String>> {
        (0..count)
            .map(|i| {
                HashMap::from([
                    ("id", format!("{:03}", i + 1)),
                    ("name", format!("User {}", i + 1)),
                    ("email", format!("user{}@example.com", i + 1)),
                    ("role", {
                        let roles = ["Admin", "Developer", "Designer", "Manager", "Analyst"];
                        roles[i % 5].to_string()
                    }),
                    ("department", {
                        let depts = ["Engineering", "Sales", "Marketing", "HR", "Finance"];
                        depts[i % 5].to_string()
                    }),
                    ("status", if i % 3 == 0 { "Active" } else { "Inactive" }.to_string()),
                    ("joined", format!("2024-{:02}-{:02}", (i % 12) + 1, (i % 28) + 1)),
                ])
            })
            .collect()
    };

    // Standard columns (store in RwSignal for multiple use)
    let standard_columns = RwSignal::new(vec![
        Column::new("name", "Name"),
        Column::new("email", "Email"),
        Column::new("role", "Role"),
        Column::new("status", "Status"),
        Column::new("joined", "Joined Date"),
    ]);

    // Columns with non-sortable
    let mixed_columns = RwSignal::new(vec![
        Column::new("id", "ID"),
        Column::new("name", "Name"),
        Column::new("email", "Email"),
        Column::new_non_sortable("status", "Status"),
        Column::new("joined", "Joined Date"),
    ]);

    // Data sets (store in RwSignal for multiple use)
    let small_data = RwSignal::new(generate_users(5));
    let medium_data = RwSignal::new(generate_users(25));
    let large_data = RwSignal::new(generate_users(10000));

    // States
    let (loading, set_loading) = signal(false);
    let (page_size, set_page_size) = signal(10_usize);

    view! {
        <ContentLayout
            title="DataTable"
            description="Production-ready data table with sorting, pagination, loading states, and efficient handling of large datasets"
        >
            // Basic Usage
            <Section title="Basic DataTable">
                <p class="text-sm opacity-70 mb-4">
                    "Click column headers to sort. Navigate pages using the controls below."
                </p>
                <DataTable
                    data=small_data
                    columns=standard_columns
                    page_size=3
                />
            </Section>

            // Sortable vs Non-Sortable Columns
            <Section title="Sortable and Non-Sortable Columns">
                <p class="text-sm opacity-70 mb-4">
                    "The 'Status' column is marked as non-sortable and won't respond to clicks."
                </p>
                <DataTable
                    data=small_data
                    columns=mixed_columns
                    page_size=5
                />
            </Section>

            // Table Sizes
            <Section title="Table Size Variants">
                <div class="space-y-6">
                    <div>
                        <h4 class="font-semibold mb-2">"Extra Small (XS)"</h4>
                        <DataTable
                            data=small_data
                            columns=standard_columns
                            page_size=3
                            table_size=TableSize::Xs
                        />
                    </div>

                    <div>
                        <h4 class="font-semibold mb-2">"Small (SM)"</h4>
                        <DataTable
                            data=small_data
                            columns=standard_columns
                            page_size=3
                            table_size=TableSize::Sm
                        />
                    </div>

                    <div>
                        <h4 class="font-semibold mb-2">"Medium (MD - Default)"</h4>
                        <DataTable
                            data=small_data
                            columns=standard_columns
                            page_size=3
                            table_size=TableSize::Md
                        />
                    </div>

                    <div>
                        <h4 class="font-semibold mb-2">"Large (LG)"</h4>
                        <DataTable
                            data=small_data
                            columns=standard_columns
                            page_size=3
                            table_size=TableSize::Lg
                        />
                    </div>
                </div>
            </Section>

            // Styling Options
            <Section title="Styling Options">
                <div class="space-y-6">
                    <div>
                        <h4 class="font-semibold mb-2">"Zebra Striping"</h4>
                        <DataTable
                            data=small_data
                            columns=standard_columns
                            page_size=5
                            zebra=true
                        />
                    </div>

                    <div>
                        <h4 class="font-semibold mb-2">"Pinned Rows (Header stays visible on scroll)"</h4>
                        <div class="max-h-64 overflow-auto">
                            <DataTable
                                data=medium_data
                                columns=standard_columns
                                page_size=50
                                pin_rows=true
                                paginate=false
                            />
                        </div>
                    </div>

                    <div>
                        <h4 class="font-semibold mb-2">"Pinned Columns (First column stays on scroll)"</h4>
                        <div class="overflow-x-auto max-w-md">
                            <DataTable
                                data=small_data
                                columns=mixed_columns
                                page_size=5
                                pin_cols=true
                            />
                        </div>
                    </div>

                    <div>
                        <h4 class="font-semibold mb-2">"Combined: Zebra + Pinned Rows + Small Size"</h4>
                        <DataTable
                            data=small_data
                            columns=standard_columns
                            page_size=5
                            zebra=true
                            pin_rows=true
                            table_size=TableSize::Sm
                        />
                    </div>
                </div>
            </Section>

            // Loading State
            <Section title="Loading State">
                <button
                    class="btn btn-primary btn-sm mb-4"
                    on:click=move |_| set_loading.update(|l| *l = !*l)
                >
                    {move || if loading.get() { "Hide Loading" } else { "Show Loading" }}
                </button>
                <DataTable
                    data=medium_data
                    columns=standard_columns
                    page_size=10
                    loading=loading
                />
            </Section>

            // Empty State
            <Section title="Empty State">
                <p class="text-sm opacity-70 mb-4">
                    "Displays a message when no data is available."
                </p>
                <DataTable
                    data=Signal::derive(Vec::<HashMap<&'static str, String>>::new)
                    columns=standard_columns
                    page_size=10
                />
            </Section>

            // Dynamic Page Size
            <Section title="Dynamic Page Size">
                <div class="mb-4 flex items-center gap-4">
                    <label class="label">
                        <span class="label-text">"Rows per page:"</span>
                    </label>
                    <select
                        class="select select-bordered select-sm"
                        on:change=move |ev| {
                            let value = event_target_value(&ev);
                            if let Ok(size) = value.parse::<usize>() {
                                set_page_size.set(size);
                            }
                        }
                    >
                        <option value="5">"5"</option>
                        <option value="10" selected>"10"</option>
                        <option value="25">"25"</option>
                        <option value="50">"50"</option>
                    </select>
                </div>
                <DataTable
                    data=medium_data
                    columns=standard_columns
                    page_size=page_size
                />
            </Section>

            // No Pagination
            <Section title="Without Pagination">
                <p class="text-sm opacity-70 mb-4">
                    "Disable pagination to show all rows at once (useful for small datasets)."
                </p>
                <DataTable
                    data=small_data
                    columns=standard_columns
                    paginate=false
                />
            </Section>

            // Performance Test
            <Section title="Performance Test: 10,000 Rows">
                <p class="text-sm opacity-70 mb-4">
                    "Efficient index-based operations handle large datasets smoothly. Try sorting by different columns."
                </p>
                <div class="alert alert-info mb-4">
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
                    <div>
                        <p class="font-semibold">"Showing 10,000 rows"</p>
                        <p class="text-sm">
                            "Only the current page's rows are rendered for optimal performance."
                        </p>
                    </div>
                </div>
                <DataTable
                    data=large_data
                    columns=Signal::derive(move || {
                        vec![
                            Column::new("id", "ID"),
                            Column::new("name", "Name"),
                            Column::new("email", "Email"),
                            Column::new("department", "Department"),
                            Column::new("role", "Role"),
                            Column::new_non_sortable("status", "Status"),
                        ]
                    })
                    page_size=50
                    zebra=true
                    table_size=TableSize::Sm
                />
            </Section>

            // Custom Text Strings
            <Section title="Custom Text Strings">
                <p class="text-sm opacity-70 mb-4">
                    "Customize loading, empty, and pagination text for internationalization."
                </p>
                <DataTable
                    data=Signal::derive(Vec::<HashMap<&'static str, String>>::new)
                    columns=standard_columns
                    page_size=10
                    texts=DataTableTexts {
                        loading: "Cargando datos...",
                        empty: "No hay datos disponibles",
                        page_indicator: "Página {current} de {total}",
                        previous: "Anterior",
                        next: "Siguiente",
                    }
                />
            </Section>

            // Code Example
            <Section title="Usage Example">
                <div class="mockup-code text-sm">
                    <pre data-prefix="1">
                        <code>"let columns = vec!["</code>
                    </pre>
                    <pre data-prefix="2">
                        <code>"    Column::new(\"name\", \"Name\"),"</code>
                    </pre>
                    <pre data-prefix="3">
                        <code>"    Column::new(\"email\", \"Email\"),"</code>
                    </pre>
                    <pre data-prefix="4">
                        <code>"    Column::new_non_sortable(\"status\", \"Status\"),"</code>
                    </pre>
                    <pre data-prefix="5">
                        <code>"];"</code>
                    </pre>
                    <pre data-prefix="6">
                        <code>""</code>
                    </pre>
                    <pre data-prefix="7">
                        <code>"let data = vec!["</code>
                    </pre>
                    <pre data-prefix="8">
                        <code>"    HashMap::from(["</code>
                    </pre>
                    <pre data-prefix="9">
                        <code>"        (\"name\", \"Alice\".to_string()),"</code>
                    </pre>
                    <pre data-prefix="10">
                        <code>"        (\"email\", \"alice@example.com\".to_string()),"</code>
                    </pre>
                    <pre data-prefix="11">
                        <code>"        (\"status\", \"Active\".to_string()),"</code>
                    </pre>
                    <pre data-prefix="12">
                        <code>"    ]),"</code>
                    </pre>
                    <pre data-prefix="13">
                        <code>"];"</code>
                    </pre>
                    <pre data-prefix="14">
                        <code>""</code>
                    </pre>
                    <pre data-prefix="15">
                        <code>"view! {"</code>
                    </pre>
                    <pre data-prefix="16">
                        <code>"    <DataTable"</code>
                    </pre>
                    <pre data-prefix="17">
                        <code>"        data=Signal::derive(move || data.clone())"</code>
                    </pre>
                    <pre data-prefix="18">
                        <code>"        columns=Signal::derive(move || columns.clone())"</code>
                    </pre>
                    <pre data-prefix="19">
                        <code>"        page_size=10"</code>
                    </pre>
                    <pre data-prefix="20">
                        <code>"        zebra=true"</code>
                    </pre>
                    <pre data-prefix="21">
                        <code>"    />"</code>
                    </pre>
                    <pre data-prefix="22">
                        <code>"}"</code>
                    </pre>
                </div>
            </Section>
        </ContentLayout>
    }
}
