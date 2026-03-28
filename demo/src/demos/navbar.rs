use crate::core::{ContentLayout, Section};
use leptos::prelude::*;

#[component]
pub fn NavbarDemo() -> impl IntoView {
    view! {
        <ContentLayout
            title="Navbar"
            description="Navigation bar component for displaying navigation links and information at the top of a page"
        >

            // <Navbar>
            <Section title="Basic Navbar">
                "a"
            // <Button style=ButtonStyle::Ghost size=ButtonSize::Lg>
            // "daisyUI"
            // </Button>
            // </Navbar>
            </Section>

            <Section title="Navbar with Menu">

                "a"
            // <Navbar>
            // <NavbarStart>
            // <Dropdown>
            // <DropdownButton style=ButtonStyle::Ghost class="lg:hidden">
            // <Icon icon=icondata::AiMenuOutlined />
            // </DropdownButton>
            // <DropdownContent class="w-52">
            // <Menu
            // size=MenuSize::Sm
            // class="mt-3 z-[1] p-2 shadow bg-base-100 rounded-box"
            // >
            // <MenuItem>"Item 1"</MenuItem>
            // <MenuItem>"Item 2"</MenuItem>
            // <MenuItem>"Item 3"</MenuItem>
            // </Menu>
            // </DropdownContent>
            // </Dropdown>
            // <Button style=ButtonStyle::Ghost size=ButtonSize::Lg>
            // "daisyUI"
            // </Button>
            // </NavbarStart>
            // <NavbarCenter class="hidden lg:flex">
            // <Menu orientation=MenuOrientation::Horizontal class="px-1">
            // <MenuItem>"Item 1"</MenuItem>
            // <MenuItem>"Item 2"</MenuItem>
            // <MenuItem>"Item 3"</MenuItem>
            // </Menu>
            // </NavbarCenter>
            // <NavbarEnd>
            // <Button>"Button"</Button>
            // </NavbarEnd>
            // </Navbar>
            // </Section>

            // <Section title="Navbar with Avatar and Indicators">
            // <Navbar>
            // <NavbarStart>
            // <Button style=ButtonStyle::Ghost shape=ButtonShape::Circle>
            // <Icon icon=icondata::AiMenuOutlined />
            // </Button>
            // </NavbarStart>
            // <NavbarCenter>
            // <Button style=ButtonStyle::Ghost size=ButtonSize::Lg>
            // "My App"
            // </Button>
            // </NavbarCenter>
            // <NavbarEnd>
            // <Button style=ButtonStyle::Ghost shape=ButtonShape::Circle>
            // <Icon icon=icondata::AiSearchOutlined />
            // </Button>
            // <Button style=ButtonStyle::Ghost shape=ButtonShape::Circle>
            // <Indicator>
            // <Icon icon=icondata::AiMailOutlined />
            // <IndicatorItem
            // vertical=IndicatorVerticalPlacement::Top
            // horizontal=IndicatorHorizontalPlacement::End
            // >
            // <Badge size=BadgeSize::Xs color=BadgeColor::Primary />
            // </IndicatorItem>
            // </Indicator>
            // </Button>
            // </NavbarEnd>
            // </Navbar>
            // </Section>

            // <Section title="Navbar with Search">
            // <Navbar>
            // <NavbarStart>
            // <Button style=ButtonStyle::Ghost size=ButtonSize::Lg>
            // "Logo"
            // </Button>
            // </NavbarStart>
            // <NavbarCenter>
            // <Input placeholder="Search" class="w-24 md:w-auto" />
            // </NavbarCenter>
            // <NavbarEnd>
            // <Dropdown position=DropdownPosition::End>
            // <DropdownButton style=ButtonStyle::Ghost shape=ButtonShape::Circle>
            // <Avatar size=AvatarSize::Xs>
            // <img
            // alt="User avatar"
            // src="https://picsum.photos/40/40?random=1"
            // />
            // </Avatar>
            // </DropdownButton>
            // <DropdownContent class="w-52">
            // <Menu
            // size=MenuSize::Sm
            // class="mt-3 z-[1] p-2 shadow bg-base-100 rounded-box"
            // >
            // <MenuItem>
            // "Profile" <Badge size=BadgeSize::Xs>"New"</Badge>
            // </MenuItem>
            // <MenuItem>"Settings"</MenuItem>
            // <MenuItem>"Logout"</MenuItem>
            // </Menu>
            // </DropdownContent>
            // </Dropdown>
            // </NavbarEnd>
            // </Navbar>
            // </Section>

            // <Section title="Navbar with Breadcrumbs">
            // <Navbar>
            // <NavbarStart>
            // <Button style=ButtonStyle::Ghost size=ButtonSize::Lg>
            // "Brand"
            // </Button>
            // </NavbarStart>
            // <NavbarCenter>
            // <Breadcrumbs size=BreadcrumbsSize::Sm>
            // <BreadcrumbItem>"Home"</BreadcrumbItem>
            // <BreadcrumbItem>"Documents"</BreadcrumbItem>
            // <BreadcrumbItem>"Current Page"</BreadcrumbItem>
            // </Breadcrumbs>
            // </NavbarCenter>
            // <NavbarEnd>
            // <Button size=ButtonSize::Sm>"Action"</Button>
            // </NavbarEnd>
            // </Navbar>
            // </Section>

            // <Section title="Colored Navbars">
            // <div class="space-y-4">
            // <Navbar class="bg-primary text-primary-content">
            // <Button style=ButtonStyle::Ghost size=ButtonSize::Lg>
            // "Primary Navbar"
            // </Button>
            // </Navbar>

            // <Navbar class="bg-secondary text-secondary-content">
            // <Button style=ButtonStyle::Ghost size=ButtonSize::Lg>
            // "Secondary Navbar"
            // </Button>
            // </Navbar>

            // <Navbar class="bg-accent text-accent-content">
            // <Button style=ButtonStyle::Ghost size=ButtonSize::Lg>
            // "Accent Navbar"
            // </Button>
            // </Navbar>
            // </div>
            </Section>

        </ContentLayout>
    }
}
