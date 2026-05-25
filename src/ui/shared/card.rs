use leptos::prelude::*;

#[component]
pub fn Card(title: &'static str, children: Children) -> impl IntoView {
    view! {
        <div class="bg-white dark:bg-surface-900 p-6 rounded-2xl border border-surface-200 dark:border-surface-800 shadow-sm transition-colors duration-300">
            <h3 class="text-lg font-semibold text-surface-800 dark:text-surface-100 mb-4">{title}</h3>
            <div class="space-y-4">
                {children()}
            </div>
        </div>
    }
}
