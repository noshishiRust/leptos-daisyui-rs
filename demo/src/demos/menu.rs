use crate::core::{ContentLayout, Section};
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;
use leptos_icons::Icon;

#[component]
pub fn MenuDemo() -> impl IntoView {
    let selected = RwSignal::new(None::<String>);
    let _manual_selected = RwSignal::new(None::<String>);

    view! {
        <ContentLayout
            title="Menu"
            description="Menu is used to display a list of links vertically or horizontally"
        >
            <Section title="Basic Menu">
                <Menu class="bg-base-200 rounded-box w-56">
                    <MenuItem value="item1">"Item 1"</MenuItem>
                    <MenuItem value="item2">"Item 2"</MenuItem>
                    <MenuItem value="item3">"Item 3"</MenuItem>
                </Menu>
            </Section>

            <Section title="Menu with Icons">
                <Menu class="bg-base-200 rounded-box w-56">
                    <MenuItem value="home">
                        <Icon icon=icondata::AiHomeOutlined />
                        "Home"
                    </MenuItem>
                    <MenuItem value="profile">
                        <Icon icon=icondata::AiUserOutlined />
                        "Profile"
                    </MenuItem>
                    <MenuItem value="settings">
                        <Icon icon=icondata::AiSettingOutlined />
                        "Settings"
                    </MenuItem>
                </Menu>
            </Section>

            <Section title="Menu with Titles">
                <Menu class="bg-base-200 rounded-box w-56">
                    <MenuTitle>"Navigation"</MenuTitle>
                    <MenuItem value="dashboard">"Dashboard"</MenuItem>
                    <MenuItem value="analytics">"Analytics"</MenuItem>

                    <MenuTitle>"Account"</MenuTitle>
                    <MenuItem value="profile2">"Profile"</MenuItem>
                    <MenuItem value="billing">"Billing"</MenuItem>
                </Menu>
            </Section>

            <Section title="Horizontal Menu">
                <Menu direction=MenuDirection::Horizontal class="bg-base-200 rounded-box">
                    <MenuItem value="file">"File"</MenuItem>
                    <MenuItem value="edit">"Edit"</MenuItem>
                    <MenuItem value="view">"View"</MenuItem>
                    <MenuItem value="help">"Help"</MenuItem>
                </Menu>
            </Section>

            <Section title="Menu with SubMenus">
                <Menu class="bg-base-200 rounded-box w-56">
                    <MenuItem value="files">
                        <Icon icon=icondata::AiFolderOutlined />
                        "Files"
                        <SubMenu class="p-2">
                            <MenuItem value="documents" is_submenu=true>
                                "Documents"
                            </MenuItem>
                            <MenuItem value="images" is_submenu=true>
                                "Images"
                            </MenuItem>
                            <MenuItem value="videos" is_submenu=true>
                                "Videos"
                            </MenuItem>
                        </SubMenu>
                    </MenuItem>
                    <MenuItem value="tools">
                        <Icon icon=icondata::AiToolOutlined />
                        "Tools"
                        <SubMenu class="p-2">
                            <MenuItem value="calculator" is_submenu=true>
                                "Calculator"
                            </MenuItem>
                            <MenuItem value="editor" is_submenu=true>
                                "Editor"
                            </MenuItem>
                        </SubMenu>
                    </MenuItem>
                </Menu>
            </Section>

            <Section title="Menu Sizes">
                <div class="space-y-4">
                    <div>
                        <h3 class="text-sm font-medium mb-2">"Extra Small"</h3>
                        <Menu size=MenuSize::Xs class="bg-base-200 rounded-box w-56">
                            <MenuItem value="xs1">"Item 1"</MenuItem>
                            <MenuItem value="xs2">"Item 2"</MenuItem>
                        </Menu>
                    </div>
                    <div>
                        <h3 class="text-sm font-medium mb-2">"Small"</h3>
                        <Menu size=MenuSize::Sm class="bg-base-200 rounded-box w-56">
                            <MenuItem value="sm1">"Item 1"</MenuItem>
                            <MenuItem value="sm2">"Item 2"</MenuItem>
                        </Menu>
                    </div>
                    <div>
                        <h3 class="text-sm font-medium mb-2">"Large"</h3>
                        <Menu size=MenuSize::Lg class="bg-base-200 rounded-box w-56">
                            <MenuItem value="lg1">"Item 1"</MenuItem>
                            <MenuItem value="lg2">"Item 2"</MenuItem>
                        </Menu>
                    </div>
                </div>
            </Section>

            <Section title="Interactive Menu with Selection">
                <div class="space-y-4">
                    <p class="text-sm text-base-content/70">
                        "Selected: " {move || selected.get().unwrap_or_else(|| "None".to_string())}
                    </p>
                    <Menu selected=selected class="bg-base-200 rounded-box w-56">
                        <MenuItem value="option1">"Option 1"</MenuItem>
                        <MenuItem value="option2">"Option 2"</MenuItem>
                        <MenuItem value="option3">"Option 3"</MenuItem>
                        <MenuItem disabled=RwSignal::new(true) value="disabled">
                            "Disabled Option"
                        </MenuItem>
                    </Menu>
                </div>
            </Section>

            <Section title="Menu as Sidebar">
                <div class="flex">
                    <Menu class="bg-base-200 rounded-box w-56 min-h-96">
                        <MenuTitle>"Main Menu"</MenuTitle>
                        <MenuItem value="dash">
                            <Icon icon=icondata::AiDashboardOutlined />
                            "Dashboard"
                        </MenuItem>
                        <MenuItem value="projects">
                            <Icon icon=icondata::AiFolderOutlined />
                            "Projects"
                        </MenuItem>
                        <MenuItem value="team">
                            <Icon icon=icondata::AiTeamOutlined />
                            "Team"
                        </MenuItem>

                        <MenuTitle>"Settings"</MenuTitle>
                        <MenuItem value="account">
                            <Icon icon=icondata::AiUserOutlined />
                            "Account"
                        </MenuItem>
                        <MenuItem value="preferences">
                            <Icon icon=icondata::AiSettingOutlined />
                            "Preferences"
                        </MenuItem>
                    </Menu>

                    <div class="flex-1 p-6 bg-base-100 rounded-box ml-4">
                        <h3 class="text-lg font-semibold">"Main Content Area"</h3>
                        <p class="text-base-content/70">
                            "This would be your main application content."
                        </p>
                    </div>
                </div>
            </Section>
        </ContentLayout>
    }
}
