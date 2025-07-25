use crate::core::{ContentLayout, Section};
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn ModalDemo() -> impl IntoView {
    let (modal_1_open, set_modal_1_open) = signal(false);

    view! {
        <ContentLayout
            title="Modal"
            description="Modals are used to show content in a layer above the page"
        >

            <Section title="Basic Modal">

                <Button color=ButtonColor::Primary on:click=move |_| set_modal_1_open.set(true)>
                    "Open Modal"
                </Button>

                <Modal open=modal_1_open on:close=move |_| set_modal_1_open.set(false)>
                    <ModalBox>
                        <h3 class="text-lg font-bold">"Hello!"</h3>
                        <p class="py-4">"Press ESC key or click the button below to close"</p>
                        <ModalAction>
                            <form method="dialog">
                                <Button on:click=move |_| {
                                    set_modal_1_open.set(false)
                                }>

                                    "Close"
                                </Button>
                            </form>
                        </ModalAction>
                    </ModalBox>
                </Modal>
            </Section>
        </ContentLayout>
    }
}
