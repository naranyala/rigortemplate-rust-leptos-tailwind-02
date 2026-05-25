use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use crate::ui::registry::{ALL_DEMOS, DemoCategory};

#[component]
pub fn Sidebar(is_open: ReadSignal<bool>, set_is_open: WriteSignal<bool>) -> impl IntoView {
    let (search, set_search) = signal("".to_string());
    let navigate = use_navigate();
    let nav_handle = navigate.clone();
    let nav_handle_comp = nav_handle.clone();
    let nav_handle_hooks = nav_handle.clone();

    let navigate_home = navigate.clone();
    let go_home = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        navigate_home("/", Default::default());
    };

    let nav_handle_comp = nav_handle.clone();
    let nav_handle_hooks = nav_handle.clone();

    view! {
        <aside class=move || format!(
            "fixed inset-y-0 left-0 z-50 w-64 bg-white dark:bg-surface-900 text-surface-600 dark:text-surface-300 h-screen overflow-y-auto border-r border-surface-200 dark:border-surface-800 transition-transform duration-300 flex flex-col md:relative md:translate-x-0 {}",
            if is_open.get() { "translate-x-0" } else { "-translate-x-full md:translate-x-0" }
        )>
            <div class="p-6">
                <div class="flex items-center justify-between mb-6">
                    <a
                        href="/"
                        on:click=go_home
                        class="flex items-center space-x-3"
                    >
                        <div class="w-8 h-8 bg-accent-600 rounded-lg flex items-center justify-center text-white font-bold">"L"</div>
                        <span class="text-xl font-bold text-surface-900 dark:text-white tracking-tight">"LeptosDash"</span>
                    </a>
                    <button 
                        on:click=move |_| set_is_open.set(false)
                        class="md:hidden p-2 text-surface-400 hover:text-surface-600 dark:hover:text-surface-200"
                    >
                        "✕"
                    </button>
                </div>

                <div class="relative group">
                    <span class="absolute inset-y-0 left-0 flex items-center pl-3 pointer-events-none text-surface-400 group-focus-within:text-accent-500 transition-colors">
                        "🔍"
                    </span>
                    <input
                        type="text"
                        placeholder="Search menu..."
                        class="w-full pl-9 pr-4 py-2 bg-surface-100 dark:bg-surface-800 text-sm text-surface-900 dark:text-white rounded-xl border border-surface-200 dark:border-surface-700 focus:ring-2 focus:ring-accent-500 focus:border-transparent outline-none transition-all"
                        on:input=move |ev| {
                            set_search.set(event_target_value(&ev));
                        }
                    />
                </div>
            </div>
            <nav class="flex-1 px-4 space-y-8">
                {ALL_DEMOS.iter().enumerate().map(|(i, _)| {
                    // This is a hack to force reactivity if the list was dynamic, 
                    // but since it's const, we just need to ensure we render the categories.
                    // We only want to render the category headers once.
                    if i == 0 || ALL_DEMOS[i-1].category != ALL_DEMOS[i].category {
                        // we handle headers separately below
                    }
                    view! { <div /> }
                }).collect_view()}

                // Better approach: render categories explicitly
                <div>
                    <p class="px-4 text-xs font-semibold text-surface-400 dark:text-surface-500 uppercase tracking-wider mb-2">"Components"</p>
                    <div class="space-y-1">
                        {move || {
                            let query = search.get().to_lowercase();
                            let nav = nav_handle_comp.clone();
                            ALL_DEMOS.iter()
                                .filter(|d| matches!(d.category, DemoCategory::Component) && d.name.to_lowercase().contains(&query))
                                .map(move |d| {
                                    let nav = nav.clone();
                                    let path = d.path;
                                    view! {
                                        <a
                                            href=path
                                            on:click=move |ev| {
                                                ev.prevent_default();
                                                nav(path, Default::default());
                                            }
                                            class="flex items-center px-4 py-3 text-sm font-medium rounded-xl hover:bg-surface-100 dark:hover:bg-surface-800 hover:text-accent-600 dark:hover:text-white transition-all text-left"
                                        >
                                            <span class="mr-3 opacity-40">"•"</span>
                                            {d.name}
                                        </a>
                                    }
                                })
                                .collect_view()
                        }}
                    </div>
                </div>
                <div>
                    <p class="px-4 text-xs font-semibold text-surface-400 dark:text-surface-500 uppercase tracking-wider mb-2">"Hooks"</p>
                    <div class="space-y-1">
                        {move || {
                            let query = search.get().to_lowercase();
                            let nav = nav_handle_hooks.clone();
                            ALL_DEMOS.iter()
                                .filter(|d| matches!(d.category, DemoCategory::Hook) && d.name.to_lowercase().contains(&query))
                                .map(move |d| {
                                    let nav = nav.clone();
                                    let path = d.path;
                                    view! {
                                        <a
                                            href=path
                                            on:click=move |ev| {
                                                ev.prevent_default();
                                                nav(path, Default::default());
                                            }
                                            class="flex items-center px-4 py-3 text-sm font-medium rounded-xl hover:bg-surface-100 dark:hover:bg-surface-800 hover:text-accent-600 dark:hover:text-white transition-all text-left"
                                        >
                                            <span class="mr-3 opacity-40">"•"</span>
                                            {d.name}
                                        </a>
                                    }
                                })
                                .collect_view()
                        }}
                    </div>
                </div>
            </nav>
        </aside>
    }
}
