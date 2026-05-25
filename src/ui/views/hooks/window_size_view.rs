use leptos::prelude::*;
use crate::hooks::use_window_size;
use crate::ui::shared::demo_page::DemoPage;

#[component]
pub fn WindowSizeView() -> impl IntoView {
    let (width, height) = use_window_size();

    view! {
        <div class="min-h-screen p-4 lg:p-8 max-w-7xl mx-auto space-y-6">
            <DemoPage 
                title="use_window_size Hook" 
                description="Real-time tracking of the browser window dimensions."
                code=r#"view! {
    let (width, height) = use_window_size();

    <div class="flex gap-8 text-2xl font-mono">
        <div>"Width: " {move || width.get()} "px"</div>
        <div>"Height: " {move || height.get()} "px"</div>
    </div>
}"#
            >
                <div class="flex gap-8 text-2xl font-mono">
                    <div class="p-4 bg-surface-100 dark:bg-surface-800 rounded-2xl border border-surface-200 dark:border-surface-700">
                        "Width: " {move || width.get()} "px"
                    </div>
                    <div class="p-4 bg-surface-100 dark:bg-surface-800 rounded-2xl border border-surface-200 dark:border-surface-700">
                        "Height: " {move || height.get()} "px"
                    </div>
                </div>
            </DemoPage>
        </div>
    }
}
