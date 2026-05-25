use leptos::prelude::*;
use crate::ui::shared::badge::Badge;
use crate::ui::shared::demo_page::DemoPage;

#[component]
pub fn BadgeView() -> impl IntoView {
    view! {
        <div class="min-h-screen p-4 lg:p-8 max-w-7xl mx-auto space-y-6">
            <DemoPage 
                title="Badge Component" 
                description="Small, colorful indicators for status, categories, or counts."
                code=r#"view! {
    <div class="flex flex-wrap gap-4">
        <Badge text="Active" color="green" />
        <Badge text="Critical" color="red" />
        <Badge text="Info" color="blue" />
        <Badge text="Neutral" color="gray" />
    </div>
}"#
            >
                <div class="flex flex-wrap gap-4">
                    <Badge text="Active" color="green" />
                    <Badge text="Critical" color="red" />
                    <Badge text="Info" color="blue" />
                    <Badge text="Neutral" color="gray" />
                </div>
            </DemoPage>
        </div>
    }
}
