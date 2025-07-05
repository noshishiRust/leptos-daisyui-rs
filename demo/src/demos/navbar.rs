use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn NavbarDemo() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <h1 class="text-3xl font-bold">"Navbar"</h1>
            <p class="text-base-content/70">
                "Navbar is used to show navigation links and information at the top of a page"
            </p>

            <div class="space-y-4">
                <h2 class="text-xl font-semibold">"Basic Navbar"</h2>
                <div class="navbar bg-base-100">
                    <a class="btn btn-ghost text-xl">"daisyUI"</a>
                </div>

                <h2 class="text-xl font-semibold">"Navbar with Menu"</h2>
                <div class="navbar bg-base-100">
                    <div class="navbar-start">
                        <div class="dropdown">
                            <div tabindex="0" role="button" class="btn btn-ghost lg:hidden">
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
                                        d="M4 6h16M4 12h8m-8 6h16"
                                    />
                                </svg>
                            </div>
                            <ul
                                tabindex="0"
                                class="menu menu-sm dropdown-content mt-3 z-[1] p-2 shadow bg-base-100 rounded-box w-52"
                            >
                                <li>
                                    <a>"Item 1"</a>
                                </li>
                                <li>
                                    <a>"Parent"</a>
                                    <ul class="p-2">
                                        <li>
                                            <a>"Submenu 1"</a>
                                        </li>
                                        <li>
                                            <a>"Submenu 2"</a>
                                        </li>
                                    </ul>
                                </li>
                                <li>
                                    <a>"Item 3"</a>
                                </li>
                            </ul>
                        </div>
                        <a class="btn btn-ghost text-xl">"daisyUI"</a>
                    </div>
                    <div class="navbar-center hidden lg:flex">
                        <ul class="menu menu-horizontal px-1">
                            <li>
                                <a>"Item 1"</a>
                            </li>
                            <li>
                                <details>
                                    <summary>"Parent"</summary>
                                    <ul class="p-2">
                                        <li>
                                            <a>"Submenu 1"</a>
                                        </li>
                                        <li>
                                            <a>"Submenu 2"</a>
                                        </li>
                                    </ul>
                                </details>
                            </li>
                            <li>
                                <a>"Item 3"</a>
                            </li>
                        </ul>
                    </div>
                    <div class="navbar-end">
                        <a class="btn">"Button"</a>
                    </div>
                </div>

                <h2 class="text-xl font-semibold">"Navbar with Icons and Avatar"</h2>
                <div class="navbar bg-base-100">
                    <div class="navbar-start">
                        <div class="dropdown">
                            <div tabindex="0" role="button" class="btn btn-ghost btn-circle">
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
                                        d="M4 6h16M4 12h16M4 18h7"
                                    />
                                </svg>
                            </div>
                            <ul
                                tabindex="0"
                                class="menu menu-sm dropdown-content mt-3 z-[1] p-2 shadow bg-base-100 rounded-box w-52"
                            >
                                <li>
                                    <a>"Homepage"</a>
                                </li>
                                <li>
                                    <a>"Portfolio"</a>
                                </li>
                                <li>
                                    <a>"About"</a>
                                </li>
                            </ul>
                        </div>
                    </div>
                    <div class="navbar-center">
                        <a class="btn btn-ghost text-xl">"My App"</a>
                    </div>
                    <div class="navbar-end">
                        <button class="btn btn-ghost btn-circle">
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
                                    d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
                                />
                            </svg>
                        </button>
                        <button class="btn btn-ghost btn-circle">
                            <div class="indicator">
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
                                        d="M15 17h5l-5 5v-5zM19 3H5a2 2 0 00-2 2v10a2 2 0 002 2h10m4-16v1a2 2 0 002 2h1m-3-3h-1a2 2 0 00-2 2v1m3-3v3"
                                    />
                                </svg>
                                <span class="badge badge-xs badge-primary indicator-item"></span>
                            </div>
                        </button>
                    </div>
                </div>

                <h2 class="text-xl font-semibold">"Navbar with Search"</h2>
                <div class="navbar bg-base-100">
                    <div class="navbar-start">
                        <a class="btn btn-ghost text-xl">"Logo"</a>
                    </div>
                    <div class="navbar-center">
                        <div class="form-control">
                            <input
                                type="text"
                                placeholder="Search"
                                class="input input-bordered w-24 md:w-auto"
                            />
                        </div>
                    </div>
                    <div class="navbar-end">
                        <div class="dropdown dropdown-end">
                            <div tabindex="0" role="button" class="btn btn-ghost btn-circle avatar">
                                <div class="w-10 rounded-full">
                                    <img
                                        alt="User avatar"
                                        src="https://picsum.photos/40/40?random=1"
                                    />
                                </div>
                            </div>
                            <ul
                                tabindex="0"
                                class="mt-3 z-[1] p-2 shadow menu menu-sm dropdown-content bg-base-100 rounded-box w-52"
                            >
                                <li>
                                    <a class="justify-between">
                                        "Profile" <span class="badge">"New"</span>
                                    </a>
                                </li>
                                <li>
                                    <a>"Settings"</a>
                                </li>
                                <li>
                                    <a>"Logout"</a>
                                </li>
                            </ul>
                        </div>
                    </div>
                </div>

                <h2 class="text-xl font-semibold">"Navbar with Breadcrumbs"</h2>
                <div class="navbar bg-base-100">
                    <div class="navbar-start">
                        <a class="btn btn-ghost text-xl">"Brand"</a>
                    </div>
                    <div class="navbar-center">
                        <div class="breadcrumbs text-sm">
                            <ul>
                                <li>
                                    <a>"Home"</a>
                                </li>
                                <li>
                                    <a>"Documents"</a>
                                </li>
                                <li>"Current Page"</li>
                            </ul>
                        </div>
                    </div>
                    <div class="navbar-end">
                        <Button size=ButtonSize::Sm>"Action"</Button>
                    </div>
                </div>

                <h2 class="text-xl font-semibold">"Colored Navbar"</h2>
                <div class="navbar bg-primary text-primary-content">
                    <a class="btn btn-ghost text-xl">"Primary Navbar"</a>
                </div>

                <div class="navbar bg-secondary text-secondary-content">
                    <a class="btn btn-ghost text-xl">"Secondary Navbar"</a>
                </div>

                <div class="navbar bg-accent text-accent-content">
                    <a class="btn btn-ghost text-xl">"Accent Navbar"</a>
                </div>

                <h2 class="text-xl font-semibold">"App-like Navbar"</h2>
                <div class="navbar bg-base-100 shadow-lg">
                    <div class="navbar-start">
                        <div class="dropdown">
                            <div tabindex="0" role="button" class="btn btn-ghost lg:hidden">
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
                                        d="M4 6h16M4 12h8m-8 6h16"
                                    />
                                </svg>
                            </div>
                            <ul
                                tabindex="0"
                                class="menu menu-sm dropdown-content mt-3 z-[1] p-2 shadow bg-base-100 rounded-box w-52"
                            >
                                <li>
                                    <a>"Dashboard"</a>
                                </li>
                                <li>
                                    <a>"Projects"</a>
                                </li>
                                <li>
                                    <a>"Tasks"</a>
                                </li>
                                <li>
                                    <a>"Reports"</a>
                                </li>
                            </ul>
                        </div>
                        <div class="flex items-center gap-2">
                            <div class="w-8 h-8 bg-primary rounded-lg flex items-center justify-center text-primary-content font-bold">
                                "A"
                            </div>
                            <span class="text-xl font-bold">"AppName"</span>
                        </div>
                    </div>

                    <div class="navbar-center hidden lg:flex">
                        <ul class="menu menu-horizontal px-1">
                            <li>
                                <a class="text-primary font-semibold">"Dashboard"</a>
                            </li>
                            <li>
                                <a>"Projects"</a>
                            </li>
                            <li>
                                <a>"Tasks"</a>
                            </li>
                            <li>
                                <a>"Reports"</a>
                            </li>
                        </ul>
                    </div>

                    <div class="navbar-end">
                        <div class="flex items-center gap-2">
                            <button class="btn btn-ghost btn-circle">
                                <div class="indicator">
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
                                            d="M15 17h5l-5 5v-5zM19 3H5a2 2 0 00-2 2v10a2 2 0 002 2h10m4-16v1a2 2 0 002 2h1m-3-3h-1a2 2 0 00-2 2v1m3-3v3"
                                        />
                                    </svg>
                                    <span class="badge badge-xs badge-primary indicator-item">
                                        "3"
                                    </span>
                                </div>
                            </button>

                            <div class="dropdown dropdown-end">
                                <div
                                    tabindex="0"
                                    role="button"
                                    class="btn btn-ghost btn-circle avatar"
                                >
                                    <div class="w-10 rounded-full ring ring-primary ring-offset-base-100 ring-offset-2">
                                        <img
                                            src="https://picsum.photos/40/40?random=2"
                                            alt="Profile"
                                        />
                                    </div>
                                </div>
                                <ul
                                    tabindex="0"
                                    class="mt-3 z-[1] p-2 shadow menu menu-sm dropdown-content bg-base-100 rounded-box w-52"
                                >
                                    <li class="menu-title">"John Doe"</li>
                                    <li>
                                        <a>"👤 Profile"</a>
                                    </li>
                                    <li>
                                        <a>"⚙️ Settings"</a>
                                    </li>
                                    <li>
                                        <a>"📊 Analytics"</a>
                                    </li>
                                    <div class="divider my-2"></div>
                                    <li>
                                        <a class="text-error">"🚪 Logout"</a>
                                    </li>
                                </ul>
                            </div>
                        </div>
                    </div>
                </div>

                <h2 class="text-xl font-semibold">"E-commerce Navbar"</h2>
                <div class="navbar bg-base-100 border-b">
                    <div class="navbar-start">
                        <div class="dropdown">
                            <div tabindex="0" role="button" class="btn btn-ghost lg:hidden">
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
                                        d="M4 6h16M4 12h8m-8 6h16"
                                    />
                                </svg>
                            </div>
                            <ul
                                tabindex="0"
                                class="menu menu-sm dropdown-content mt-3 z-[1] p-2 shadow bg-base-100 rounded-box w-52"
                            >
                                <li>
                                    <a>"Electronics"</a>
                                </li>
                                <li>
                                    <a>"Fashion"</a>
                                </li>
                                <li>
                                    <a>"Home & Garden"</a>
                                </li>
                                <li>
                                    <a>"Sports"</a>
                                </li>
                            </ul>
                        </div>
                        <a class="btn btn-ghost text-xl font-bold text-primary">"ShopLogo"</a>
                    </div>

                    <div class="navbar-center flex-1 lg:flex-none">
                        <div class="form-control w-full max-w-md">
                            <div class="join">
                                <input
                                    class="input input-bordered join-item flex-1"
                                    placeholder="Search products..."
                                />
                                <button class="btn btn-primary join-item">
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
                                            d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
                                        />
                                    </svg>
                                </button>
                            </div>
                        </div>
                    </div>

                    <div class="navbar-end">
                        <div class="flex items-center gap-2">
                            <button class="btn btn-ghost btn-circle">
                                <div class="indicator">
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
                                            d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"
                                        />
                                    </svg>
                                    <span class="badge badge-xs badge-error indicator-item">
                                        "2"
                                    </span>
                                </div>
                            </button>

                            <button class="btn btn-ghost btn-circle">
                                <div class="indicator">
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
                                            d="M3 3h2l.4 2M7 13h10l4-8H5.4m0 0L7 13m0 0l-2.293 2.293a1 1 0 001.414 1.414L10 12M7 13v6a2 2 0 002 2h6a2 2 0 002-2v-6"
                                        />
                                    </svg>
                                    <span class="badge badge-xs badge-primary indicator-item">
                                        "5"
                                    </span>
                                </div>
                            </button>

                            <div class="dropdown dropdown-end">
                                <div
                                    tabindex="0"
                                    role="button"
                                    class="btn btn-ghost btn-circle avatar"
                                >
                                    <div class="w-10 rounded-full">
                                        <img
                                            src="https://picsum.photos/40/40?random=3"
                                            alt="User"
                                        />
                                    </div>
                                </div>
                                <ul
                                    tabindex="0"
                                    class="mt-3 z-[1] p-2 shadow menu menu-sm dropdown-content bg-base-100 rounded-box w-52"
                                >
                                    <li>
                                        <a>"My Account"</a>
                                    </li>
                                    <li>
                                        <a>"Orders"</a>
                                    </li>
                                    <li>
                                        <a>"Wishlist"</a>
                                    </li>
                                    <div class="divider my-2"></div>
                                    <li>
                                        <a>"Sign Out"</a>
                                    </li>
                                </ul>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}