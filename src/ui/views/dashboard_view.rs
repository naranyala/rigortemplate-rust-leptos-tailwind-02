use leptos::prelude::*;
use crate::ui::shared::StatsCard;

#[component]
pub fn DashboardView() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <div class="p-6 space-y-6">
            <div class="flex items-center justify-between">
                <h2 class="text-2xl font-bold text-slate-800">"System Overview"</h2>
                <span class="text-sm text-slate-500">"Last updated: Just now"</span>
            </div>
            
            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
                <StatsCard title="Active Users" value="1,234" trend="+12%" color="text-green-600 bg-green-50" />
                <StatsCard title="Revenue" value="$12,400" trend="+5%" color="text-green-600 bg-green-50" />
                <StatsCard title="Errors" value="12" trend="-20%" color="text-red-600 bg-red-50" />
                <StatsCard title="Uptime" value="99.9%" trend="Stable" color="text-blue-600 bg-blue-50" />
            </div>

            <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
                <div class="lg:col-span-2 bg-white p-6 rounded-2xl border border-slate-200 shadow-sm">
                    <h3 class="text-lg font-semibold text-slate-800 mb-4">"Growth Analytics"</h3>
                    <div class="h-64 bg-slate-50 rounded-xl border-2 border-dashed border-slate-200 flex items-center justify-center text-slate-400 italic">
                        "Interactive Chart Placeholder"
                    </div>
                </div>
                <div class="bg-slate-900 p-6 rounded-2xl text-white shadow-xl">
                    <h3 class="text-lg font-semibold mb-4">"Quick Action"</h3>
                    <div class="text-center py-8">
                        <div class="text-5xl font-mono font-bold mb-6 text-indigo-400">
                            {move || count.get()}
                        </div>
                        <div class="flex justify-center gap-3">
                            <button 
                                class="bg-slate-800 hover:bg-slate-700 w-12 h-12 rounded-xl transition-all active:scale-95"
                                on:click=move |_| set_count.update(|n| *n -= 1)
                            >
                                "-"
                            </button>
                            <button 
                                class="bg-indigo-600 hover:bg-indigo-500 px-6 py-3 rounded-xl font-bold transition-all active:scale-95"
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
