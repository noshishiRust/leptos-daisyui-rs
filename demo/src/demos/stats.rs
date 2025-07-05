use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn StatsDemo() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <h1 class="text-3xl font-bold">"Stats"</h1>
            <p class="text-base-content/70">
                "Stats component is used to show statistics with numbers and descriptions"
            </p>

            <div class="space-y-4">
                <h2 class="text-xl font-semibold">"Basic Stats"</h2>
                <div class="stats shadow">
                    <div class="stat">
                        <div class="stat-title">"Total Page Views"</div>
                        <div class="stat-value">"89,400"</div>
                        <div class="stat-desc">"21% more than last month"</div>
                    </div>
                </div>

                <h2 class="text-xl font-semibold">"Multiple Stats"</h2>
                <div class="stats shadow">
                    <div class="stat">
                        <div class="stat-figure text-primary">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                fill="none"
                                viewBox="0 0 24 24"
                                class="inline-block w-8 h-8 stroke-current"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"
                                ></path>
                            </svg>
                        </div>
                        <div class="stat-title">"Total Likes"</div>
                        <div class="stat-value text-primary">"25.6K"</div>
                        <div class="stat-desc">"21% more than last month"</div>
                    </div>

                    <div class="stat">
                        <div class="stat-figure text-secondary">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                fill="none"
                                viewBox="0 0 24 24"
                                class="inline-block w-8 h-8 stroke-current"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M13 10V3L4 14h7v7l9-11h-7z"
                                ></path>
                            </svg>
                        </div>
                        <div class="stat-title">"Page Views"</div>
                        <div class="stat-value text-secondary">"2.6M"</div>
                        <div class="stat-desc">"21% more than last month"</div>
                    </div>

                    <div class="stat">
                        <div class="stat-figure text-secondary">
                            <div class="avatar online">
                                <div class="w-16 rounded-full">
                                    <img src="https://picsum.photos/64/64?random=1" alt="User" />
                                </div>
                            </div>
                        </div>
                        <div class="stat-value">"86%"</div>
                        <div class="stat-title">"Tasks done"</div>
                        <div class="stat-desc text-secondary">"31 tasks remaining"</div>
                    </div>
                </div>

                <h2 class="text-xl font-semibold">"Vertical Stats"</h2>
                <div class="stats stats-vertical shadow lg:stats-horizontal">
                    <div class="stat">
                        <div class="stat-title">"Downloads"</div>
                        <div class="stat-value">"31K"</div>
                        <div class="stat-desc">"Jan 1st - Feb 1st"</div>
                    </div>

                    <div class="stat">
                        <div class="stat-title">"New Users"</div>
                        <div class="stat-value">"4,200"</div>
                        <div class="stat-desc">"↗︎ 400 (22%)"</div>
                    </div>

                    <div class="stat">
                        <div class="stat-title">"New Registers"</div>
                        <div class="stat-value">"1,200"</div>
                        <div class="stat-desc">"↘︎ 90 (14%)"</div>
                    </div>
                </div>

                <h2 class="text-xl font-semibold">"Centered Stats"</h2>
                <div class="stats shadow">
                    <div class="stat place-items-center">
                        <div class="stat-title">"Downloads"</div>
                        <div class="stat-value">"31K"</div>
                        <div class="stat-desc">"From January 1st to February 1st"</div>
                    </div>

                    <div class="stat place-items-center">
                        <div class="stat-title">"Users"</div>
                        <div class="stat-value text-secondary">"4,200"</div>
                        <div class="stat-desc text-secondary">"↗︎ 40 (2%)"</div>
                    </div>

                    <div class="stat place-items-center">
                        <div class="stat-title">"New Registers"</div>
                        <div class="stat-value">"1,200"</div>
                        <div class="stat-desc">"↘︎ 90 (14%)"</div>
                    </div>
                </div>

                <h2 class="text-xl font-semibold">"Stats with Icons and Colors"</h2>
                <div class="stats shadow">
                    <div class="stat">
                        <div class="stat-figure text-primary">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                fill="none"
                                viewBox="0 0 24 24"
                                class="inline-block w-8 h-8 stroke-current"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M5 8h14M5 8a2 2 0 110-4h14a2 2 0 110 4M5 8v10a2 2 0 002 2h10a2 2 0 002-2V8m-9 4h4"
                                ></path>
                            </svg>
                        </div>
                        <div class="stat-title">"Account balance"</div>
                        <div class="stat-value">"$89,400"</div>
                        <div class="stat-actions">
                            <Button
                                size=ButtonSize::Sm
                                color=ButtonColor::Success
                            >
                                "Add funds"
                            </Button>
                        </div>
                    </div>

                    <div class="stat">
                        <div class="stat-figure text-secondary">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                fill="none"
                                viewBox="0 0 24 24"
                                class="inline-block w-8 h-8 stroke-current"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 100 4m0-4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 100 4m0-4v2m0-6V4"
                                ></path>
                            </svg>
                        </div>
                        <div class="stat-title">"Current balance"</div>
                        <div class="stat-value">"$89,400"</div>
                        <div class="stat-actions">
                            <Button size=ButtonSize::Sm>"Withdrawal"</Button>
                            <Button
                                size=ButtonSize::Sm
                                color=ButtonColor::Primary
                            >
                                "deposit"
                            </Button>
                        </div>
                    </div>
                </div>

                <h2 class="text-xl font-semibold">"Dashboard Stats"</h2>
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
                    <div class="stats shadow">
                        <div class="stat bg-primary text-primary-content">
                            <div class="stat-figure text-primary-content">
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    fill="none"
                                    viewBox="0 0 24 24"
                                    class="inline-block w-8 h-8 stroke-current"
                                >
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        stroke-width="2"
                                        d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                                    ></path>
                                </svg>
                            </div>
                            <div class="stat-title text-primary-content">"Total Orders"</div>
                            <div class="stat-value">"31K"</div>
                            <div class="stat-desc text-primary-content">"Jan 1st - Feb 1st"</div>
                        </div>
                    </div>

                    <div class="stats shadow">
                        <div class="stat bg-secondary text-secondary-content">
                            <div class="stat-figure text-secondary-content">
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    fill="none"
                                    viewBox="0 0 24 24"
                                    class="inline-block w-8 h-8 stroke-current"
                                >
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        stroke-width="2"
                                        d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 100 4m0-4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 100 4m0-4v2m0-6V4"
                                    ></path>
                                </svg>
                            </div>
                            <div class="stat-title text-secondary-content">"Revenue"</div>
                            <div class="stat-value">"$4,200"</div>
                            <div class="stat-desc text-secondary-content">"↗︎ 400 (22%)"</div>
                        </div>
                    </div>

                    <div class="stats shadow">
                        <div class="stat bg-accent text-accent-content">
                            <div class="stat-figure text-accent-content">
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    fill="none"
                                    viewBox="0 0 24 24"
                                    class="inline-block w-8 h-8 stroke-current"
                                >
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        stroke-width="2"
                                        d="M5 8h14M5 8a2 2 0 110-4h14a2 2 0 110 4M5 8v10a2 2 0 002 2h10a2 2 0 002-2V8m-9 4h4"
                                    ></path>
                                </svg>
                            </div>
                            <div class="stat-title text-accent-content">"New Users"</div>
                            <div class="stat-value">"1,200"</div>
                            <div class="stat-desc text-accent-content">"↘︎ 90 (14%)"</div>
                        </div>
                    </div>

                    <div class="stats shadow">
                        <div class="stat bg-info text-info-content">
                            <div class="stat-figure text-info-content">
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    fill="none"
                                    viewBox="0 0 24 24"
                                    class="inline-block w-8 h-8 stroke-current"
                                >
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        stroke-width="2"
                                        d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"
                                    ></path>
                                </svg>
                            </div>
                            <div class="stat-title text-info-content">"Conversion Rate"</div>
                            <div class="stat-value">"86%"</div>
                            <div class="stat-desc text-info-content">"↗︎ 8% this month"</div>
                        </div>
                    </div>
                </div>

                <h2 class="text-xl font-semibold">"E-commerce Stats"</h2>
                <Card class="bg-base-100 shadow-xl">
                    <CardBody>
                        <h2 class="card-title">"Sales Dashboard"</h2>
                        <div class="stats stats-vertical lg:stats-horizontal shadow">
                            <div class="stat">
                                <div class="stat-figure text-success">
                                    <svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        fill="none"
                                        viewBox="0 0 24 24"
                                        class="inline-block w-8 h-8 stroke-current"
                                    >
                                        <path
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            stroke-width="2"
                                            d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6"
                                        ></path>
                                    </svg>
                                </div>
                                <div class="stat-title">"Sales"</div>
                                <div class="stat-value text-success">"$125,430"</div>
                                <div class="stat-desc">"↗︎ 12% increase"</div>
                            </div>

                            <div class="stat">
                                <div class="stat-figure text-warning">
                                    <svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        fill="none"
                                        viewBox="0 0 24 24"
                                        class="inline-block w-8 h-8 stroke-current"
                                    >
                                        <path
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            stroke-width="2"
                                            d="M3 3h2l.4 2M7 13h10l4-8H5.4m0 0L7 13m0 0l-2.293 2.293a1 1 0 001.414 1.414L10 12M7 13v6a2 2 0 002 2h6a2 2 0 002-2v-6"
                                        ></path>
                                    </svg>
                                </div>
                                <div class="stat-title">"Orders"</div>
                                <div class="stat-value text-warning">"2,847"</div>
                                <div class="stat-desc">"↗︎ 5% increase"</div>
                            </div>

                            <div class="stat">
                                <div class="stat-figure text-error">
                                    <svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        fill="none"
                                        viewBox="0 0 24 24"
                                        class="inline-block w-8 h-8 stroke-current"
                                    >
                                        <path
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            stroke-width="2"
                                            d="M16 11V7a4 4 0 00-8 0v4M5 9h14l1 12H4L5 9z"
                                        ></path>
                                    </svg>
                                </div>
                                <div class="stat-title">"Products"</div>
                                <div class="stat-value text-error">"1,264"</div>
                                <div class="stat-desc">"↘︎ 3% decrease"</div>
                            </div>

                            <div class="stat">
                                <div class="stat-figure text-info">
                                    <svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        fill="none"
                                        viewBox="0 0 24 24"
                                        class="inline-block w-8 h-8 stroke-current"
                                    >
                                        <path
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            stroke-width="2"
                                            d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"
                                        ></path>
                                    </svg>
                                </div>
                                <div class="stat-title">"Customers"</div>
                                <div class="stat-value text-info">"15,672"</div>
                                <div class="stat-desc">"↗︎ 18% increase"</div>
                            </div>
                        </div>
                    </CardBody>
                </Card>

                <h2 class="text-xl font-semibold">"Company Performance"</h2>
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                    <Card class="bg-gradient-to-r from-blue-500 to-blue-600 text-white">
                        <CardBody>
                            <h2 class="card-title text-white">"Q4 Performance"</h2>
                            <div class="stats stats-vertical bg-transparent text-white">
                                <div class="stat bg-transparent">
                                    <div class="stat-title text-blue-100">"Revenue Growth"</div>
                                    <div class="stat-value text-white">"24.8%"</div>
                                    <div class="stat-desc text-blue-200">
                                        "Best quarter this year"
                                    </div>
                                </div>
                                <div class="stat bg-transparent">
                                    <div class="stat-title text-blue-100">"Team Productivity"</div>
                                    <div class="stat-value text-white">"94%"</div>
                                    <div class="stat-desc text-blue-200">
                                        "Above industry average"
                                    </div>
                                </div>
                            </div>
                        </CardBody>
                    </Card>

                    <Card class="bg-gradient-to-r from-green-500 to-green-600 text-white">
                        <CardBody>
                            <h2 class="card-title text-white">"Customer Satisfaction"</h2>
                            <div class="stats stats-vertical bg-transparent text-white">
                                <div class="stat bg-transparent">
                                    <div class="stat-title text-green-100">
                                        "Net Promoter Score"
                                    </div>
                                    <div class="stat-value text-white">"87"</div>
                                    <div class="stat-desc text-green-200">"Excellent rating"</div>
                                </div>
                                <div class="stat bg-transparent">
                                    <div class="stat-title text-green-100">"Support Tickets"</div>
                                    <div class="stat-value text-white">"2.1hrs"</div>
                                    <div class="stat-desc text-green-200">
                                        "Average response time"
                                    </div>
                                </div>
                            </div>
                        </CardBody>
                    </Card>
                </div>
            </div>
        </div>
    }
}