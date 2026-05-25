use leptos::prelude::*;
use crate::ui::shared::modal::Modal;
use crate::ui::shared::button::Button;
use crate::ui::shared::demo_page::DemoPage;

#[component]
pub fn ModalView() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);
    
    let close_modal = move |_| set_is_open.set(false);

    view! {
        <div class="min-h-screen p-4 lg:p-8 max-w-7xl mx-auto space-y-6">
            <DemoPage 
                title="Modal Component" 
                description="A focused overlay for critical interactions, forms, or detailed views."
                code=r#"view! {
    let (is_open, set_is_open) = signal(false);
    
    <div class="space-y-4">
        <Button on:click=move |_| set_is_open.set(true)>
            "Open Modal"
        </Button>
        
        <Modal 
            is_open=is_open 
            on_close=Callback::new(move |_| set_is_open.set(false))
        >
            <div class="space-y-4">
                <p class="text-surface-500">"This is a modal window. It provides a focused area for user interaction."</p>
                <Button variant="primary" on:click=move |_| set_is_open.set(false)>
                    "Confirm"
                </Button>
            </div>
        </Modal>
    </div>
}"#
            >
                <div class="space-y-4">
                    <Button on:click=move |_| set_is_open.set(true)>
                        "Open Modal"
                    </Button>
                    
                    <Modal 
                        is_open=is_open 
                        on_close=Callback::new(close_modal)
                    >
                        <div class="space-y-4">
                            <p class="text-surface-500">"This is a modal window. It provides a focused area for user interaction."</p>
                            <Button variant="primary" on:click=move |_| set_is_open.set(false)>
                                "Confirm"
                            </Button>
                        </div>
                    </Modal>
                </div>
            </DemoPage>
        </div>
    }
}
