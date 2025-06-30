use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn SkeletonDemo() -> impl IntoView {
    let (loading, set_loading) = signal(true);

    view! {
        <div class="space-y-6">
            <h1 class="text-3xl font-bold">"Skeleton"</h1>
            <p class="text-base-content/70">
                "Skeleton is used to show a placeholder while content is loading"
            </p>

            <div class="space-y-4">
                <h2 class="text-xl font-semibold">"Basic Skeleton"</h2>
                <div class="skeleton h-4 w-28"></div>
                <div class="skeleton h-4 w-full"></div>
                <div class="skeleton h-4 w-full"></div>

                <h2 class="text-xl font-semibold">"Circle Skeleton"</h2>
                <div class="skeleton w-16 h-16 rounded-full shrink-0"></div>

                <h2 class="text-xl font-semibold">"Rectangle Skeleton"</h2>
                <div class="skeleton h-32 w-full"></div>

                <h2 class="text-xl font-semibold">"Card Layout Skeleton"</h2>
                <div class="flex flex-col gap-4">
                    <div class="flex items-center gap-4">
                        <div class="skeleton w-16 h-16 rounded-full shrink-0"></div>
                        <div class="flex flex-col gap-4 flex-1">
                            <div class="skeleton h-4 w-1/2"></div>
                            <div class="skeleton h-4 w-1/4"></div>
                        </div>
                    </div>
                    <div class="skeleton h-32 w-full"></div>
                </div>

                <h2 class="text-xl font-semibold">"Loading vs Content Toggle"</h2>
                <div class="space-y-4">
                    <Button
                        color=Signal::derive(|| ButtonColor::Primary)
                        on:click=move |_| set_loading.update(|loading| *loading = !*loading)
                    >
                        {move || if loading.get() { "Show Content" } else { "Show Loading" }}
                    </Button>

                    <Card class="bg-base-100 shadow-xl">
                        <CardBody>
                            {move || {
                                if loading.get() {
                                    view! {
                                        <div class="flex items-center gap-4 mb-4">
                                            <div class="skeleton w-12 h-12 rounded-full shrink-0"></div>
                                            <div class="flex flex-col gap-2 flex-1">
                                                <div class="skeleton h-4 w-1/3"></div>
                                                <div class="skeleton h-3 w-1/4"></div>
                                            </div>
                                        </div>
                                        <div class="skeleton h-4 w-full mb-2"></div>
                                        <div class="skeleton h-4 w-full mb-2"></div>
                                        <div class="skeleton h-4 w-3/4 mb-4"></div>
                                        <div class="skeleton h-8 w-24"></div>
                                    }
                                        .into_any()
                                } else {
                                    view! {
                                        <div class="flex items-center gap-4 mb-4">
                                            <div class="avatar">
                                                <div class="w-12 rounded-full">
                                                    <img
                                                        src="https://picsum.photos/48/48?random=1"
                                                        alt="Avatar"
                                                    />
                                                </div>
                                            </div>
                                            <div>
                                                <h3 class="font-bold">"John Doe"</h3>
                                                <p class="text-sm text-base-content/70">"2 hours ago"</p>
                                            </div>
                                        </div>
                                        <p class="mb-4">
                                            "This is the actual content that appears after loading is complete. The skeleton was just a placeholder to show the structure while data was being fetched."
                                        </p>
                                        <Button
                                            size=Signal::derive(|| ButtonSize::Sm)
                                            color=Signal::derive(|| ButtonColor::Primary)
                                        >
                                            "Read More"
                                        </Button>
                                    }
                                        .into_any()
                                }
                            }}
                        </CardBody>
                    </Card>
                </div>

                <h2 class="text-xl font-semibold">"Article Layout Skeleton"</h2>
                <div class="space-y-4">
                    <div class="skeleton h-8 w-3/4"></div>
                    <div class="flex items-center gap-2">
                        <div class="skeleton w-8 h-8 rounded-full shrink-0"></div>
                        <div class="skeleton h-4 w-32"></div>
                        <div class="skeleton h-4 w-20"></div>
                    </div>
                    <div class="skeleton h-48 w-full"></div>
                    <div class="space-y-2">
                        <div class="skeleton h-4 w-full"></div>
                        <div class="skeleton h-4 w-full"></div>
                        <div class="skeleton h-4 w-full"></div>
                        <div class="skeleton h-4 w-3/4"></div>
                    </div>
                </div>

                <h2 class="text-xl font-semibold">"Product Grid Skeleton"</h2>
                <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                    {(0..6)
                        .map(|_| {
                            view! {
                                <div class="space-y-4">
                                    <div class="skeleton h-48 w-full"></div>
                                    <div class="skeleton h-4 w-3/4"></div>
                                    <div class="skeleton h-4 w-1/2"></div>
                                    <div class="skeleton h-6 w-1/4"></div>
                                </div>
                            }
                        })
                        .collect::<Vec<_>>()}
                </div>

                <h2 class="text-xl font-semibold">"Chat Message Skeleton"</h2>
                <div class="space-y-4">
                    <div class="chat chat-start">
                        <div class="chat-image avatar">
                            <div class="skeleton w-10 h-10 rounded-full"></div>
                        </div>
                        <div class="chat-header">
                            <div class="skeleton h-3 w-16"></div>
                        </div>
                        <div class="chat-bubble">
                            <div class="skeleton h-4 w-32"></div>
                        </div>
                    </div>

                    <div class="chat chat-end">
                        <div class="chat-image avatar">
                            <div class="skeleton w-10 h-10 rounded-full"></div>
                        </div>
                        <div class="chat-header">
                            <div class="skeleton h-3 w-20"></div>
                        </div>
                        <div class="chat-bubble">
                            <div class="skeleton h-4 w-48"></div>
                        </div>
                    </div>
                </div>

                <h2 class="text-xl font-semibold">"Table Row Skeleton"</h2>
                <div class="overflow-x-auto">
                    <table class="table">
                        <thead>
                            <tr>
                                <th>"Name"</th>
                                <th>"Email"</th>
                                <th>"Status"</th>
                                <th>"Actions"</th>
                            </tr>
                        </thead>
                        <tbody>
                            {(0..5)
                                .map(|_| {
                                    view! {
                                        <tr>
                                            <td>
                                                <div class="flex items-center gap-3">
                                                    <div class="skeleton w-8 h-8 rounded-full shrink-0"></div>
                                                    <div class="skeleton h-4 w-24"></div>
                                                </div>
                                            </td>
                                            <td>
                                                <div class="skeleton h-4 w-32"></div>
                                            </td>
                                            <td>
                                                <div class="skeleton h-6 w-16 rounded-full"></div>
                                            </td>
                                            <td>
                                                <div class="skeleton h-8 w-20"></div>
                                            </td>
                                        </tr>
                                    }
                                })
                                .collect::<Vec<_>>()}
                        </tbody>
                    </table>
                </div>

                <h2 class="text-xl font-semibold">"Form Skeleton"</h2>
                <Card class="bg-base-100 shadow-xl">
                    <CardBody>
                        <div class="skeleton h-6 w-48 mb-4"></div>
                        <div class="space-y-4">
                            <div>
                                <div class="skeleton h-4 w-16 mb-2"></div>
                                <div class="skeleton h-12 w-full"></div>
                            </div>
                            <div>
                                <div class="skeleton h-4 w-20 mb-2"></div>
                                <div class="skeleton h-12 w-full"></div>
                            </div>
                            <div>
                                <div class="skeleton h-4 w-24 mb-2"></div>
                                <div class="skeleton h-32 w-full"></div>
                            </div>
                            <div class="flex gap-2 justify-end">
                                <div class="skeleton h-10 w-20"></div>
                                <div class="skeleton h-10 w-24"></div>
                            </div>
                        </div>
                    </CardBody>
                </Card>

                <h2 class="text-xl font-semibold">"Dashboard Widget Skeleton"</h2>
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
                    {(0..4)
                        .map(|_| {
                            view! {
                                <div class="bg-base-100 p-6 rounded-lg shadow">
                                    <div class="flex items-center justify-between mb-4">
                                        <div class="skeleton h-4 w-16"></div>
                                        <div class="skeleton w-8 h-8 rounded"></div>
                                    </div>
                                    <div class="skeleton h-8 w-20 mb-2"></div>
                                    <div class="skeleton h-3 w-24"></div>
                                </div>
                            }
                        })
                        .collect::<Vec<_>>()}
                </div>
            </div>
        </div>
    }
}