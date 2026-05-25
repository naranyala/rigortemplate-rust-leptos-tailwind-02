use leptos::prelude::*;
use crate::hooks::use_local_storage;
use crate::ui::shared::demo_page::DemoPage;
use crate::ui::shared::button::Button;

#[component]
pub fn LocalStorageView() -> impl IntoView {
    let (name, set_name) = use_local_storage("demo_name", "Guest".to_string());

    view! {
        <div class="min-h-screen p-4 lg:p-8 max-w-7xl mx-auto space-y-6">
            <DemoPage 
                title="use_local_storage Hook" 
                description="A hook to persist signal state across browser refreshes using LocalStorage."
                code=r#"view! {
    let (name, set_name) = use_local_storage("demo_name", "Guest".to_string());

    <div class="space-y-4">
        <p class="text-lg">"Hello, " {move || name.get()} "!"</p>
        <input 
            type="text" 
            class="px-4 py-2 border rounded-xl"
            on:input=move |ev| set_name.set(event_target_value(&ev))
        />
        <Button on:click=move |_| set_name.set("Guest".to_string())>
            "Reset to Guest"
        </Button>
    </div>
}"#
            >
                <div class="space-y-4">
                    <p class="text-lg">"Hello, " {move || name.get()} "!"</p>
                    <input 
                        type="text" 
                        class="px-4 py-2 bg-surface-100 dark:bg-surface-800 border border-surface-200 dark:border-surface-700 rounded-xl text-surface-900 dark:text-white"
                        on:input=move |ev| set_name.set(event_target_value(&ev))
                        placeholder="Type your name..."
                    />
                    <div class="flex gap-2">
                        <Button on:click=move |_| set_name.set("Guest".to_string())>
                            "Reset to Guest"
                        </Button>
                    </div>
                </div>
            </DemoPage>
        </div>
    }
}
