use leptos::prelude::{ElementChild, ClassAttribute};
use leptos::prelude::*;

#[component]
pub fn StatsCard(title: &'static str, value: &'static str, trend: &'static str, color: &'static str) -> impl IntoView {
    view! {
        <div class="bg-white p-6 rounded-xl border border-slate-200 shadow-sm">
            <p class="text-sm font-medium text-slate-500">{title}</p>
            <div class="mt-2 flex items-baseline justify-between">
                <h3 class="text-2xl font-bold text-slate-900">{value}</h3>
                <span class=format!("text-xs font-semibold px-2 py-1 rounded-full {}", color)>
                    {trend}
                </span>
            </div>
        </div>
    }
}
