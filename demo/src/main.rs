mod demos;
use demos::*;
use leptos::mount::mount_to_body;
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;
use leptos_router::{
    components::{Outlet, ParentRoute, Route, Router, Routes},
    hooks::use_location,
    path,
};

fn main() {
    console_error_panic_hook::set_once();
    let _ = console_log::init_with_level(log::Level::Debug);

    mount_to_body(|| {
        view! { <App /> }
    });
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| "Page not found">
                <ParentRoute path=path!("/components") view=Layout>
                    <Route path=path!("/") view=AccordionDemo />
                    <Route path=path!("/accordion") view=AccordionDemo />
                    <Route path=path!("/alert") view=AlertDemo />
                    <Route path=path!("/avatar") view=AvatarDemo />
                    <Route path=path!("/badge") view=BadgeDemo />
                    <Route path=path!("/breadcrumbs") view=BreadcrumbsDemo />
                    <Route path=path!("/button") view=ButtonDemo />
                    <Route path=path!("/card") view=CardDemo />
                    <Route path=path!("/carousel") view=CarouselDemo />
                    <Route path=path!("/chat") view=ChatDemo />
                    <Route path=path!("/checkbox") view=CheckboxDemo />
                    <Route path=path!("/collapse") view=CollapseDemo />
                    <Route path=path!("/countdown") view=CountdownDemo />
                    <Route path=path!("/divider") view=DividerDemo />
                    <Route path=path!("/dropdown") view=DropdownDemo />
                    <Route path=path!("/file_input") view=FileInputDemo />
                    <Route path=path!("/hero") view=HeroDemo />
                    <Route path=path!("/input") view=InputDemo />
                    <Route path=path!("/join") view=JoinDemo />
                    <Route path=path!("/kbd") view=KbdDemo />
                    <Route path=path!("/loading") view=LoadingDemo />
                    <Route path=path!("/modal") view=ModalDemo />
                    <Route path=path!("/navbar") view=NavbarDemo />
                    <Route path=path!("/pagination") view=PaginationDemo />
                    <Route path=path!("/progress") view=ProgressDemo />
                    <Route path=path!("/radio") view=RadioDemo />
                    <Route path=path!("/range") view=RangeDemo />
                    <Route path=path!("/rating") view=RatingDemo />
                    <Route path=path!("/select") view=SelectDemo />
                    <Route path=path!("/skeleton") view=SkeletonDemo />
                    <Route path=path!("/stats") view=StatsDemo />
                    <Route path=path!("/steps") view=StepsDemo />
                    <Route path=path!("/swap") view=SwapDemo />
                    <Route path=path!("/tab") view=TabDemo />
                    <Route path=path!("/table") view=TableDemo />
                    <Route path=path!("/textarea") view=TextareaDemo />
                    <Route path=path!("/theme_controller") view=ThemeControllerDemo />
                    <Route path=path!("/timeline") view=TimelineDemo />
                    <Route path=path!("/toast") view=ToastDemo />
                    <Route path=path!("/toggle") view=ToggleDemo />
                </ParentRoute>
            </Routes>
        </Router>
    }
}

