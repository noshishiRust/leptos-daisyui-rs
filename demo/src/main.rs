mod core;
mod demos;

use core::Layout;
use demos::*;
use leptos::mount::mount_to_body;
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;
use leptos_icons::Icon;
use leptos_meta::*;
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes},
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
    provide_meta_context();

    view! {
        <Title text="Leptos x daisyUI" />

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
