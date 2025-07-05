use leptos::prelude::*;
use leptos_daisyui_rs::components::*;
use leptos_icons::Icon;

#[component]
pub fn MenuDemo() -> impl IntoView {
    let (selected, set_selected) = signal(Some("item1".to_string()));

    view! {
        // <div class="space-y-6">
        //     <h1 class="text-3xl font-bold">"Menu"</h1>
        //     <p class="text-base-content/70">
        //         "Menu is used to display a list of links vertically or horizontally"
        //     </p>

        //     <div class="space-y-4">
        //         <h2 class="text-xl font-semibold">"Vertical Menu"</h2>
        //         <div class="bg-base-200 rounded-box w-56 p-2">
        //             <Menu
        //                 selected=selected
        //                 direction=MenuDirection::Vertical
        //                 class="w-full"
        //             >
        //                 <MenuItem value="item1">
        //                     <Icon icon=icondata::AiHomeOutlined />
        //                     "Home"
        //                 </MenuItem>
        //                 <MenuItem value="item2">
        //                     <Icon icon=icondata::AiUserOutlined />
        //                     "Profile"
        //                 </MenuItem>
        //                 <MenuItem value="item3">
        //                     <Icon icon=icondata::AiSettingOutlined />
        //                     "Settings"
        //                 </MenuItem>
        //                 <MenuItem value="item4" disabled=true>
        //                     <Icon icon=icondata::AiLockOutlined />
        //                     "Disabled"
        //                 </MenuItem>
        //             </Menu>
        //         </div>

        //         <h2 class="text-xl font-semibold">"Horizontal Menu"</h2>
        //         <div class="bg-base-200 rounded-box p-2">
        //             <Menu direction=MenuDirection::Horizontal class="rounded-box">
        //                 <MenuItem>
        //                     <Icon icon=icondata::AiFileOutlined />
        //                     "File"
        //                 </MenuItem>
        //                 <MenuItem>
        //                     <Icon icon=icondata::AiEditOutlined />
        //                     "Edit"
        //                 </MenuItem>
        //                 <MenuItem>
        //                     <Icon icon=icondata::AiEyeOutlined />
        //                     "View"
        //                 </MenuItem>
        //                 <MenuItem>
        //                     <Icon icon=icondata::AiQuestionCircleOutlined />
        //                     "Help"
        //                 </MenuItem>
        //             </Menu>
        //         </div>

        //         <h2 class="text-xl font-semibold">"Menu with Submenu"</h2>
        //         <div class="bg-base-200 rounded-box w-56 p-2">
        //             <Menu direction=MenuDirection::Vertical class="w-full">
        //                 <MenuItem>
        //                     <Icon icon=icondata::AiDashboardOutlined />
        //                     "Dashboard"
        //                 </MenuItem>
        //                 <MenuItem is_submenu=true>
        //                     <MenuTitle>
        //                         <Icon icon=icondata::AiFolderOutlined />
        //                         "Projects"
        //                     </MenuTitle>
        //                     <SubMenu>
        //                         <MenuItem>"Website Redesign"</MenuItem>
        //                         <MenuItem>"Mobile App"</MenuItem>
        //                         <MenuItem>"API Development"</MenuItem>
        //                         <MenuItem>"Database Migration"</MenuItem>
        //                     </SubMenu>
        //                 </MenuItem>
        //                 <MenuItem is_submenu=true>
        //                     <MenuTitle>
        //                         <Icon icon=icondata::AiTeamOutlined />
        //                         "Team"
        //                     </MenuTitle>
        //                     <SubMenu>
        //                         <MenuItem>"Developers"</MenuItem>
        //                         <MenuItem>"Designers"</MenuItem>
        //                         <MenuItem>"Product Managers"</MenuItem>
        //                     </SubMenu>
        //                 </MenuItem>
        //                 <MenuItem>
        //                     <Icon icon=icondata::AiBarChartOutlined />
        //                     "Analytics"
        //                 </MenuItem>
        //             </Menu>
        //         </div>

        //         <h2 class="text-xl font-semibold">"Menu with Icons Only"</h2>
        //         <div class="bg-base-200 rounded-box p-2">
        //             <Menu direction=MenuDirection::Horizontal class="rounded-box">
        //                 <MenuItem>
        //                     <Icon icon=icondata::AiHomeOutlined />
        //                 </MenuItem>
        //                 <MenuItem>
        //                     <Icon icon=icondata::AiHeartOutlined />
        //                 </MenuItem>
        //                 <MenuItem>
        //                     <Icon icon=icondata::AiStarOutlined />
        //                 </MenuItem>
        //                 <MenuItem>
        //                     <Icon icon=icondata::AiShareAltOutlined />
        //                 </MenuItem>
        //                 <MenuItem>
        //                     <Icon icon=icondata::AiMoreOutlined />
        //                 </MenuItem>
        //             </Menu>
        //         </div>

        //         <h2 class="text-xl font-semibold">"Menu Sizes"</h2>
        //         <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
        //             <div>
        //                 <h3 class="font-semibold mb-2">"Small"</h3>
        //                 <div class="bg-base-200 rounded-box p-2">
        //                     <Menu
        //                         direction=MenuDirection::Vertical
        //                         size=MenuSize::Sm
        //                         class="w-full"
        //                     >
        //                         <MenuItem>
        //                             <Icon icon=icondata::AiHomeOutlined />
        //                             "Home"
        //                         </MenuItem>
        //                         <MenuItem>
        //                             <Icon icon=icondata::AiUserOutlined />
        //                             "Profile"
        //                         </MenuItem>
        //                         <MenuItem>
        //                             <Icon icon=icondata::AiSettingOutlined />
        //                             "Settings"
        //                         </MenuItem>
        //                     </Menu>
        //                 </div>
        //             </div>
        //             <div>
        //                 <h3 class="font-semibold mb-2">"Normal"</h3>
        //                 <div class="bg-base-200 rounded-box p-2">
        //                     <Menu direction=MenuDirection::Vertical class="w-full">
        //                         <MenuItem>
        //                             <Icon icon=icondata::AiHomeOutlined />
        //                             "Home"
        //                         </MenuItem>
        //                         <MenuItem>
        //                             <Icon icon=icondata::AiUserOutlined />
        //                             "Profile"
        //                         </MenuItem>
        //                         <MenuItem>
        //                             <Icon icon=icondata::AiSettingOutlined />
        //                             "Settings"
        //                         </MenuItem>
        //                     </Menu>
        //                 </div>
        //             </div>
        //             <div>
        //                 <h3 class="font-semibold mb-2">"Large"</h3>
        //                 <div class="bg-base-200 rounded-box p-2">
        //                     <Menu
        //                         direction=MenuDirection::Vertical
        //                         size=MenuSize::Lg
        //                         class="w-full"
        //                     >
        //                         <MenuItem>
        //                             <Icon icon=icondata::AiHomeOutlined />
        //                             "Home"
        //                         </MenuItem>
        //                         <MenuItem>
        //                             <Icon icon=icondata::AiUserOutlined />
        //                             "Profile"
        //                         </MenuItem>
        //                         <MenuItem>
        //                             <Icon icon=icondata::AiSettingOutlined />
        //                             "Settings"
        //                         </MenuItem>
        //                     </Menu>
        //                 </div>
        //             </div>
        //         </div>

        //         <h2 class="text-xl font-semibold">"Menu with Badges"</h2>
        //         <div class="bg-base-200 rounded-box w-56 p-2">
        //             <Menu direction=MenuDirection::Vertical class="w-full">
        //                 <MenuItem>
        //                     <Icon icon=icondata::AiInboxOutlined />
        //                     "Inbox"
        //                     <Badge color=BadgeColor::Primary size=BadgeSize::Sm>"12"</Badge>
        //                 </MenuItem>
        //                 <MenuItem>
        //                     <Icon icon=icondata::AiBellOutlined />
        //                     "Notifications"
        //                     <Badge color=BadgeColor::Warning size=BadgeSize::Sm>"3"</Badge>
        //                 </MenuItem>
        //                 <MenuItem>
        //                     <Icon icon=icondata::AiMessageOutlined />
        //                     "Messages"
        //                     <Badge color=BadgeColor::Success size=BadgeSize::Sm>"New"</Badge>
        //                 </MenuItem>
        //                 <MenuItem>
        //                     <Icon icon=icondata::AiCalendarOutlined />
        //                     "Calendar"
        //                 </MenuItem>
        //             </Menu>
        //         </div>

        //         <h2 class="text-xl font-semibold">"Interactive Example"</h2>
        //         <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
        //             <div>
        //                 <h3 class="font-semibold mb-2">"Navigation Menu"</h3>
        //                 <div class="bg-base-200 rounded-box w-64 p-2">
        //                     <Menu
        //                         selected=selected
        //                         direction=MenuDirection::Vertical
        //                         class="w-full"
        //                         on:select=move |value: String| {
        //                             set_selected(Some(value));
        //                         }
        //                     >
        //                         <MenuItem value="dashboard">
        //                             <Icon icon=icondata::AiDashboardOutlined />
        //                             "Dashboard"
        //                         </MenuItem>
        //                         <MenuItem value="projects">
        //                             <Icon icon=icondata::AiFolderOutlined />
        //                             "Projects"
        //                         </MenuItem>
        //                         <MenuItem value="team">
        //                             <Icon icon=icondata::AiTeamOutlined />
        //                             "Team"
        //                         </MenuItem>
        //                         <MenuItem value="analytics">
        //                             <Icon icon=icondata::AiBarChartOutlined />
        //                             "Analytics"
        //                         </MenuItem>
        //                         <MenuItem value="settings">
        //                             <Icon icon=icondata::AiSettingOutlined />
        //                             "Settings"
        //                         </MenuItem>
        //                     </Menu>
        //                 </div>
        //             </div>
        //             <div>
        //                 <h3 class="font-semibold mb-2">"Selected Item"</h3>
        //                 <div class="bg-base-100 rounded-box p-4">
        //                     <p class="text-sm opacity-60">"Currently selected:"</p>
        //                     <p class="font-semibold">
        //                         {move || selected().unwrap_or_else(|| "None".to_string())}
        //                     </p>
        //                 </div>
        //             </div>
        //         </div>
        //     </div>
        // </div>
    }
}
