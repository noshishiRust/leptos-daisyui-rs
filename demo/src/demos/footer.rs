use leptos::prelude::*;
use leptos_daisyui_rs::components::*;
use leptos_icons::Icon;

#[component]
pub fn FooterDemo() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <h1 class="text-3xl font-bold">"Footer"</h1>
            <p class="text-base-content/70">
                "Footer component can be used at the bottom of a page to show information"
            </p>

            <div class="space-y-8">
                <h2 class="text-xl font-semibold">"Basic Footer"</h2>
                <Footer class="bg-base-200 text-base-content p-10">
                    <nav>
                        <h6 class="footer-title">"Services"</h6>
                        <a class="link link-hover">"Branding"</a>
                        <a class="link link-hover">"Design"</a>
                        <a class="link link-hover">"Marketing"</a>
                        <a class="link link-hover">"Advertisement"</a>
                    </nav>
                    <nav>
                        <h6 class="footer-title">"Company"</h6>
                        <a class="link link-hover">"About us"</a>
                        <a class="link link-hover">"Contact"</a>
                        <a class="link link-hover">"Jobs"</a>
                        <a class="link link-hover">"Press kit"</a>
                    </nav>
                    <nav>
                        <h6 class="footer-title">"Legal"</h6>
                        <a class="link link-hover">"Terms of use"</a>
                        <a class="link link-hover">"Privacy policy"</a>
                        <a class="link link-hover">"Cookie policy"</a>
                    </nav>
                </Footer>

                <h2 class="text-xl font-semibold">"Footer with Logo"</h2>
                <Footer class="bg-base-200 text-base-content p-10">
                    <aside>
                        <Icon icon=icondata::CgComponents attr:class="w-12 h-12" />
                        <p>"ACME Industries Ltd." <br /> "Providing reliable tech since 1992"</p>
                    </aside>
                    <nav>
                        <h6 class="footer-title">"Services"</h6>
                        <a class="link link-hover">"Branding"</a>
                        <a class="link link-hover">"Design"</a>
                        <a class="link link-hover">"Marketing"</a>
                        <a class="link link-hover">"Advertisement"</a>
                    </nav>
                    <nav>
                        <h6 class="footer-title">"Company"</h6>
                        <a class="link link-hover">"About us"</a>
                        <a class="link link-hover">"Contact"</a>
                        <a class="link link-hover">"Jobs"</a>
                        <a class="link link-hover">"Press kit"</a>
                    </nav>
                    <nav>
                        <h6 class="footer-title">"Legal"</h6>
                        <a class="link link-hover">"Terms of use"</a>
                        <a class="link link-hover">"Privacy policy"</a>
                        <a class="link link-hover">"Cookie policy"</a>
                    </nav>
                </Footer>

                <h2 class="text-xl font-semibold">"Footer with Social Icons"</h2>
                <Footer class="bg-base-200 text-base-content p-10">
                    <aside>
                        <Icon icon=icondata::CgComponents attr:class="w-12 h-12" />
                        <p>"ACME Industries Ltd." <br /> "Providing reliable tech since 1992"</p>
                    </aside>
                    <nav>
                        <h6 class="footer-title">"Services"</h6>
                        <a class="link link-hover">"Branding"</a>
                        <a class="link link-hover">"Design"</a>
                        <a class="link link-hover">"Marketing"</a>
                        <a class="link link-hover">"Advertisement"</a>
                    </nav>
                    <nav>
                        <h6 class="footer-title">"Company"</h6>
                        <a class="link link-hover">"About us"</a>
                        <a class="link link-hover">"Contact"</a>
                        <a class="link link-hover">"Jobs"</a>
                        <a class="link link-hover">"Press kit"</a>
                    </nav>
                    <nav>
                        <h6 class="footer-title">"Social"</h6>
                        <div class="grid grid-flow-col gap-4">
                            <a href="#" class="link">
                                <Icon icon=icondata::AiTwitterOutlined attr:class="w-6 h-6" />
                            </a>
                            <a href="#" class="link">
                                <Icon icon=icondata::AiFacebookOutlined attr:class="w-6 h-6" />
                            </a>
                            <a href="#" class="link">
                                <Icon icon=icondata::AiInstagramOutlined attr:class="w-6 h-6" />
                            </a>
                        </div>
                    </nav>
                </Footer>

                <h2 class="text-xl font-semibold">"Centered Footer"</h2>
                <Footer class="bg-base-200 text-base-content footer-center p-10">
                    <aside>
                        <Icon icon=icondata::CgComponents attr:class="w-12 h-12" />
                        <p class="font-bold">
                            "ACME Industries Ltd." <br /> "Providing reliable tech since 1992"
                        </p>
                        <p>"Copyright © 2024 - All right reserved"</p>
                    </aside>
                    <nav>
                        <div class="grid grid-flow-col gap-4">
                            <a href="#" class="link">
                                <Icon icon=icondata::AiTwitterOutlined attr:class="w-6 h-6" />
                            </a>
                            <a href="#" class="link">
                                <Icon icon=icondata::AiFacebookOutlined attr:class="w-6 h-6" />
                            </a>
                            <a href="#" class="link">
                                <Icon icon=icondata::AiInstagramOutlined attr:class="w-6 h-6" />
                            </a>
                        </div>
                    </nav>
                </Footer>

                <h2 class="text-xl font-semibold">"Footer with Newsletter"</h2>
                <Footer class="bg-base-200 text-base-content p-10">
                    <aside>
                        <Icon icon=icondata::CgComponents attr:class="w-12 h-12" />
                        <p>"ACME Industries Ltd." <br /> "Providing reliable tech since 1992"</p>
                    </aside>
                    <nav>
                        <h6 class="footer-title">"Services"</h6>
                        <a class="link link-hover">"Branding"</a>
                        <a class="link link-hover">"Design"</a>
                        <a class="link link-hover">"Marketing"</a>
                        <a class="link link-hover">"Advertisement"</a>
                    </nav>
                    <nav>
                        <h6 class="footer-title">"Company"</h6>
                        <a class="link link-hover">"About us"</a>
                        <a class="link link-hover">"Contact"</a>
                        <a class="link link-hover">"Jobs"</a>
                        <a class="link link-hover">"Press kit"</a>
                    </nav>
                    <form>
                        <h6 class="footer-title">"Newsletter"</h6>
                        <fieldset class="form-control w-80">
                            <label class="label">
                                <span class="label-text">"Enter your email address"</span>
                            </label>
                            <div class="join">
                                <Input
                                    placeholder="username@site.com"
                                    class="input input-bordered join-item"
                                />
                                <Button color=ButtonColor::Primary class="join-item">
                                    "Subscribe"
                                </Button>
                            </div>
                        </fieldset>
                    </form>
                </Footer>

                <h2 class="text-xl font-semibold">"Dark Footer"</h2>
                <Footer class="bg-neutral text-neutral-content p-10">
                    <aside>
                        <Icon icon=icondata::CgComponents attr:class="w-12 h-12" />
                        <p>"ACME Industries Ltd." <br /> "Providing reliable tech since 1992"</p>
                    </aside>
                    <nav>
                        <h6 class="footer-title">"Services"</h6>
                        <a class="link link-hover">"Branding"</a>
                        <a class="link link-hover">"Design"</a>
                        <a class="link link-hover">"Marketing"</a>
                        <a class="link link-hover">"Advertisement"</a>
                    </nav>
                    <nav>
                        <h6 class="footer-title">"Company"</h6>
                        <a class="link link-hover">"About us"</a>
                        <a class="link link-hover">"Contact"</a>
                        <a class="link link-hover">"Jobs"</a>
                        <a class="link link-hover">"Press kit"</a>
                    </nav>
                    <nav>
                        <h6 class="footer-title">"Legal"</h6>
                        <a class="link link-hover">"Terms of use"</a>
                        <a class="link link-hover">"Privacy policy"</a>
                        <a class="link link-hover">"Cookie policy"</a>
                    </nav>
                </Footer>

                <h2 class="text-xl font-semibold">"Footer with Bottom Bar"</h2>
                <div>
                    <Footer class="bg-base-200 text-base-content p-10">
                        <aside>
                            <Icon icon=icondata::CgComponents attr:class="w-12 h-12" />
                            <p>
                                "ACME Industries Ltd." <br /> "Providing reliable tech since 1992"
                            </p>
                        </aside>
                        <nav>
                            <h6 class="footer-title">"Services"</h6>
                            <a class="link link-hover">"Branding"</a>
                            <a class="link link-hover">"Design"</a>
                            <a class="link link-hover">"Marketing"</a>
                            <a class="link link-hover">"Advertisement"</a>
                        </nav>
                        <nav>
                            <h6 class="footer-title">"Company"</h6>
                            <a class="link link-hover">"About us"</a>
                            <a class="link link-hover">"Contact"</a>
                            <a class="link link-hover">"Jobs"</a>
                            <a class="link link-hover">"Press kit"</a>
                        </nav>
                        <nav>
                            <h6 class="footer-title">"Legal"</h6>
                            <a class="link link-hover">"Terms of use"</a>
                            <a class="link link-hover">"Privacy policy"</a>
                            <a class="link link-hover">"Cookie policy"</a>
                        </nav>
                    </Footer>
                    <Footer class="bg-base-300 text-base-content border-base-300 border-t px-10 py-4">
                        <aside class="grid-flow-col items-center">
                            <p>"Copyright © 2024 - All right reserved by ACME Industries Ltd"</p>
                        </aside>
                        <nav class="md:place-self-center md:justify-self-end">
                            <div class="grid grid-flow-col gap-4">
                                <a href="#" class="link">
                                    <Icon icon=icondata::AiTwitterOutlined attr:class="w-6 h-6" />
                                </a>
                                <a href="#" class="link">
                                    <Icon icon=icondata::AiFacebookOutlined attr:class="w-6 h-6" />
                                </a>
                                <a href="#" class="link">
                                    <Icon icon=icondata::AiInstagramOutlined attr:class="w-6 h-6" />
                                </a>
                            </div>
                        </nav>
                    </Footer>
                </div>
            </div>
        </div>
    }
}
