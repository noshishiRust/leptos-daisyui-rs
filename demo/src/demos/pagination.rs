use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn PaginationDemo() -> impl IntoView {
    let current_page = RwSignal::new(3);
    let total_pages = 10;

    view! {
        <div class="space-y-6">
            <h1 class="text-3xl font-bold">"Pagination"</h1>
            <p class="text-base-content/70">
                "Pagination component is used for displaying page numbers and navigation controls"
            </p>

            <div class="space-y-4">
                <h2 class="text-xl font-semibold">"Basic Pagination"</h2>
                <div class="join">
                    <Button class="join-item" style=ButtonStyle::Outline>
                        "«"
                    </Button>
                    <Button class="join-item" style=ButtonStyle::Outline>
                        "Page 22"
                    </Button>
                    <Button class="join-item" style=ButtonStyle::Outline>
                        "»"
                    </Button>
                </div>

                <h2 class="text-xl font-semibold">"Numbered Pagination"</h2>
                <div class="join">
                    <Button class="join-item" style=ButtonStyle::Outline>
                        "1"
                    </Button>
                    <Button class="join-item" style=ButtonStyle::Outline>
                        "2"
                    </Button>
                    <Button class="join-item btn-active" color=ButtonColor::Primary>
                        "3"
                    </Button>
                    <Button class="join-item" style=ButtonStyle::Outline>
                        "4"
                    </Button>
                    <Button class="join-item" style=ButtonStyle::Outline>
                        "5"
                    </Button>
                </div>

                <h2 class="text-xl font-semibold">"Pagination with Disabled States"</h2>
                <div class="join">
                    <Button class="join-item" style=ButtonStyle::Outline disabled=true>
                        "«"
                    </Button>
                    <Button class="join-item" style=ButtonStyle::Outline>
                        "Page 1 of 10"
                    </Button>
                    <Button class="join-item" style=ButtonStyle::Outline>
                        "»"
                    </Button>
                </div>

                <h2 class="text-xl font-semibold">"Pagination Sizes"</h2>
                <div class="space-y-4">
                    <div>
                        <p class="text-sm font-medium mb-2">"Extra Small"</p>
                        <div class="join">
                            <Button
                                class="join-item"
                                size=ButtonSize::Xs
                                style=ButtonStyle::Outline
                            >
                                "1"
                            </Button>
                            <Button
                                class="join-item btn-active"
                                size=ButtonSize::Xs
                                color=ButtonColor::Primary
                            >
                                "2"
                            </Button>
                            <Button
                                class="join-item"
                                size=ButtonSize::Xs
                                style=ButtonStyle::Outline
                            >
                                "3"
                            </Button>
                            <Button
                                class="join-item"
                                size=ButtonSize::Xs
                                style=ButtonStyle::Outline
                            >
                                "4"
                            </Button>
                        </div>
                    </div>

                    <div>
                        <p class="text-sm font-medium mb-2">"Small"</p>
                        <div class="join">
                            <Button
                                class="join-item"
                                size=ButtonSize::Sm
                                style=ButtonStyle::Outline
                            >
                                "1"
                            </Button>
                            <Button
                                class="join-item btn-active"
                                size=ButtonSize::Sm
                                color=ButtonColor::Primary
                            >
                                "2"
                            </Button>
                            <Button
                                class="join-item"
                                size=ButtonSize::Sm
                                style=ButtonStyle::Outline
                            >
                                "3"
                            </Button>
                            <Button
                                class="join-item"
                                size=ButtonSize::Sm
                                style=ButtonStyle::Outline
                            >
                                "4"
                            </Button>
                        </div>
                    </div>

                    <div>
                        <p class="text-sm font-medium mb-2">"Large"</p>
                        <div class="join">
                            <Button
                                class="join-item"
                                size=ButtonSize::Lg
                                style=ButtonStyle::Outline
                            >
                                "1"
                            </Button>
                            <Button
                                class="join-item btn-active"
                                size=ButtonSize::Lg
                                color=ButtonColor::Primary
                            >
                                "2"
                            </Button>
                            <Button
                                class="join-item"
                                size=ButtonSize::Lg
                                style=ButtonStyle::Outline
                            >
                                "3"
                            </Button>
                            <Button
                                class="join-item"
                                size=ButtonSize::Lg
                                style=ButtonStyle::Outline
                            >
                                "4"
                            </Button>
                        </div>
                    </div>
                </div>

                <h2 class="text-xl font-semibold">"Responsive Pagination"</h2>
                <div class="join">
                    <Button class="join-item" style=ButtonStyle::Outline>
                        "First"
                    </Button>
                    <Button class="join-item" style=ButtonStyle::Outline>
                        "Prev"
                    </Button>
                    <Button class="join-item hidden sm:inline-flex" style=ButtonStyle::Outline>
                        "1"
                    </Button>
                    <Button class="join-item hidden sm:inline-flex" style=ButtonStyle::Outline>
                        "2"
                    </Button>
                    <Button class="join-item btn-active" color=ButtonColor::Primary>
                        "3"
                    </Button>
                    <Button class="join-item hidden sm:inline-flex" style=ButtonStyle::Outline>
                        "4"
                    </Button>
                    <Button class="join-item hidden sm:inline-flex" style=ButtonStyle::Outline>
                        "5"
                    </Button>
                    <Button class="join-item" style=ButtonStyle::Outline>
                        "Next"
                    </Button>
                    <Button class="join-item" style=ButtonStyle::Outline>
                        "Last"
                    </Button>
                </div>

                <h2 class="text-xl font-semibold">"Interactive Pagination"</h2>
                <Card class="bg-base-100 shadow-xl">
                    <CardBody>
                        <div class="flex items-center justify-between mb-4">
                            <h2 class="card-title">"Product List"</h2>
                            <div class="text-sm text-base-content/70">
                                "Showing " {move || (current_page.get() - 1) * 10 + 1} " to "
                                {move || current_page.get() * 10} " of 100 results"
                            </div>
                        </div>

                        <div class="space-y-2">
                            {(1..=10)
                                .map(|i| {
                                    let product_number = move || (current_page.get() - 1) * 10 + i;
                                    view! {
                                        <div class="p-3 bg-base-200 rounded-lg flex justify-between items-center">
                                            <span>"Product #" {product_number}</span>
                                            <Badge color=BadgeColor::Primary size=BadgeSize::Sm>
                                                "In Stock"
                                            </Badge>
                                        </div>
                                    }
                                })
                                .collect::<Vec<_>>()}
                        </div>

                        <div class="flex justify-center mt-6">
                            <div class="join">
                                <Button
                                    class="join-item"
                                    style=ButtonStyle::Outline
                                    disabled=Signal::derive(move || current_page.get() == 1)
                                    on:click=move |_| {
                                        if current_page.get() > 1 {
                                            current_page.update(|p| *p -= 1);
                                        }
                                    }
                                >
                                    "Previous"
                                </Button>

                                {(1..=total_pages)
                                    .map(|page| {
                                        view! {
                                            <Button
                                                class=move || if current_page.get() == page {
                                                    "join-item btn-active"
                                                } else {
                                                    "join-item"
                                                }
                                                color=Signal::derive(move || {
                                                    if current_page.get() == page {
                                                        ButtonColor::Primary
                                                    } else {
                                                        ButtonColor::Neutral
                                                    }
                                                })
                                                style=Signal::derive(move || {
                                                    if current_page.get() == page {
                                                        ButtonStyle::default()
                                                    } else {
                                                        ButtonStyle::Outline
                                                    }
                                                })
                                                on:click=move |_| current_page.set(page)
                                            >
                                                {page.to_string()}
                                            </Button>
                                        }
                                    })
                                    .collect::<Vec<_>>()}

                                <Button
                                    class="join-item"
                                    style=ButtonStyle::Outline
                                    disabled=Signal::derive(move || {
                                        current_page.get() == total_pages
                                    })
                                    on:click=move |_| {
                                        if current_page.get() < total_pages {
                                            current_page.update(|p| *p += 1);
                                        }
                                    }
                                >
                                    "Next"
                                </Button>
                            </div>
                        </div>
                    </CardBody>
                </Card>

                <h2 class="text-xl font-semibold">"Pagination with Ellipsis"</h2>
                <div class="join">
                    <Button class="join-item" style=ButtonStyle::Outline>
                        "1"
                    </Button>
                    <Button class="join-item" style=ButtonStyle::Outline>
                        "2"
                    </Button>
                    <Button class="join-item" style=ButtonStyle::Outline>
                        "3"
                    </Button>
                    <Button class="join-item btn-disabled" disabled=true>
                        "..."
                    </Button>
                    <Button class="join-item" style=ButtonStyle::Outline>
                        "97"
                    </Button>
                    <Button class="join-item" style=ButtonStyle::Outline>
                        "98"
                    </Button>
                    <Button class="join-item btn-active" color=ButtonColor::Primary>
                        "99"
                    </Button>
                    <Button class="join-item" style=ButtonStyle::Outline>
                        "100"
                    </Button>
                </div>

                <h2 class="text-xl font-semibold">"Table with Pagination"</h2>
                <Card class="bg-base-100 shadow-xl">
                    <CardBody>
                        <h2 class="card-title">"User Management"</h2>

                        <div class="overflow-x-auto">
                            <table class="table table-zebra">
                                <thead>
                                    <tr>
                                        <th>"ID"</th>
                                        <th>"Name"</th>
                                        <th>"Email"</th>
                                        <th>"Role"</th>
                                        <th>"Actions"</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {(1..=5)
                                        .map(|i| {
                                            let user_id = move || (current_page.get() - 1) * 5 + i;
                                            let names = [
                                                "Alice Johnson",
                                                "Bob Smith",
                                                "Carol Brown",
                                                "David Wilson",
                                                "Eva Davis",
                                            ];
                                            let name = names
                                                .get((i - 1) % names.len())
                                                .unwrap_or(&"Unknown");
                                            view! {
                                                <tr>
                                                    <td>{user_id}</td>
                                                    <td>{*name}</td>
                                                    <td>
                                                        {format!(
                                                            "{}@example.com",
                                                            name
                                                                .split_whitespace()
                                                                .next()
                                                                .unwrap_or("user")
                                                                .to_lowercase(),
                                                        )}
                                                    </td>
                                                    <td>
                                                        <Badge
                                                            color=Signal::derive(move || {
                                                                if i % 3 == 0 {
                                                                    BadgeColor::Primary
                                                                } else if i % 2 == 0 {
                                                                    BadgeColor::Secondary
                                                                } else {
                                                                    BadgeColor::Accent
                                                                }
                                                            })
                                                            size=BadgeSize::Sm
                                                        >
                                                            {if i % 3 == 0 {
                                                                "Admin"
                                                            } else if i % 2 == 0 {
                                                                "Editor"
                                                            } else {
                                                                "User"
                                                            }}
                                                        </Badge>
                                                    </td>
                                                    <td>
                                                        <div class="flex gap-1">
                                                            <Button size=ButtonSize::Xs style=ButtonStyle::Ghost>
                                                                "Edit"
                                                            </Button>
                                                            <Button
                                                                size=ButtonSize::Xs
                                                                color=ButtonColor::Error
                                                                style=ButtonStyle::Ghost
                                                            >
                                                                "Delete"
                                                            </Button>
                                                        </div>
                                                    </td>
                                                </tr>
                                            }
                                        })
                                        .collect::<Vec<_>>()}
                                </tbody>
                            </table>
                        </div>

                        <div class="flex justify-between items-center mt-4">
                            <div class="text-sm text-base-content/70">
                                "Showing " {move || (current_page.get() - 1) * 5 + 1} " to "
                                {move || current_page.get() * 5} " of 50 entries"
                            </div>

                            <div class="join">
                                <Button
                                    class="join-item"
                                    size=ButtonSize::Sm
                                    style=ButtonStyle::Outline
                                    disabled=Signal::derive(move || current_page.get() == 1)
                                    on:click=move |_| {
                                        if current_page.get() > 1 {
                                            current_page.update(|p| *p -= 1);
                                        }
                                    }
                                >
                                    "‹"
                                </Button>

                                {move || {
                                    (1..=total_pages)
                                        .filter(|&page| {
                                            let current = current_page.get();
                                            page == 1 || page == total_pages
                                                || (page >= current - 1 && page <= current + 1)
                                        })
                                        .enumerate()
                                        .map(|(idx, page)| {
                                            let prev_page = if idx > 0 {
                                                (1..=total_pages)
                                                    .filter(|&p| {
                                                        let current = current_page.get();
                                                        p == 1 || p == total_pages
                                                            || (p >= current - 1 && p <= current + 1)
                                                    })
                                                    .nth(idx - 1)
                                                    .unwrap_or(1)
                                            } else {
                                                1
                                            };

                                        view! {
                                            <>
                                                {if page > prev_page + 1 && idx > 0 {
                                                    Some(
                                                        view! {
                                                            <Button
                                                                class="join-item btn-disabled"
                                                                size=ButtonSize::Sm
                                                                disabled=true
                                                            >
                                                                "..."
                                                            </Button>
                                                        },
                                                    )
                                                } else {
                                                    None
                                                }}
                                                <Button
                                                    class=move || if current_page.get() == page {
                                                        "join-item btn-active"
                                                    } else {
                                                        "join-item"
                                                    }
                                                    size=ButtonSize::Sm
                                                    color=Signal::derive(move || {
                                                        if current_page.get() == page {
                                                            ButtonColor::Primary
                                                        } else {
                                                            ButtonColor::Neutral
                                                        }
                                                    })
                                                    style=Signal::derive(move || {
                                                        if current_page.get() == page {
                                                            ButtonStyle::default()
                                                        } else {
                                                            ButtonStyle::Outline
                                                        }
                                                    })
                                                    on:click=move |_| current_page.set(page)
                                                >
                                                    {page.to_string()}
                                                </Button>
                                            </>
                                        }
                                    })
                                    .collect::<Vec<_>>()
                                }}

                                <Button
                                    class="join-item"
                                    size=ButtonSize::Sm
                                    style=ButtonStyle::Outline
                                    disabled=Signal::derive(move || {
                                        current_page.get() == total_pages
                                    })
                                    on:click=move |_| {
                                        if current_page.get() < total_pages {
                                            current_page.update(|p| *p += 1);
                                        }
                                    }
                                >
                                    "›"
                                </Button>
                            </div>
                        </div>
                    </CardBody>
                </Card>

                <h2 class="text-xl font-semibold">"Colored Pagination"</h2>
                <div class="space-y-4">
                    <div>
                        <p class="text-sm font-medium mb-2">"Primary"</p>
                        <div class="join">
                            <Button class="join-item" style=ButtonStyle::Outline>
                                "1"
                            </Button>
                            <Button class="join-item btn-active" color=ButtonColor::Primary>
                                "2"
                            </Button>
                            <Button class="join-item" style=ButtonStyle::Outline>
                                "3"
                            </Button>
                        </div>
                    </div>

                    <div>
                        <p class="text-sm font-medium mb-2">"Secondary"</p>
                        <div class="join">
                            <Button class="join-item" style=ButtonStyle::Outline>
                                "1"
                            </Button>
                            <Button class="join-item btn-active" color=ButtonColor::Secondary>
                                "2"
                            </Button>
                            <Button class="join-item" style=ButtonStyle::Outline>
                                "3"
                            </Button>
                        </div>
                    </div>

                    <div>
                        <p class="text-sm font-medium mb-2">"Accent"</p>
                        <div class="join">
                            <Button class="join-item" style=ButtonStyle::Outline>
                                "1"
                            </Button>
                            <Button class="join-item btn-active" color=ButtonColor::Accent>
                                "2"
                            </Button>
                            <Button class="join-item" style=ButtonStyle::Outline>
                                "3"
                            </Button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
