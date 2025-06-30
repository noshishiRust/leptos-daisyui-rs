use leptos::prelude::*;

#[component]
pub fn DropdownDemo() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <h1 class="text-3xl font-bold">"Dropdown"</h1>
            <p class="text-base-content/70">
                "Dropdown can open a menu or any other element when the button is clicked"
            </p>

            <div class="space-y-4">
                <h2 class="text-xl font-semibold">"Basic Dropdown"</h2>
                <div class="dropdown">
                    <div tabindex="0" role="button" class="btn m-1">
                        "Click"
                    </div>
                    <ul
                        tabindex="0"
                        class="dropdown-content menu bg-base-100 rounded-box z-[1] w-52 p-2 shadow"
                    >
                        <li>
                            <a>"Item 1"</a>
                        </li>
                        <li>
                            <a>"Item 2"</a>
                        </li>
                    </ul>
                </div>

                <h2 class="text-xl font-semibold">"Dropdown Positions"</h2>
                <div class="flex flex-wrap gap-4">
                    <div class="dropdown">
                        <div tabindex="0" role="button" class="btn m-1">
                            "Top"
                        </div>
                        <ul
                            tabindex="0"
                            class="dropdown-content dropdown-top menu bg-base-100 rounded-box z-[1] w-52 p-2 shadow"
                        >
                            <li>
                                <a>"Item 1"</a>
                            </li>
                            <li>
                                <a>"Item 2"</a>
                            </li>
                        </ul>
                    </div>

                    <div class="dropdown dropdown-end">
                        <div tabindex="0" role="button" class="btn m-1">
                            "End"
                        </div>
                        <ul
                            tabindex="0"
                            class="dropdown-content menu bg-base-100 rounded-box z-[1] w-52 p-2 shadow"
                        >
                            <li>
                                <a>"Item 1"</a>
                            </li>
                            <li>
                                <a>"Item 2"</a>
                            </li>
                        </ul>
                    </div>

                    <div class="dropdown dropdown-left">
                        <div tabindex="0" role="button" class="btn m-1">
                            "Left"
                        </div>
                        <ul
                            tabindex="0"
                            class="dropdown-content menu bg-base-100 rounded-box z-[1] w-52 p-2 shadow"
                        >
                            <li>
                                <a>"Item 1"</a>
                            </li>
                            <li>
                                <a>"Item 2"</a>
                            </li>
                        </ul>
                    </div>

                    <div class="dropdown dropdown-right">
                        <div tabindex="0" role="button" class="btn m-1">
                            "Right"
                        </div>
                        <ul
                            tabindex="0"
                            class="dropdown-content menu bg-base-100 rounded-box z-[1] w-52 p-2 shadow"
                        >
                            <li>
                                <a>"Item 1"</a>
                            </li>
                            <li>
                                <a>"Item 2"</a>
                            </li>
                        </ul>
                    </div>
                </div>

                <h2 class="text-xl font-semibold">"Dropdown with Icons"</h2>
                <div class="dropdown">
                    <div tabindex="0" role="button" class="btn m-1">
                        "Menu"
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="h-5 w-5"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke="currentColor"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M19 9l-7 7-7-7"
                            />
                        </svg>
                    </div>
                    <ul
                        tabindex="0"
                        class="dropdown-content menu bg-base-100 rounded-box z-[1] w-52 p-2 shadow"
                    >
                        <li>
                            <a class="flex items-center gap-2">
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
                                        d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"
                                    />
                                </svg>
                                "Profile"
                            </a>
                        </li>
                        <li>
                            <a class="flex items-center gap-2">
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
                                        d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"
                                    />
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        stroke-width="2"
                                        d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
                                    />
                                </svg>
                                "Settings"
                            </a>
                        </li>
                        <li>
                            <a class="flex items-center gap-2 text-error">
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
                                        d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1"
                                    />
                                </svg>
                                "Logout"
                            </a>
                        </li>
                    </ul>
                </div>
            </div>
        </div>
    }
}