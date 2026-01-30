use crate::core::{ContentLayout, Section};
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;
use leptos_icons::Icon;

#[component]
pub fn DrawerDemo() -> impl IntoView {
    let (drawer_open, set_drawer_open) = signal(false);

    view! {
        <ContentLayout
            title="Drawer"
            description="Drawer is a sidebar overlay that can be toggled on and off"
        >
            <Section title="Basic Drawer" col=true>
                <div class="h-96 bg-base-200 rounded-lg overflow-hidden">
                    <Drawer open=drawer_open>
                        <DrawerToggle
                            id="drawer-demo"
                            checked=drawer_open
                            on:click=move |_| set_drawer_open.update(|open| *open = !*open)
                        />

                        <div class="drawer-content flex flex-col items-center justify-center">
                            <div class="text-center">
                                <h3 class="text-lg font-semibold mb-4">"Page Content"</h3>
                                <Button
                                    color=ButtonColor::Primary
                                    on:click=move |_| set_drawer_open.set(true)
                                >
                                    "Open Drawer"
                                </Button>
                            </div>
                        </div>

                        <DrawerSide>
                            <label
                                for="drawer-demo"
                                class="drawer-overlay"
                                on:click=move |_| set_drawer_open.set(false)
                            ></label>
                            <div class="menu bg-base-200 text-base-content min-h-full w-80 p-4">
                                <div class="flex justify-between items-center mb-6">
                                    <h3 class="text-lg font-semibold">"Sidebar Menu"</h3>
                                    <Button
                                        size=ButtonSize::Sm
                                        style=ButtonStyle::Ghost
                                        shape=ButtonShape::Circle
                                        on:click=move |_| set_drawer_open.set(false)
                                    >
                                        <Icon icon=icondata::AiCloseOutlined />
                                    </Button>
                                </div>

                                <Menu direction=MenuDirection::Vertical class="w-full">
                                    <MenuItem>
                                        <Icon icon=icondata::AiHomeOutlined />
                                        "Home"
                                    </MenuItem>
                                    <MenuItem>
                                        <Icon icon=icondata::AiUserOutlined />
                                        "Profile"
                                    </MenuItem>
                                    <MenuItem>
                                        <Icon icon=icondata::AiSettingOutlined />
                                        "Settings"
                                    </MenuItem>
                                    <MenuItem is_submenu=true>
                                        <MenuTitle>
                                            <Icon icon=icondata::AiFolderOutlined />
                                            "Projects"
                                        </MenuTitle>
                                        <SubMenu>
                                            <MenuItem>"Project A"</MenuItem>
                                            <MenuItem>"Project B"</MenuItem>
                                            <MenuItem>"Project C"</MenuItem>
                                        </SubMenu>
                                    </MenuItem>
                                    <MenuItem>
                                        <Icon icon=icondata::AiQuestionCircleOutlined />
                                        "Help"
                                    </MenuItem>
                                </Menu>
                            </div>
                        </DrawerSide>
                    </Drawer>
                </div>
            </Section>

            <Section title="Drawer with Different Content" col=true>
                <Alert color=AlertColor::Info>
                    <Icon icon=icondata::AiInfoCircleOutlined />
                    "The drawer component is also used in the main layout of this demo application. You can see it in action by toggling the sidebar menu."
                </Alert>

                <div class="mockup-code">
                    <pre data-prefix="1">
                        <code>"<Drawer open={{drawer_open}}>"</code>
                    </pre>
                    <pre data-prefix="2">
                        <code>"  <DrawerToggle id=\"my-drawer\" />"</code>
                    </pre>
                    <pre data-prefix="3">
                        <code>"  <div class=\"drawer-content\">"</code>
                    </pre>
                    <pre data-prefix="4">
                        <code>"    <!-- Page content -->"</code>
                    </pre>
                    <pre data-prefix="5">
                        <code>"  </div>"</code>
                    </pre>
                    <pre data-prefix="6">
                        <code>"  <DrawerSide>"</code>
                    </pre>
                    <pre data-prefix="7">
                        <code>"    <!-- Sidebar content -->"</code>
                    </pre>
                    <pre data-prefix="8">
                        <code>"  </DrawerSide>"</code>
                    </pre>
                    <pre data-prefix="9">
                        <code>"</Drawer>"</code>
                    </pre>
                </div>
            </Section>
        </ContentLayout>
    }
}