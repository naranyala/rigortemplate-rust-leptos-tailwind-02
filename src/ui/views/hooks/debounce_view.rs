use leptos::prelude::*;
use crate::hooks::use_debounce;
use crate::ui::shared::demo_page::DemoPage;

#[component]
pub fn DebounceView() -> impl IntoView {
    let (val, set_val) = use_debounce("Type something...".to_string(), 500);

    view! {
        <div class="min-h-screen p-4 lg:p-8 max-w-7xl mx-auto space-y-6">
            <DemoPage 
                title="use_debounce Hook" 
                description="Limits the frequency of updates for a signal, preventing expensive operations on every keystroke."
                code=r#"view! {
    let (val, set_val) = use_debounce("Type something...".to_string(), 500);

    <div class="space-y-4">
        <input 
            type="text" 
            class="px-4 py-2 bg-surface-100 dark:bg-surface-800 border border-surface-200 dark:border-surface-700 rounded-xl"
            on:input=move |ev| set_val.set(event_target_value(&ev))
        />
        <p class="text-lg font-mono">"Debounced Value: " {move || val.get()}</p>
    </div>
}"#
            >
                <div class="space-y-4">
                    <input 
                        type="text" 
                        class="px-4 py-2 bg-surface-100 dark:bg-surface-800 border border-surface-200 dark:border-surface-700 rounded-xl text-surface-900 dark:text-white w-full max-w-md"
                        on:input=move |ev| set_val.set(event_target_value(&ev))
                        placeholder="Type fast to see debounce..."
                    />
                    <div class="p-4 bg-surface-100 dark:bg-surface-800 rounded-2xl border border-surface-200 dark:border-surface-700 inline-block">
                        <p class="text-lg font-mono text-accent-600">"Debounced Value: " {move || val.get()}</p>
                    </div>
                </div>
            </DemoPage>
        </div>
    }
}
