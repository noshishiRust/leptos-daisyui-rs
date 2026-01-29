use crate::core::{ContentLayout, Section};
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;
use leptos_icons::Icon;

#[component]
pub fn NavbarDemo() -> impl IntoView {
    view! {
        <ContentLayout
            title="Navbar"
            description="Navigation bar component for displaying navigation links and information at the top of a page"
        >

            <Section title="Basic Navbar">
                <Navbar>
                    <Button style=ButtonStyle::Ghost size=ButtonSize::Lg>
                        "daisyUI"
                    </Button>
                </Navbar>
            </Section>

            <Section title="Navbar with Menu">
                <Navbar>
                    <NavbarStart>
                        <Dropdown>
                            <Button style=ButtonStyle::Ghost class="lg:hidden">
                                <Icon icon=icondata::AiMenuOutlined />
                            </Button>
                            <DropdownContent
                                is_menu=true
                                class="w-52 mt-3 z-[1] p-2 shadow bg-base-100 rounded-box"
                            >
                                <MenuItem>"Item 1"</MenuItem>
                                <MenuItem>"Item 2"</MenuItem>
                                <MenuItem>"Item 3"</MenuItem>
                            </DropdownContent>
                        </Dropdown>
                        <Button style=ButtonStyle::Ghost size=ButtonSize::Lg>
                            "daisyUI"
                        </Button>
                    </NavbarStart>
                    <NavbarCenter class="hidden lg:flex">
                        <Menu direction=MenuDirection::Horizontal class="px-1">
                            <MenuItem>"Item 1"</MenuItem>
                            <MenuItem>"Item 2"</MenuItem>
                            <MenuItem>"Item 3"</MenuItem>
                        </Menu>
                    </NavbarCenter>
                    <NavbarEnd>
                        <Button>"Button"</Button>
                    </NavbarEnd>
                </Navbar>
            </Section>

            <Section title="Navbar with Avatar and Indicators">
                <Navbar>
                    <NavbarStart>
                        <Button style=ButtonStyle::Ghost shape=ButtonShape::Circle>
                            <Icon icon=icondata::AiMenuOutlined />
                        </Button>
                    </NavbarStart>
                    <NavbarCenter>
                        <Button style=ButtonStyle::Ghost size=ButtonSize::Lg>
                            "My App"
                        </Button>
                    </NavbarCenter>
                    <NavbarEnd>
                        <Button style=ButtonStyle::Ghost shape=ButtonShape::Circle>
                            <Icon icon=icondata::AiSearchOutlined />
                        </Button>
                        <Button style=ButtonStyle::Ghost shape=ButtonShape::Circle>
                            <Indicator>
                                <Icon icon=icondata::AiMailOutlined />
                                <IndicatorItem
                                    vertical=IndicatorVerticalPlacement::Top
                                    horizontal=IndicatorHorizontalPlacement::End
                                >
                                    <Badge size=BadgeSize::Xs color=BadgeColor::Primary>"5"</Badge>
                                </IndicatorItem>
                            </Indicator>
                        </Button>
                    </NavbarEnd>
                </Navbar>
            </Section>

            <Section title="Navbar with Search">
                <Navbar>
                    <NavbarStart>
                        <Button style=ButtonStyle::Ghost size=ButtonSize::Lg>
                            "Logo"
                        </Button>
                    </NavbarStart>
                    <NavbarCenter>
                        <Input attr:placeholder="Search" class="w-24 md:w-auto" />
                    </NavbarCenter>
                    <NavbarEnd>
                        <Dropdown alignment=DropdownAlignment::End>
                            <Button style=ButtonStyle::Ghost shape=ButtonShape::Circle>
                                <Avatar>
                                    <div class="w-10 rounded-full">
                                        <img
                                            alt="User avatar"
                                            src="https://picsum.photos/40/40?random=1"
                                        />
                                    </div>
                                </Avatar>
                            </Button>
                            <DropdownContent
                                is_menu=true
                                class="w-52 mt-3 z-[1] p-2 shadow bg-base-100 rounded-box"
                            >
                                <MenuItem>
                                    <span>"Profile" {" "}</span>
                                    <Badge size=BadgeSize::Xs>"New"</Badge>
                                </MenuItem>
                                <MenuItem>"Settings"</MenuItem>
                                <MenuItem>"Logout"</MenuItem>
                            </DropdownContent>
                        </Dropdown>
                    </NavbarEnd>
                </Navbar>
            </Section>

            <Section title="Navbar with Breadcrumbs">
                <Navbar>
                    <NavbarStart>
                        <Button style=ButtonStyle::Ghost size=ButtonSize::Lg>
                            "Brand"
                        </Button>
                    </NavbarStart>
                    <NavbarCenter>
                        <Breadcrumbs inner_class="text-sm">
                            <BreadcrumbItem>"Home"</BreadcrumbItem>
                            <BreadcrumbItem>"Documents"</BreadcrumbItem>
                            <BreadcrumbItem>"Current Page"</BreadcrumbItem>
                        </Breadcrumbs>
                    </NavbarCenter>
                    <NavbarEnd>
                        <Button size=ButtonSize::Sm>"Action"</Button>
                    </NavbarEnd>
                </Navbar>
            </Section>

            <Section title="Colored Navbars">
                <div class="space-y-4">
                    <Navbar class="bg-primary text-primary-content">
                        <Button style=ButtonStyle::Ghost size=ButtonSize::Lg>
                            "Primary Navbar"
                        </Button>
                    </Navbar>

                    <Navbar class="bg-secondary text-secondary-content">
                        <Button style=ButtonStyle::Ghost size=ButtonSize::Lg>
                            "Secondary Navbar"
                        </Button>
                    </Navbar>

                    <Navbar class="bg-accent text-accent-content">
                        <Button style=ButtonStyle::Ghost size=ButtonSize::Lg>
                            "Accent Navbar"
                        </Button>
                    </Navbar>
                </div>
            </Section>

        </ContentLayout>
    }
}
