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
            <h2 class="text-2xl font-bold text-surface-800">"Detailed Analytics"</h2>
            <div class="bg-white rounded-2xl border border-surface-200 shadow-sm overflow-hidden">
                <table class="w-full text-left border-collapse">
                    <thead class="bg-surface-50 border-b border-surface-200">
                        <tr>
                            <th class="px-6 py-3 text-xs font-semibold text-surface-500 uppercase">"Region"</th>
                            <th class="px-6 py-3 text-xs font-semibold text-surface-500 uppercase">"Usage"</th>
                            <th class="px-6 py-3 text-xs font-semibold text-surface-500 uppercase">"Status"</th>
                        </tr>
                    </thead>
                    <tbody class="divide-y divide-surface-200">
                        {data.into_iter().map(|(region, usage, status)| {
                            view! {
                                <tr class="hover:bg-surface-50 transition-colors">
                                    <td class="px-6 py-4 text-sm text-surface-700 font-medium">{region}</td>
                                    <td class="px-6 py-4 text-sm text-surface-600">{usage}</td>
                                    <td class="px-6 py-4 text-sm">
                                        <span class="px-2 py-1 rounded-full text-xs font-medium bg-surface-100 text-surface-600">{status}</span>
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
