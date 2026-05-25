use leptos::prelude::*;
use crate::ui::shared::button::Button;
use crate::ui::shared::demo_page::DemoPage;

#[component]
pub fn ButtonView() -> impl IntoView {
    view! {
        <div class="min-h-screen p-4 lg:p-8 max-w-7xl mx-auto space-y-6">
            <DemoPage 
                title="Button Component" 
                description="A versatile button component with multiple variants for different action priorities."
                code=""
            >
            >
                <div class="flex flex-wrap gap-4">
                    <Button variant="primary">"Primary Action"</Button>
                    <Button variant="secondary">"Secondary Action"</Button>
                    <Button variant="outline">"Outline Action"</Button>
                </div>
            </DemoPage>
        </div>
    }
}
