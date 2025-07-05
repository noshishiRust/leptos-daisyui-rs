mod demos;
use demos::*;
use leptos::mount::mount_to_body;
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;
use leptos_icons::Icon;
use leptos_router::{
    components::{Outlet, ParentRoute, Route, Router, Routes},
    hooks::use_location,
    path,
};
use leptos_use::{breakpoints_tailwind, use_breakpoints, BreakpointsTailwind};

#[derive(Clone, Debug)]
struct ComponentItem {
    name: &'static str,
    href: &'static str,
    value: &'static str,
}

#[derive(Clone, Debug)]
struct MenuCategory {
    title: &'static str,
    items: Vec<ComponentItem>,
}

fn get_menu_categories() -> Vec<MenuCategory> {
    vec![
        MenuCategory {
            title: "Action",
            items: vec![
                ComponentItem {
                    name: "Button",
                    href: "/components/button",
                    value: "button",
                },
                ComponentItem {
                    name: "Dropdown",
                    href: "/components/dropdown",
                    value: "dropdown",
                },
                ComponentItem {
                    name: "Modal",
                    href: "/components/modal",
                    value: "modal",
                },
                ComponentItem {
                    name: "Swap",
                    href: "/components/swap",
                    value: "swap",
                },
                ComponentItem {
                    name: "Theme Controller",
                    href: "/components/theme_controller",
                    value: "theme_controller",
                },
            ],
        },
        MenuCategory {
            title: "Data Display",
            items: vec![
                ComponentItem {
                    name: "Accordion",
                    href: "/components/accordion",
                    value: "accordion",
                },
                ComponentItem {
                    name: "Avatar",
                    href: "/components/avatar",
                    value: "avatar",
                },
                ComponentItem {
                    name: "Badge",
                    href: "/components/badge",
                    value: "badge",
                },
                ComponentItem {
                    name: "Card",
                    href: "/components/card",
                    value: "card",
                },
                ComponentItem {
                    name: "Carousel",
                    href: "/components/carousel",
                    value: "carousel",
                },
                ComponentItem {
                    name: "Chat",
                    href: "/components/chat",
                    value: "chat",
                },
                ComponentItem {
                    name: "Collapse",
                    href: "/components/collapse",
                    value: "collapse",
                },
                ComponentItem {
                    name: "Countdown",
                    href: "/components/countdown",
                    value: "countdown",
                },
                ComponentItem {
                    name: "Diff",
                    href: "/components/diff",
                    value: "diff",
                },
                ComponentItem {
                    name: "Kbd",
                    href: "/components/kbd",
                    value: "kbd",
                },
                ComponentItem {
                    name: "List",
                    href: "/components/list",
                    value: "list",
                },
                ComponentItem {
                    name: "Stats",
                    href: "/components/stats",
                    value: "stats",
                },
                ComponentItem {
                    name: "Status",
                    href: "/components/status",
                    value: "status",
                },
                ComponentItem {
                    name: "Table",
                    href: "/components/table",
                    value: "table",
                },
                ComponentItem {
                    name: "Timeline",
                    href: "/components/timeline",
                    value: "timeline",
                },
            ],
        },
        MenuCategory {
            title: "Navigation",
            items: vec![
                ComponentItem {
                    name: "Breadcrumbs",
                    href: "/components/breadcrumbs",
                    value: "breadcrumbs",
                },
                ComponentItem {
                    name: "Menu",
                    href: "/components/menu",
                    value: "menu",
                },
                ComponentItem {
                    name: "Navbar",
                    href: "/components/navbar",
                    value: "navbar",
                },
                ComponentItem {
                    name: "Pagination",
                    href: "/components/pagination",
                    value: "pagination",
                },
                ComponentItem {
                    name: "Steps",
                    href: "/components/steps",
                    value: "steps",
                },
                ComponentItem {
                    name: "Tab",
                    href: "/components/tab",
                    value: "tab",
                },
            ],
        },
        MenuCategory {
            title: "Feedback",
            items: vec![
                ComponentItem {
                    name: "Alert",
                    href: "/components/alert",
                    value: "alert",
                },
                ComponentItem {
                    name: "Loading",
                    href: "/components/loading",
                    value: "loading",
                },
                ComponentItem {
                    name: "Progress",
                    href: "/components/progress",
                    value: "progress",
                },
                ComponentItem {
                    name: "Radial Progress",
                    href: "/components/radial_progress",
                    value: "radial_progress",
                },
                ComponentItem {
                    name: "Skeleton",
                    href: "/components/skeleton",
                    value: "skeleton",
                },
                ComponentItem {
                    name: "Toast",
                    href: "/components/toast",
                    value: "toast",
                },
            ],
        },
        MenuCategory {
            title: "Data Input",
            items: vec![
                ComponentItem {
                    name: "Calendar",
                    href: "/components/calendar",
                    value: "calendar",
                },
                ComponentItem {
                    name: "Checkbox",
                    href: "/components/checkbox",
                    value: "checkbox",
                },
                ComponentItem {
                    name: "Fieldset",
                    href: "/components/fieldset",
                    value: "fieldset",
                },
                ComponentItem {
                    name: "File Input",
                    href: "/components/file_input",
                    value: "file_input",
                },
                ComponentItem {
                    name: "Filter",
                    href: "/components/filter",
                    value: "filter",
                },
                ComponentItem {
                    name: "Label",
                    href: "/components/label",
                    value: "label",
                },
                ComponentItem {
                    name: "Radio",
                    href: "/components/radio",
                    value: "radio",
                },
                ComponentItem {
                    name: "Range",
                    href: "/components/range",
                    value: "range",
                },
                ComponentItem {
                    name: "Rating",
                    href: "/components/rating",
                    value: "rating",
                },
                ComponentItem {
                    name: "Select",
                    href: "/components/select",
                    value: "select",
                },
                ComponentItem {
                    name: "Textarea",
                    href: "/components/textarea",
                    value: "textarea",
                },
                ComponentItem {
                    name: "Toggle",
                    href: "/components/toggle",
                    value: "toggle",
                },
                ComponentItem {
                    name: "Validator",
                    href: "/components/validator",
                    value: "validator",
                },
            ],
        },
        MenuCategory {
            title: "Layout",
            items: vec![
                ComponentItem {
                    name: "Divider",
                    href: "/components/divider",
                    value: "divider",
                },
                ComponentItem {
                    name: "Drawer",
                    href: "/components/drawer",
                    value: "drawer",
                },
                ComponentItem {
                    name: "Footer",
                    href: "/components/footer",
                    value: "footer",
                },
                ComponentItem {
                    name: "Hero",
                    href: "/components/hero",
                    value: "hero",
                },
                ComponentItem {
                    name: "Indicator",
                    href: "/components/indicator",
                    value: "indicator",
                },
                ComponentItem {
                    name: "Join",
                    href: "/components/join",
                    value: "join",
                },
                ComponentItem {
                    name: "Link",
                    href: "/components/link",
                    value: "link",
                },
                ComponentItem {
                    name: "Mask",
                    href: "/components/mask",
                    value: "mask",
                },
                ComponentItem {
                    name: "Stack",
                    href: "/components/stack",
                    value: "stack",
                },
            ],
        },
        MenuCategory {
            title: "Mockup",
            items: vec![
                ComponentItem {
                    name: "Mockup Browser",
                    href: "/components/mockup_browser",
                    value: "mockup_browser",
                },
                ComponentItem {
                    name: "Mockup Code",
                    href: "/components/mockup_code",
                    value: "mockup_code",
                },
                ComponentItem {
                    name: "Mockup Phone",
                    href: "/components/mockup_phone",
                    value: "mockup_phone",
                },
                ComponentItem {
                    name: "Mockup Window",
                    href: "/components/mockup_window",
                    value: "mockup_window",
                },
            ],
        },
    ]
}

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
                <Route path=path!("/") view=Landing />
                <ParentRoute path=path!("/components") view=Layout>
                    <Route path=path!("/") view=AccordionDemo />
                    <Route path=path!("/accordion") view=AccordionDemo />
                    <Route path=path!("/alert") view=AlertDemo />
                    <Route path=path!("/avatar") view=AvatarDemo />
                    <Route path=path!("/badge") view=BadgeDemo />
                    <Route path=path!("/breadcrumbs") view=BreadcrumbsDemo />
                    <Route path=path!("/button") view=ButtonDemo />
                    <Route path=path!("/calendar") view=CalendarDemo />
                    <Route path=path!("/card") view=CardDemo />
                    <Route path=path!("/carousel") view=CarouselDemo />
                    <Route path=path!("/chat") view=ChatDemo />
                    <Route path=path!("/checkbox") view=CheckboxDemo />
                    <Route path=path!("/collapse") view=CollapseDemo />
                    <Route path=path!("/countdown") view=CountdownDemo />
                    <Route path=path!("/diff") view=DiffDemo />
                    <Route path=path!("/divider") view=DividerDemo />
                    <Route path=path!("/drawer") view=DrawerDemo />
                    <Route path=path!("/dropdown") view=DropdownDemo />
                    <Route path=path!("/fieldset") view=FieldsetDemo />
                    <Route path=path!("/file_input") view=FileInputDemo />
                    <Route path=path!("/filter") view=FilterDemo />
                    <Route path=path!("/footer") view=FooterDemo />
                    <Route path=path!("/hero") view=HeroDemo />
                    <Route path=path!("/indicator") view=IndicatorDemo />
                    <Route path=path!("/input") view=InputDemo />
                    <Route path=path!("/join") view=JoinDemo />
                    <Route path=path!("/kbd") view=KbdDemo />
                    <Route path=path!("/label") view=LabelDemo />
                    <Route path=path!("/link") view=LinkDemo />
                    <Route path=path!("/list") view=ListDemo />
                    <Route path=path!("/loading") view=LoadingDemo />
                    <Route path=path!("/mask") view=MaskDemo />
                    <Route path=path!("/menu") view=MenuDemo />
                    <Route path=path!("/modal") view=ModalDemo />
                    <Route path=path!("/navbar") view=NavbarDemo />
                    <Route path=path!("/pagination") view=PaginationDemo />
                    <Route path=path!("/progress") view=ProgressDemo />
                    <Route path=path!("/radial_progress") view=RadialProgressDemo />
                    <Route path=path!("/radio") view=RadioDemo />
                    <Route path=path!("/range") view=RangeDemo />
                    <Route path=path!("/rating") view=RatingDemo />
                    <Route path=path!("/select") view=SelectDemo />
                    <Route path=path!("/skeleton") view=SkeletonDemo />
                    <Route path=path!("/stack") view=StackDemo />
                    <Route path=path!("/stats") view=StatsDemo />
                    <Route path=path!("/status") view=StatusDemo />
                    <Route path=path!("/steps") view=StepsDemo />
                    <Route path=path!("/swap") view=SwapDemo />
                    <Route path=path!("/tab") view=TabDemo />
                    <Route path=path!("/table") view=TableDemo />
                    <Route path=path!("/textarea") view=TextareaDemo />
                    <Route path=path!("/theme_controller") view=ThemeControllerDemo />
                    <Route path=path!("/timeline") view=TimelineDemo />
                    <Route path=path!("/toast") view=ToastDemo />
                    <Route path=path!("/toggle") view=ToggleDemo />
                    <Route path=path!("/validator") view=ValidatorDemo />
                </ParentRoute>
            </Routes>
        </Router>
    }
}