#[component]
fn Layout() -> impl IntoView {
    let location = use_location();

    let selected = RwSignal::new(None);

    Effect::new(move || {
        let init_component_name = location
            .pathname
            .get_untracked()
            .strip_prefix("/components/")
            .map(|v| v.to_string());

        selected.set(init_component_name);
    });

    view! {
        <div class="min-h-screen bg-base-100">
            <Navbar class="bg-base-200 border-b border-base-300">
                <NavbarStart>
                    <h1 class="text-xl font-bold">"daisyUI + Leptos Showcase"</h1>
                </NavbarStart>
                <NavbarEnd>
                    <ThemeController theme_name="dark" checked=Signal::derive(move || false) />
                    <span class="ml-2 text-sm">"Toggle Dark Mode"</span>
                </NavbarEnd>
            </Navbar>
            <div class="drawer lg:drawer-open">
                <input id="drawer-toggle" type="checkbox" class="drawer-toggle" />

                <div class="drawer-content">
                    <div class="navbar lg:hidden bg-base-200">
                        <div class="flex-none">
                            <label
                                for="drawer-toggle"
                                class="btn btn-square btn-ghost drawer-button"
                            >
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    fill="none"
                                    viewBox="0 0 24 24"
                                    class="inline-block w-6 h-6 stroke-current"
                                >
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        stroke-width="2"
                                        d="M4 6h16M4 12h16M4 18h16"
                                    ></path>
                                </svg>
                            </label>
                        </div>
                        <div class="flex-1">
                            <span class="text-lg font-semibold">"Components"</span>
                        </div>
                    </div>

                    // Content area with padding
                    <div class="p-6">
                        <Outlet />
                    </div>
                </div>

                // Sidebar
                <div class="drawer-side">
                    <label for="drawer-toggle" class="drawer-overlay"></label>
                    <div class="min-h-full w-64 bg-base-200 text-base-content">
                        <div class="p-4">
                            <h2 class="text-lg font-semibold mb-4">"Components"</h2>
                            <Menu
                                selected=selected
                                direction=MenuDirection::Vertical
                                class="w-full"
                            >
                                <MenuItem is_submenu=true>
                                    <MenuTitle>"Action"</MenuTitle>
                                    <SubMenu>
                                        <MenuItem href="/components/button" value="button">
                                            "Button"
                                        </MenuItem>
                                        <MenuItem href="/components/dropdown" value="dropdown">
                                            "Dropdown"
                                        </MenuItem>
                                        <MenuItem href="/components/modal" value="modal">
                                            "Modal"
                                        </MenuItem>
                                        <MenuItem href="/components/swap" value="swap">
                                            "Swap"
                                        </MenuItem>
                                        <MenuItem
                                            href="/components/theme_controller"
                                            value="theme_controller"
                                        >
                                            "Theme Controller"
                                        </MenuItem>
                                    </SubMenu>
                                </MenuItem>
                                <MenuItem is_submenu=true>
                                    <MenuTitle>"Data Display"</MenuTitle>
                                    <SubMenu>
                                        <MenuItem href="/components/accordion" value="accordion">
                                            "Accordion"
                                        </MenuItem>
                                        <MenuItem href="/components/avatar" value="avatar">
                                            "Avatar"
                                        </MenuItem>
                                        <MenuItem href="/components/badge" value="badge">
                                            "Badge"
                                        </MenuItem>
                                        <MenuItem href="/components/card" value="card">
                                            "Card"
                                        </MenuItem>
                                        <MenuItem href="/components/carousel" value="carousel">
                                            "Carousel"
                                        </MenuItem>
                                        <MenuItem href="/components/chat" value="chat">
                                            "Chat"
                                        </MenuItem>
                                        <MenuItem href="/components/collapse" value="collapse">
                                            "Collapse"
                                        </MenuItem>
                                        <MenuItem href="/components/countdown" value="countdown">
                                            "Countdown"
                                        </MenuItem>
                                        <MenuItem href="/components/diff" value="diff">
                                            "Diff"
                                        </MenuItem>
                                        <MenuItem href="/components/kbd" value="kbd">
                                            "Kbd"
                                        </MenuItem>
                                        <MenuItem href="/components/list" value="list">
                                            "List"
                                        </MenuItem>
                                        <MenuItem href="/components/stats" value="stats">
                                            "Stats"
                                        </MenuItem>
                                        <MenuItem href="/components/status" value="status">
                                            "Status"
                                        </MenuItem>
                                        <MenuItem href="/components/table" value="table">
                                            "Table"
                                        </MenuItem>
                                        <MenuItem href="/components/timeline" value="timeline">
                                            "Timeline"
                                        </MenuItem>
                                    </SubMenu>
                                </MenuItem>
                                <MenuItem is_submenu=true>
                                    <MenuTitle>"Navigation"</MenuTitle>
                                    <SubMenu>
                                        <MenuItem
                                            href="/components/breadcrumbs"
                                            value="breadcrumbs"
                                        >
                                            "Breadcrumbs"
                                        </MenuItem>
                                        <MenuItem href="/components/dock" value="dock">
                                            "Dock"
                                        </MenuItem>
                                        <MenuItem href="/components/link" value="link">
                                            "Link"
                                        </MenuItem>
                                        <MenuItem href="/components/menu" value="menu">
                                            "Menu"
                                        </MenuItem>
                                        <MenuItem href="/components/navbar" value="navbar">
                                            "Navbar"
                                        </MenuItem>
                                        <MenuItem href="/components/pagination" value="pagination">
                                            "Pagination"
                                        </MenuItem>
                                        <MenuItem href="/components/steps" value="steps">
                                            "Steps"
                                        </MenuItem>
                                        <MenuItem href="/components/tab" value="tab">
                                            "Tab"
                                        </MenuItem>
                                    </SubMenu>
                                </MenuItem>

                                <MenuItem is_submenu=true>
                                    <MenuTitle>"Feedback"</MenuTitle>
                                    <SubMenu>
                                        <MenuItem href="/components/alert" value="alert">
                                            "Alert"
                                        </MenuItem>
                                        <MenuItem href="/components/loading" value="loading">
                                            "Loading"
                                        </MenuItem>
                                        <MenuItem href="/components/progress" value="progress">
                                            "Progress"
                                        </MenuItem>
                                        <MenuItem
                                            href="/components/radial_progress"
                                            value="radial_progress"
                                        >
                                            "Radial Progress"
                                        </MenuItem>
                                        <MenuItem href="/components/skeleton" value="skeleton">
                                            "Skeleton"
                                        </MenuItem>
                                        <MenuItem href="/components/toast" value="toast">
                                            "Toast"
                                        </MenuItem>
                                    </SubMenu>
                                </MenuItem>

                                <MenuItem is_submenu=true>
                                    <MenuTitle>"Data Input"</MenuTitle>
                                    <SubMenu>
                                        <MenuItem href="/components/calendar" value="calendar">
                                            "Calendar"
                                        </MenuItem>
                                        <MenuItem href="/components/checkbox" value="checkbox">
                                            "Checkbox"
                                        </MenuItem>
                                        <MenuItem href="/components/fieldset" value="fieldset">
                                            "Fieldset"
                                        </MenuItem>
                                        <MenuItem href="/components/file_input" value="file_input">
                                            "File Input"
                                        </MenuItem>
                                        <MenuItem href="/components/filter" value="filter">
                                            "Filter"
                                        </MenuItem>
                                        <MenuItem href="/components/label" value="label">
                                            "Label"
                                        </MenuItem>
                                        <MenuItem href="/components/radio" value="radio">
                                            "Radio"
                                        </MenuItem>
                                        <MenuItem href="/components/range" value="range">
                                            "Range"
                                        </MenuItem>
                                        <MenuItem href="/components/rating" value="rating">
                                            "Rating"
                                        </MenuItem>
                                        <MenuItem href="/components/select" value="select">
                                            "Select"
                                        </MenuItem>
                                        <MenuItem href="/components/textarea" value="textarea">
                                            "Textarea"
                                        </MenuItem>
                                        <MenuItem href="/components/toggle" value="toggle">
                                            "Toggle"
                                        </MenuItem>
                                        <MenuItem href="/components/validator" value="validator">
                                            "Validator"
                                        </MenuItem>
                                    </SubMenu>
                                </MenuItem>

                                <MenuItem is_submenu=true>
                                    <MenuTitle>"Layout"</MenuTitle>
                                    <SubMenu>
                                        <MenuItem href="/components/divider" value="divider">
                                            "Divider"
                                        </MenuItem>
                                        <MenuItem href="/components/drawer" value="drawer">
                                            "Drawer"
                                        </MenuItem>
                                        <MenuItem href="/components/footer" value="footer">
                                            "Footer"
                                        </MenuItem>
                                        <MenuItem href="/components/hero" value="hero">
                                            "Hero"
                                        </MenuItem>
                                        <MenuItem href="/components/indicator" value="indicator">
                                            "Indicator"
                                        </MenuItem>
                                        <MenuItem href="/components/input" value="input">
                                            "Input"
                                        </MenuItem>
                                        <MenuItem href="/components/join" value="join">
                                            "Join"
                                        </MenuItem>
                                        <MenuItem href="/components/mask" value="mask">
                                            "Mask"
                                        </MenuItem>
                                        <MenuItem href="/components/stack" value="stack">
                                            "Stack"
                                        </MenuItem>
                                    </SubMenu>

                                </MenuItem>
                                <MenuItem is_submenu=true>
                                    <MenuTitle>"Mockup"</MenuTitle>
                                    <SubMenu>
                                        <MenuItem href="/components/divider" value="divider">
                                            "Divider"
                                        </MenuItem>
                                        <MenuItem href="/components/drawer" value="drawer">
                                            "Drawer"
                                        </MenuItem>
                                        <MenuItem href="/components/footer" value="footer">
                                            "Footer"
                                        </MenuItem>
                                        <MenuItem href="/components/hero" value="hero">
                                            "Hero"
                                        </MenuItem>
                                        <MenuItem href="/components/indicator" value="indicator">
                                            "Indicator"
                                        </MenuItem>
                                        <MenuItem href="/components/input" value="input">
                                            "Input"
                                        </MenuItem>
                                        <MenuItem href="/components/join" value="join">
                                            "Join"
                                        </MenuItem>
                                        <MenuItem href="/components/mask" value="mask">
                                            "Mask"
                                        </MenuItem>
                                        <MenuItem href="/components/stack" value="stack">
                                            "Stack"
                                        </MenuItem>
                                    </SubMenu>
                                </MenuItem>
                            </Menu>
                        </div>
                    </div>
                </div>

            </div>
        </div>
    }
}
