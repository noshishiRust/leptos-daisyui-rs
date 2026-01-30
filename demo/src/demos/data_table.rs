use leptos::prelude::*;
use leptos_daisyui_rs::components::*;
use std::collections::HashMap;

#[component]
pub fn DataTableDemo() -> impl IntoView {
    // Sample data (create signal to avoid move issues)
    let sample_data = vec![
        HashMap::from([
            ("name", "Alice Johnson".to_string()),
            ("email", "alice@example.com".to_string()),
            ("role", "Admin".to_string()),
            ("status", "Active".to_string()),
            ("joined", "2024-01-15".to_string()),
        ]),
        HashMap::from([
            ("name", "Bob Smith".to_string()),
            ("email", "bob@example.com".to_string()),
            ("role", "Developer".to_string()),
            ("status", "Active".to_string()),
            ("joined", "2024-02-20".to_string()),
        ]),
        HashMap::from([
            ("name", "Carol Williams".to_string()),
            ("email", "carol@example.com".to_string()),
            ("role", "Designer".to_string()),
            ("status", "Inactive".to_string()),
            ("joined", "2024-03-10".to_string()),
        ]),
        HashMap::from([
            ("name", "David Brown".to_string()),
            ("email", "david@example.com".to_string()),
            ("role", "Manager".to_string()),
            ("status", "Active".to_string()),
            ("joined", "2023-12-01".to_string()),
        ]),
        HashMap::from([
            ("name", "Eve Davis".to_string()),
            ("email", "eve@example.com".to_string()),
            ("role", "Developer".to_string()),
            ("status", "Active".to_string()),
            ("joined", "2024-04-05".to_string()),
        ]),
    ];

    // Column definitions
    let columns = vec![
        Column::new("name", "Name"),
        Column::new("email", "Email"),
        Column::new("role", "Role"),
        Column::new_non_sortable("status", "Status"),
        Column::new("joined", "Joined"),
    ];

    // Clone data for use in multiple closures
    let data_for_basic = sample_data.clone();
    let data_for_zebra = sample_data.clone();
    let data_for_loading = sample_data.clone();
    let data_for_custom = sample_data.clone();

    let cols_for_basic = columns.clone();
    let cols_for_zebra = columns.clone();
    let cols_for_loading = columns.clone();
    let cols_for_empty = columns.clone();
    let cols_for_custom = columns.clone();

    // States
    let (loading, set_loading) = signal(false);

    view! {
        <div class="space-y-6">
            <h1 class="text-3xl font-bold">"DataTable"</h1>
            <p class="text-base-content/70">
                "Production-ready data table with sorting, pagination, and loading states"
            </p>

            <div class="space-y-8">
                // Basic DataTable
                <div class="space-y-4">
                    <h2 class="text-xl font-semibold">"Basic DataTable"</h2>
                    <DataTable
                        data=Signal::derive(move || data_for_basic.clone())
                        columns=Signal::derive(move || cols_for_basic.clone())
                        page_size=3
                    />
                </div>

                // With Zebra Striping
                <div class="space-y-4">
                    <h2 class="text-xl font-semibold">"With Zebra Striping"</h2>
                    <DataTable
                        data=Signal::derive(move || data_for_zebra.clone())
                        columns=Signal::derive(move || cols_for_zebra.clone())
                        page_size=5
                        zebra=true
                    />
                </div>

                // Loading State
                <div class="space-y-4">
                    <h2 class="text-xl font-semibold">"Loading State"</h2>
                    <button
                        class="btn btn-sm btn-primary mb-2"
                        on:click=move |_| {
                            set_loading.set(!loading.get());
                        }
                    >
                        "Toggle Loading"
                    </button>
                    <DataTable
                        data=Signal::derive(move || data_for_loading.clone())
                        columns=Signal::derive(move || cols_for_loading.clone())
                        page_size=5
                        loading=Signal::derive(move || loading.get())
                    />
                </div>

                // Empty State
                <div class="space-y-4">
                    <h2 class="text-xl font-semibold">"Empty State"</h2>
                    <DataTable
                        data=Signal::derive(move || Vec::<HashMap<&'static str, String>>::new())
                        columns=Signal::derive(move || cols_for_empty.clone())
                        page_size=5
                    />
                </div>

                // Custom Styling
                <div class="space-y-4">
                    <h2 class="text-xl font-semibold">"Custom Styling (Small Table)"</h2>
                    <DataTable
                        data=Signal::derive(move || data_for_custom.clone())
                        columns=Signal::derive(move || cols_for_custom.clone())
                        page_size=3
                        table_size=TableSize::Sm
                        zebra=true
                        pin_rows=true
                    />
                </div>

                // Performance Test with 10,000 Rows
                <div class="space-y-4">
                    <h2 class="text-xl font-semibold">"Performance Test (10,000 rows)"</h2>
                    <p class="text-sm text-base-content/70">
                        "Click column headers to sort. Pagination handles large datasets efficiently."
                    </p>
                    <LargeDataTableDemo />
                </div>
            </div>
        </div>
    }
}

#[component]
fn LargeDataTableDemo() -> impl IntoView {
    // Generate 10,000 rows
    let large_data: Vec<HashMap<&'static str, String>> = (0..10000)
        .map(|i| {
            HashMap::from([
                ("id", format!("ID-{:05}", i)),
                ("name", format!("User {}", i)),
                ("email", format!("user{}@example.com", i)),
                ("department", {
                    let depts = ["Engineering", "Sales", "Marketing", "HR", "Finance"];
                    depts[i % 5].to_string()
                }),
                ("score", format!("{}", (i * 17) % 100)),
            ])
        })
        .collect();

    let large_columns = vec![
        Column::new("id", "ID"),
        Column::new("name", "Name"),
        Column::new("email", "Email"),
        Column::new("department", "Department"),
        Column::new("score", "Score"),
    ];

    view! {
        <DataTable
            data=Signal::derive(move || large_data.clone())
            columns=Signal::derive(move || large_columns.clone())
            page_size=50
            zebra=true
            table_size=TableSize::Sm
        />
    }
}
