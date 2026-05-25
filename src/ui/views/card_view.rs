use leptos::prelude::*;
use crate::ui::shared::card::Card;
use crate::ui::shared::demo_page::DemoPage;

#[component]
pub fn CardView() -> impl IntoView {
    view! {
        <div class="min-h-screen p-4 lg:p-8 max-w-7xl mx-auto space-y-6">
            <DemoPage 
                title="Card Component" 
                description="A clean, bordered container for grouping related information."
                code=r#"view! {
    <Card title="Project Details">
        <p class="text-surface-500 leading-relaxed">
            "This is a sample card content. It can contain any layout or other components."
        </p>
        <div class="flex gap-2">
            <span class="text-xs bg-surface-100 dark:bg-surface-800 px-2 py-1 rounded">"Tag 1"</span>
            <span class="text-xs bg-surface-100 dark:bg-surface-800 px-2 py-1 rounded">"Tag 2"</span>
        </div>
    </Card>
}"#
            >
                <Card title="Project Details">
                    <p class="text-surface-500 leading-relaxed">
                        "This is a sample card content. It can contain any layout or other components."
                    </p>
                    <div class="flex gap-2">
                        <span class="text-xs bg-surface-100 dark:bg-surface-800 px-2 py-1 rounded">"Tag 1"</span>
                        <span class="text-xs bg-surface-100 dark:bg-surface-800 px-2 py-1 rounded">"Tag 2"</span>
                    </div>
                </Card>
            </DemoPage>
        </div>
    }
}
