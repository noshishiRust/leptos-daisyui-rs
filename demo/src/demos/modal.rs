use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn ModalDemo() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <h1 class="text-3xl font-bold">"Modal"</h1>
            <p class="text-base-content/70">
                "Modals are used to show content in a layer above the page"
            </p>

            <div class="space-y-4">
                <h2 class="text-xl font-semibold">"Basic Modal"</h2>
                <Button color=Signal::derive(move || {
                    ButtonColor::Primary
                })>

                    "Open Modal"
                </Button>

                <Modal>
                    <ModalBox>
                        <h3 class="text-lg font-bold">"Hello!"</h3>
                        <p class="py-4">"Press ESC key or click the button below to close"</p>
                        <ModalAction>
                            <form method="dialog">
                                <Button>"Close"</Button>
                            </form>
                        </ModalAction>
                    </ModalBox>
                </Modal>

                <h2 class="text-xl font-semibold">"Modal with Custom Close"</h2>
                <Button color=Signal::derive(move || ButtonColor::Secondary)>"Open Modal 2"</Button>

                <Modal>
                    <form>
                        <Button class="btn-sm btn-circle btn-ghost absolute right-2 top-2">
                            "✕"
                        </Button>
                    </form>
                    <ModalBox>
                        <h3 class="text-lg font-bold">"Hello!"</h3>
                        <p class="py-4">"This modal has a custom close button"</p>
                    </ModalBox>
                </Modal>
            </div>
        </div>
    }
}