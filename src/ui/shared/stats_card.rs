use leptos::prelude::*;

#[component]
pub fn StatsCard<
    T: ToString,
>(
    title: &'static str,
    value: T,
    trend: &'static str,
    color: &'static str,
) -> impl IntoView {
    let value_str = value.to_string();
    view! {
        <div class="bg-white p-6 rounded-xl border border-surface-200 shadow-sm">
            <p class="text-sm font-medium text-surface-500">{title}</p>
            <div class="mt-2 flex items-baseline justify-between">
                <h3 class="text-2xl font-bold text-surface-900">{value_str}</h3>
                <span class=format!("text-xs font-semibold px-2 py-1 rounded-full {}", color)>
                    {trend}
                </span>
            </div>
        </div>
    }
}
