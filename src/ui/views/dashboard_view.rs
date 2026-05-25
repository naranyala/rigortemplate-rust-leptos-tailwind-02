use leptos::prelude::*;
use crate::ui::shared::StatsCard;

#[component]
pub fn DashboardView() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <div class="p-6 space-y-6">
            <div class="flex items-center justify-between">
                <h2 class="text-2xl font-bold text-surface-800">"System Overview"</h2>
                <span class="text-sm text-surface-500">"Last updated: Just now"</span>
            </div>
            
            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
                <StatsCard title="Active Users" value="1,234" trend="+12%" color="text-green-600 bg-green-50" />
                <StatsCard title="Revenue" value="$12,400" trend="+5%" color="text-green-600 bg-green-50" />
                <StatsCard title="Errors" value="12" trend="-20%" color="text-red-600 bg-red-50" />
                <StatsCard title="Uptime" value="99.9%" trend="Stable" color="text-accent-600 bg-accent-50" />
            </div>

            <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
                <div class="lg:col-span-2 bg-white p-6 rounded-2xl border border-surface-200 shadow-sm">
                    <h3 class="text-lg font-semibold text-surface-800 mb-4">"Growth Analytics"</h3>
                    <div class="h-64 bg-surface-50 rounded-xl border-2 border-dashed border-surface-200 flex items-center justify-center text-surface-400 italic">
                        "Interactive Chart Placeholder"
                    </div>
                </div>
                <div class="bg-surface-900 p-6 rounded-2xl text-white shadow-xl">
                    <h3 class="text-lg font-semibold mb-4">"Quick Action"</h3>
                    <div class="text-center py-8">
                        <div class="text-5xl font-mono font-bold mb-6 text-accent-400">
                            {move || count.get()}
                        </div>
                        <div class="flex justify-center gap-3">
                            <button 
                                class="bg-surface-800 hover:bg-surface-700 w-12 h-12 rounded-xl transition-all active:scale-95"
                                on:click=move |_| set_count.update(|n| *n -= 1)
                            >
                                "-"
                            </button>
                            <button 
                                class="bg-accent-600 hover:bg-accent-500 px-6 py-3 rounded-xl font-bold transition-all active:scale-95"
                                on:click=move |_| set_count.update(|n| *n += 1)
                            >
                                "Add"
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
