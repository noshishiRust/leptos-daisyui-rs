use crate::core::ContentLayout;
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn DropdownDemo() -> impl IntoView {
    view! {
        <ContentLayout
            title="Dropdown"
            description="Dropdown can open a menu or any other element when the button is clicked"
        >

            <h2 class="text-xl font-semibold">"Basic Dropdown"</h2>

            <Dropdown>
                <Button>"Menu"</Button>
                <DropdownContent is_menu=true class="bg-base-100 rounded-box z-[1] w-52 p-2 shadow">
                    <MenuItem>"Item1"</MenuItem>
                    <MenuItem>"Item2"</MenuItem>
                </DropdownContent>
            </Dropdown>

            <h2 class="text-xl font-semibold">"Dropdown Positions"</h2>
            <div class="flex flex-wrap gap-4">
                <Dropdown>
                    <Button>"Top"</Button>
                    <DropdownContent is_menu=true class="bg-base-100 rounded-box p-2 shadow">
                        <MenuItem>"Item1"</MenuItem>
                        <MenuItem>"Item2"</MenuItem>
                    </DropdownContent>
                </Dropdown>

                <Dropdown alignment=DropdownAlignment::End>
                    <Button>"End"</Button>
                    <DropdownContent is_menu=true class="bg-base-100 rounded-box p-2 shadow">
                        <MenuItem>"Item1"</MenuItem>
                        <MenuItem>"Item2"</MenuItem>
                    </DropdownContent>
                </Dropdown>

                <Dropdown placement=DropdownPlacement::Left>
                    <Button>"Left"</Button>
                    <DropdownContent is_menu=true class="bg-base-100 rounded-box p-2 shadow">
                        <MenuItem>"Item1"</MenuItem>
                        <MenuItem>"Item2"</MenuItem>
                    </DropdownContent>
                </Dropdown>

                <Dropdown placement=DropdownPlacement::Right>
                    <Button>"Right"</Button>
                    <DropdownContent is_menu=true class="bg-base-100 rounded-box p-2 shadow">
                        <MenuItem>"Item1"</MenuItem>
                        <MenuItem>"Item2"</MenuItem>
                    </DropdownContent>
                </Dropdown>
            </div>
        </ContentLayout>
    }
}
