use leptos::prelude::*;

#[component]
pub fn ProjectsView() -> impl IntoView {
    let projects = vec![
        ("Cloud Migration", "In Progress", "accent"),
        ("API v2 Rewrite", "Completed", "green"),
        ("Mobile App", "Planning", "amber"),
        ("Auth System", "Testing", "accent"),
    ];

    view! {
        <div class="p-6 space-y-6">
            <div class="flex items-center justify-between">
                <h2 class="text-2xl font-bold text-surface-800">"Project Portfolio"</h2>
                <button class="bg-accent-600 text-white px-4 py-2 rounded-lg text-sm font-medium hover:bg-accent-700 transition-colors">
                    "+ New Project"
                </button>
            </div>
            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
                {projects.into_iter().map(|(name, status, color)| {
                    view! {
                        <div class="bg-white p-5 rounded-2xl border border-surface-200 shadow-sm hover:shadow-md transition-all group">
                            <div class=format!("w-2 h-12 rounded-full bg-{}-500", color)></div>
                            <div class="ml-4">
                                <h3 class="font-bold text-surface-800 group-hover:text-accent-600 transition-colors">{name}</h3>
                                <p class="text-sm text-surface-500 mt-1">{status}</p>
                                <div class="mt-4 flex items-center justify-between">
                                    <div class="flex -space-x-2">
                                        <div class="w-6 h-6 rounded-full bg-surface-200 border-2 border-white"></div>
                                        <div class="w-6 h-6 rounded-full bg-surface-300 border-2 border-white"></div>
                                    </div>
                                    <span class="text-xs text-surface-400">"2 days ago"</span>
                                </div>
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}
