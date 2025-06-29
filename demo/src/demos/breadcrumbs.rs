use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn BreadcrumbsDemo() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <h1 class="text-3xl font-bold">"Breadcrumbs"</h1>
            <p class="text-base-content/70">
                "Breadcrumbs help users navigate and understand their current location"
            </p>

            <div class="space-y-4">
                <h2 class="text-xl font-semibold">"Basic Breadcrumbs"</h2>
                <Breadcrumbs>
                    <BreadcrumbItem>
                        <a href="#" class="link">
                            "Home"
                        </a>
                    </BreadcrumbItem>
                    <BreadcrumbItem>
                        <a href="#" class="link">
                            "Documents"
                        </a>
                    </BreadcrumbItem>
                    <BreadcrumbItem>"Current Page"</BreadcrumbItem>
                </Breadcrumbs>

                <h2 class="text-xl font-semibold">"With Icons"</h2>
                <Breadcrumbs>
                    <BreadcrumbItem>
                        <a href="#" class="link flex items-center gap-1">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                class="h-4 w-4"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke="currentColor"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2H5a2 2 0 00-2-2V7z"
                                />
                            </svg>
                            "Home"
                        </a>
                    </BreadcrumbItem>
                    <BreadcrumbItem>
                        <a href="#" class="link flex items-center gap-1">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                class="h-4 w-4"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke="currentColor"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
                                />
                            </svg>
                            "Documents"
                        </a>
                    </BreadcrumbItem>
                    <BreadcrumbItem>"File.pdf"</BreadcrumbItem>
                </Breadcrumbs>
            </div>
        </div>
    }
}