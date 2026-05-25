use leptos::prelude::{ElementChild, ClassAttribute};
use leptos::prelude::*;

#[component]
pub fn Card(title: &'static str, children: Children) -> impl IntoView {
    view! {
        <div class="bg-white dark:bg-slate-900 p-6 rounded-2xl border border-slate-200 dark:border-slate-800 shadow-sm transition-colors duration-300">
            <h3 class="text-lg font-semibold text-slate-800 dark:text-slate-100 mb-4">{title}</h3>
            <div class="space-y-4">
                {children()}
            </div>
        </div>
    }
}
