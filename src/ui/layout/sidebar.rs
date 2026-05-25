use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

#[component]
pub fn Sidebar() -> impl IntoView {
    let (search, set_search) = signal("".to_string());
    let navigate = use_navigate();
    let nav_components = navigate.clone();

    let component_nav = vec![
        ("Accordion Demo", "/accordion"),
        ("Sliding Panel Demo", "/panel"),
    ];

    let hooks_nav = vec![
        ("use_toggle", "/toggle"),
        ("use_is_mounted", "/mounted"),
        ("use_clipboard", "/clipboard"),
    ];

    let navigate_home = navigate.clone();
    let go_home = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        navigate_home("/", Default::default());
    };

    view! {
        <aside class="hidden md:flex flex-col w-64 bg-white dark:bg-surface-900 text-surface-600 dark:text-surface-300 h-screen sticky top-0 overflow-y-auto border-r border-surface-200 dark:border-surface-800 transition-colors duration-300">
            <div class="p-6">
                <a
                    href="/"
                    on:click=go_home
                    class="flex items-center space-x-3 mb-6"
                >
                    <div class="w-8 h-8 bg-accent-600 rounded-lg flex items-center justify-center text-white font-bold">"L"</div>
                    <span class="text-xl font-bold text-surface-900 dark:text-white tracking-tight">"LeptosDash"</span>
                </a>

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
                <div>
                    <p class="px-4 text-xs font-semibold text-surface-400 dark:text-surface-500 uppercase tracking-wider mb-2">"Components"</p>
                    <div class="space-y-1">
                        {move || {
                            let query = search.get().to_lowercase();
                            let nav = nav_components.clone();
                            component_nav.iter()
                                .filter(|(name, _)| name.to_lowercase().contains(&query))
                                .map(move |(name, href)| {
                                    let nav = nav.clone();
                                    let href = *href;
                                    view! {
                                        <a
                                            href=href
                                            on:click=move |ev| {
                                                ev.prevent_default();
                                                nav(href, Default::default());
                                            }
                                            class="flex items-center px-4 py-3 text-sm font-medium rounded-xl hover:bg-surface-100 dark:hover:bg-surface-800 hover:text-accent-600 dark:hover:text-white transition-all text-left"
                                        >
                                            <span class="mr-3 opacity-40">"•"</span>
                                            {name.to_string()}
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
                            let nav = navigate.clone();
                            hooks_nav.iter()
                                .filter(|(name, _)| name.to_lowercase().contains(&query))
                                .map(move |(name, href)| {
                                    let nav = nav.clone();
                                    let href = *href;
                                    view! {
                                        <a
                                            href=href
                                            on:click=move |ev| {
                                                ev.prevent_default();
                                                nav(href, Default::default());
                                            }
                                            class="flex items-center px-4 py-3 text-sm font-medium rounded-xl hover:bg-surface-100 dark:hover:bg-surface-800 hover:text-accent-600 dark:hover:text-white transition-all text-left"
                                        >
                                            <span class="mr-3 opacity-40">"•"</span>
                                            {name.to_string()}
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
