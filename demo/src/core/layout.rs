use leptos::prelude::*;
use leptos_daisyui_rs::components::*;
use leptos_icons::Icon;
use leptos_router::{components::Outlet, hooks::use_location};
use leptos_use::{breakpoints_tailwind, use_breakpoints, BreakpointsTailwind};

/// Layout component for the demos
#[component]
pub fn Layout() -> impl IntoView {
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
        <div class="h-screen w-screen bg-base-100">
            <Navbar class="w-screen bg-base-200 border-b border-base-300">
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
                <DrawerToggle id="drawer-toggle" checked=breakpoints.ge(BreakpointsTailwind::Lg) />

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
                    name: "FAB",
                    href: "/components/fab",
                    value: "fab",
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
                    name: "Hover 3D",
                    href: "/components/hover_3d",
                    value: "hover_3d",
                },
                ComponentItem {
                    name: "Hover Gallery",
                    href: "/components/hover_gallery",
                    value: "hover_gallery",
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
                    name: "Text Rotate",
                    href: "/components/text_rotate",
                    value: "text_rotate",
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
                ComponentItem {
                    name: "Tooltip",
                    href: "/components/tooltip",
                    value: "tooltip",
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
                    name: "Input",
                    href: "/components/input",
                    value: "input",
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
                    name: "Dock",
                    href: "/components/dock",
                    value: "dock",
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
