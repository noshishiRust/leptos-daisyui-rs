use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn TableDemo() -> impl IntoView {
    let users = vec![
        ("John Doe", "john@example.com", "Admin", "Active"),
        ("Jane Smith", "jane@example.com", "User", "Active"),
        ("Bob Wilson", "bob@example.com", "User", "Inactive"),
        ("Alice Brown", "alice@example.com", "Moderator", "Active"),
        ("Charlie Davis", "charlie@example.com", "User", "Pending"),
    ];

    view! {
        <div class="space-y-6">
            <h1 class="text-3xl font-bold">"Table"</h1>
            <p class="text-base-content/70">
                "Tables are used to display data in rows and columns"
            </p>

            <div class="space-y-4">
                <h2 class="text-xl font-semibold">"Basic Table"</h2>
                <div class="overflow-x-auto">
                    <table class="table">
                        <thead>
                            <tr>
                                <th>"Name"</th>
                                <th>"Job"</th>
                                <th>"Company"</th>
                                <th>"Location"</th>
                                <th>"Last Login"</th>
                                <th>"Favorite Color"</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td>"Cy Ganderton"</td>
                                <td>"Quality Control Specialist"</td>
                                <td>"Littel, Schaden and Vandervort"</td>
                                <td>"Canada"</td>
                                <td>"12/16/2020"</td>
                                <td>"Blue"</td>
                            </tr>
                            <tr>
                                <td>"Hart Hagerty"</td>
                                <td>"Desktop Support Technician"</td>
                                <td>"Zemlak, Daniel and Leannon"</td>
                                <td>"United States"</td>
                                <td>"12/5/2020"</td>
                                <td>"Purple"</td>
                            </tr>
                            <tr>
                                <td>"Brice Swyre"</td>
                                <td>"Tax Accountant"</td>
                                <td>"Carroll Group"</td>
                                <td>"China"</td>
                                <td>"8/15/2020"</td>
                                <td>"Red"</td>
                            </tr>
                        </tbody>
                    </table>
                </div>

                <h2 class="text-xl font-semibold">"Table with Visual Elements"</h2>
                <div class="overflow-x-auto">
                    <table class="table">
                        <thead>
                            <tr>
                                <th>"User"</th>
                                <th>"Email"</th>
                                <th>"Role"</th>
                                <th>"Status"</th>
                                <th>"Actions"</th>
                            </tr>
                        </thead>
                        <tbody>
                            {users
                                .into_iter()
                                .enumerate()
                                .map(|(index, (name, email, role, status))| {
                                    view! {
                                        <tr class="hover">
                                            <td>
                                                <div class="flex items-center gap-3">
                                                    <div class="avatar">
                                                        <div class="mask mask-squircle w-12 h-12">
                                                            <img
                                                                src=format!(
                                                                    "https://picsum.photos/48/48?random={}",
                                                                    index + 1,
                                                                )
                                                                alt="Avatar"
                                                            />
                                                        </div>
                                                    </div>
                                                    <div>
                                                        <div class="font-bold">{name}</div>
                                                        <div class="text-sm opacity-50">"United States"</div>
                                                    </div>
                                                </div>
                                            </td>
                                            <td>{email}</td>
                                            <td>
                                                <Badge
                                                    color=Signal::derive(move || match role {
                                                        "Admin" => BadgeColor::Primary,
                                                        "Moderator" => BadgeColor::Secondary,
                                                        _ => BadgeColor::Neutral,
                                                    })
                                                    size=BadgeSize::Sm
                                                >
                                                    {role}
                                                </Badge>
                                            </td>
                                            <td>
                                                <Badge
                                                    color=Signal::derive(move || match status {
                                                        "Active" => BadgeColor::Success,
                                                        "Inactive" => BadgeColor::Error,
                                                        "Pending" => BadgeColor::Warning,
                                                        _ => BadgeColor::Neutral,
                                                    })
                                                    size=BadgeSize::Sm
                                                >
                                                    {status}
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

                <h2 class="text-xl font-semibold">"Zebra Striped Table"</h2>
                <div class="overflow-x-auto">
                    <table class="table table-zebra">
                        <thead>
                            <tr>
                                <th>"#"</th>
                                <th>"Product"</th>
                                <th>"Category"</th>
                                <th>"Price"</th>
                                <th>"Stock"</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <th>"1"</th>
                                <td>"Laptop Pro"</td>
                                <td>"Electronics"</td>
                                <td>"$1,299"</td>
                                <td>"25"</td>
                            </tr>
                            <tr>
                                <th>"2"</th>
                                <td>"Wireless Mouse"</td>
                                <td>"Electronics"</td>
                                <td>"$29"</td>
                                <td>"150"</td>
                            </tr>
                            <tr>
                                <th>"3"</th>
                                <td>"Coffee Mug"</td>
                                <td>"Kitchen"</td>
                                <td>"$12"</td>
                                <td>"88"</td>
                            </tr>
                            <tr>
                                <th>"4"</th>
                                <td>"Office Chair"</td>
                                <td>"Furniture"</td>
                                <td>"$249"</td>
                                <td>"12"</td>
                            </tr>
                        </tbody>
                    </table>
                </div>

                <h2 class="text-xl font-semibold">"Compact Table"</h2>
                <div class="overflow-x-auto">
                    <table class="table table-xs">
                        <thead>
                            <tr>
                                <th>"Name"</th>
                                <th>"Job"</th>
                                <th>"Company"</th>
                                <th>"Location"</th>
                                <th>"Last Login"</th>
                                <th>"Favorite Color"</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td>"Cy Ganderton"</td>
                                <td>"Quality Control Specialist"</td>
                                <td>"Littel, Schaden and Vandervort"</td>
                                <td>"Canada"</td>
                                <td>"12/16/2020"</td>
                                <td>"Blue"</td>
                            </tr>
                            <tr>
                                <td>"Hart Hagerty"</td>
                                <td>"Desktop Support Technician"</td>
                                <td>"Zemlak, Daniel and Leannon"</td>
                                <td>"United States"</td>
                                <td>"12/5/2020"</td>
                                <td>"Purple"</td>
                            </tr>
                        </tbody>
                    </table>
                </div>

                <h2 class="text-xl font-semibold">"Table with Selection"</h2>
                <div class="overflow-x-auto">
                    <table class="table">
                        <thead>
                            <tr>
                                <th>
                                    <label>
                                        <Checkbox />
                                    </label>
                                </th>
                                <th>"Name"</th>
                                <th>"Job"</th>
                                <th>"Favorite Color"</th>
                                <th></th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <th>
                                    <label>
                                        <Checkbox />
                                    </label>
                                </th>
                                <td>
                                    <div class="flex items-center gap-3">
                                        <div class="avatar">
                                            <div class="mask mask-squircle w-12 h-12">
                                                <img
                                                    src="https://picsum.photos/48/48?random=10"
                                                    alt="Avatar"
                                                />
                                            </div>
                                        </div>
                                        <div>
                                            <div class="font-bold">"Hart Hagerty"</div>
                                            <div class="text-sm opacity-50">"United States"</div>
                                        </div>
                                    </div>
                                </td>
                                <td>
                                    "Desktop Support Technician" <br />
                                    <Badge color=BadgeColor::Neutral size=BadgeSize::Sm>
                                        "Desktop Support Technician"
                                    </Badge>
                                </td>
                                <td>"Purple"</td>
                                <th>
                                    <Button size=ButtonSize::Xs style=ButtonStyle::Ghost>
                                        "details"
                                    </Button>
                                </th>
                            </tr>
                            <tr>
                                <th>
                                    <label>
                                        <Checkbox />
                                    </label>
                                </th>
                                <td>
                                    <div class="flex items-center gap-3">
                                        <div class="avatar">
                                            <div class="mask mask-squircle w-12 h-12">
                                                <img
                                                    src="https://picsum.photos/48/48?random=11"
                                                    alt="Avatar"
                                                />
                                            </div>
                                        </div>
                                        <div>
                                            <div class="font-bold">"Brice Swyre"</div>
                                            <div class="text-sm opacity-50">"China"</div>
                                        </div>
                                    </div>
                                </td>
                                <td>
                                    "Tax Accountant" <br />
                                    <Badge color=BadgeColor::Neutral size=BadgeSize::Sm>
                                        "Tax Accountant"
                                    </Badge>
                                </td>
                                <td>"Red"</td>
                                <th>
                                    <Button size=ButtonSize::Xs style=ButtonStyle::Ghost>
                                        "details"
                                    </Button>
                                </th>
                            </tr>
                        </tbody>
                    </table>
                </div>

                <h2 class="text-xl font-semibold">"Responsive Data Table"</h2>
                <Card class="bg-base-100 shadow-xl">
                    <CardBody>
                        <h2 class="card-title">"Sales Report"</h2>
                        <div class="overflow-x-auto">
                            <table class="table table-zebra">
                                <thead>
                                    <tr>
                                        <th>"Month"</th>
                                        <th>"Revenue"</th>
                                        <th>"Orders"</th>
                                        <th>"Avg. Order"</th>
                                        <th>"Growth"</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    <tr>
                                        <td>"January"</td>
                                        <td class="font-mono">"$45,230"</td>
                                        <td>"1,234"</td>
                                        <td class="font-mono">"$36.66"</td>
                                        <td>
                                            <div class="flex items-center gap-1">
                                                <span class="text-success">"↗ 12%"</span>
                                            </div>
                                        </td>
                                    </tr>
                                    <tr>
                                        <td>"February"</td>
                                        <td class="font-mono">"$52,180"</td>
                                        <td>"1,456"</td>
                                        <td class="font-mono">"$35.83"</td>
                                        <td>
                                            <div class="flex items-center gap-1">
                                                <span class="text-success">"↗ 15%"</span>
                                            </div>
                                        </td>
                                    </tr>
                                    <tr>
                                        <td>"March"</td>
                                        <td class="font-mono">"$48,920"</td>
                                        <td>"1,387"</td>
                                        <td class="font-mono">"$35.28"</td>
                                        <td>
                                            <div class="flex items-center gap-1">
                                                <span class="text-error">"↘ 6%"</span>
                                            </div>
                                        </td>
                                    </tr>
                                </tbody>
                            </table>
                        </div>
                    </CardBody>
                </Card>
            </div>
        </div>
    }
}