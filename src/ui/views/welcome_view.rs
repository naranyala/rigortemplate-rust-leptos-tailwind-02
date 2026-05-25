use leptos::prelude::*;
use crate::ui::registry::{ALL_DEMOS, DemoCategory};
use crate::ui::shared::stats_card::StatsCard;

#[component]
pub fn WelcomeView() -> impl IntoView {
    // Count components and hooks at compile time
    const COMPONENT_COUNT: usize = {
        let mut count = 0;
        let mut i = 0;
        while i < ALL_DEMOS.len() {
            if matches!(ALL_DEMOS[i].category, DemoCategory::Component) {
                count += 1;
            }
            i += 1;
        }
        count
    };
    
    const HOOK_COUNT: usize = {
        let mut count = 0;
        let mut i = 0;
        while i < ALL_DEMOS.len() {
            if matches!(ALL_DEMOS[i].category, DemoCategory::Hook) {
                count += 1;
            }
            i += 1;
        }
        count
    };

    view! {
        <div class="min-h-screen flex items-center justify-center">
            <div class="w-full max-w-4xl space-y-8 px-4 py-12">
                <div class="text-center">
                    <h1 class="text-4xl font-bold text-surface-800 dark:text-surface-100">
                        "Welcome to the App"
                    </h1>
                    <p class="text-surface-500 dark:text-surface-400">
                        "Select a page from the sidebar to get started."
                    </p>
                </div>
                
                <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                    <StatsCard 
                        title="Components Demo" 
                        value=COMPONENT_COUNT.to_string()
                        trend="available"
                        color="bg-blue-100 text-blue-800"
                    />
                    <StatsCard 
                        title="Hooks Demo" 
                        value=HOOK_COUNT.to_string()
                        trend="available"
                        color="bg-green-100 text-green-800"
                    />
                </div>
            </div>
        </div>
    }
}