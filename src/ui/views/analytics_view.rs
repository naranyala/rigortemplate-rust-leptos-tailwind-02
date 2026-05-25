use leptos::prelude::*;

#[component]
pub fn AnalyticsView() -> impl IntoView {
    let data = vec![
        ("Region A", "45%", "Active"),
        ("Region B", "32%", "Warning"),
        ("Region C", "12%", "Critical"),
        ("Region D", "11%", "Active"),
    ];

    view! {
        <div class="p-6 space-y-6">
            <h2 class="text-2xl font-bold text-slate-800">"Detailed Analytics"</h2>
            <div class="bg-white rounded-2xl border border-slate-200 shadow-sm overflow-hidden">
                <table class="w-full text-left border-collapse">
                    <thead class="bg-slate-50 border-b border-slate-200">
                        <tr>
                            <th class="px-6 py-3 text-xs font-semibold text-slate-500 uppercase">"Region"</th>
                            <th class="px-6 py-3 text-xs font-semibold text-slate-500 uppercase">"Usage"</th>
                            <th class="px-6 py-3 text-xs font-semibold text-slate-500 uppercase">"Status"</th>
                        </tr>
                    </thead>
                    <tbody class="divide-y divide-slate-200">
                        {data.into_iter().map(|(region, usage, status)| {
                            view! {
                                <tr class="hover:bg-slate-50 transition-colors">
                                    <td class="px-6 py-4 text-sm text-slate-700 font-medium">{region}</td>
                                    <td class="px-6 py-4 text-sm text-slate-600">{usage}</td>
                                    <td class="px-6 py-4 text-sm">
                                        <span class="px-2 py-1 rounded-full text-xs font-medium bg-slate-100 text-slate-600">{status}</span>
                                    </td>
                                </tr>
                            }
                        }).collect_view()}
                    </tbody>
                </table>
            </div>
        </div>
    }
}