#[component]
fn Layout() -> impl IntoView {
    let location = use_location();
    let breakpoints = use_breakpoints(breakpoints_tailwind());

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
        <div class="min-h-screen min-w-screen bg-base-100">
            <Navbar class="min-w-screen bg-base-200 border-b border-base-300">
                <NavbarStart class="gap-4">
                    <div class="lg:hidden">
                        <label for="drawer-toggle" class="hover:cursor-pointer">
                            <Icon icon=icondata::AiMenuOutlined />
                        </label>
                    </div>
                    <Icon icon=icondata::CgComponents />
                    <h1 class="text-xl font-bold">"daisyUI + Leptos Showcase"</h1>
                </NavbarStart>
                <NavbarEnd class="gap-2">
                    <LinkButton
                        href="https://github.com/noshishiRust/leptos-daisyui-rs"
                        style=ButtonStyle::Ghost
                        shape=ButtonShape::Circle
                    >
                        <Icon icon=icondata::AiGithubFilled />
                    </LinkButton>
                </NavbarEnd>
            </Navbar>

            <Drawer open=breakpoints.ge(BreakpointsTailwind::Lg)>
                <DrawerToggle id="drawer-toggle" />

                <div class="drawer-content">

                    // Content area with padding
                    <div class="p-6 w-full">
                        <Outlet />
                    </div>
                </div>

                <DrawerSide>
                    <label for="drawer-toggle" class="drawer-overlay"></label>
                    <div class="min-h-full w-64 bg-base-200 text-base-content">
                        <div class="p-4">
                            <h2 class="text-lg font-semibold mb-4">"Components"</h2>
                            <Menu
                                selected=selected
                                direction=MenuDirection::Vertical
                                class="w-full"
                            >
                                {get_menu_categories()
                                    .into_iter()
                                    .map(|category| {
                                        view! {
                                            <MenuItem is_submenu=true>
                                                <MenuTitle>{category.title}</MenuTitle>
                                                <SubMenu>
                                                    {category
                                                        .items
                                                        .into_iter()
                                                        .map(|item| {
                                                            view! {
                                                                <MenuItem href=item.href value=item.value>
                                                                    {item.name}
                                                                </MenuItem>
                                                            }
                                                        })
                                                        .collect_view()}
                                                </SubMenu>
                                            </MenuItem>
                                        }
                                    })
                                    .collect_view()}
                            </Menu>
                        </div>
                    </div>
                </DrawerSide>
            </Drawer>
        </div>
    }
}

#[component]
fn Landing() -> impl IntoView {
    view! {
        <Hero class="min-h-screen">
            <HeroContent>
                <div class="max-w-md">
                    <h1 class="text-5xl font-bold mb-6">"leptos-daisyui-rs"</h1>
                    <div class="flex flex-wrap gap-4 justify-center mb-8">
                        <Badge color=BadgeColor::Accent>"Leptos"</Badge>
                        <Badge color=BadgeColor::Success>"Tailwind CSS"</Badge>
                        <Badge color=BadgeColor::Info>"daisyUI"</Badge>
                    </div>
                    <div class="flex flex-row gap-4 justify-center">
                        <LinkButton href="/components/button" color=ButtonColor::Primary>
                            "Browse Components"
                        </LinkButton>
                        <LinkButton
                            href="https://github.com/noshishiRust/leptos-daisyui-rs"
                            style=ButtonStyle::Ghost
                        >
                            <Icon icon=icondata::AiGithubFilled />
                            "View on GitHub"
                        </LinkButton>
                    </div>
                </div>
            </HeroContent>
        </Hero>
    }
}
