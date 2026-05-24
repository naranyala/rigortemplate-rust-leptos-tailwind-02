use leptos::prelude::*;
use crate::app::AppRoute;

#[component]
pub fn Sidebar(route: ReadSignal<AppRoute>) -> impl IntoView {
    let (search, set_search) = signal("".to_string());
    let set_route = use_context::<WriteSignal<AppRoute>>()
        .expect("set_route context not provided");
    
    let component_nav = vec![
        ("Gallery", AppRoute::Catalog),
        ("Accordion Demo", AppRoute::Accordion),
        ("Sliding Panel Demo", AppRoute::Panel),
    ];

    let hooks_nav = vec![
        ("Hooks Gallery", AppRoute::Hooks),
    ];

    view! {
        <aside class="hidden md:flex flex-col w-64 bg-slate-900 text-slate-300 h-screen sticky top-0 overflow-y-auto transition-all">
            <div class="p-6">
                <div class="flex items-center space-x-3 mb-6">
                    <div class="w-8 h-8 bg-indigo-600 rounded-lg flex items-center justify-center text-white font-bold">"L"</div>
                    <span class="text-xl font-bold text-white tracking-tight">"LeptosDash"</span>
                </div>
                
                <div class="relative group">
                    <span class="absolute inset-y-0 left-0 flex items-center pl-3 pointer-events-none text-slate-500 group-focus-within:text-indigo-400 transition-colors">
                        "🔍"
                    </span>
                    <input 
                        type="text" 
                        placeholder="Search menu..." 
                        class="w-full pl-9 pr-4 py-2 bg-slate-800 text-sm text-white rounded-xl border border-slate-700 focus:ring-2 focus:ring-indigo-500 focus:border-transparent outline-none transition-all"
                        on:input=move |ev| {
                            set_search.set(event_target_value(&ev));
                        }
                    />
                </div>
            </div>
            <nav class="flex-1 px-4 space-y-8">
                <div>
                    <p class="px-4 text-xs font-semibold text-slate-500 uppercase tracking-wider mb-2">"Components"</p>
                    <div class="space-y-1">
                        {move || {
                            let query = search.get().to_lowercase();
                            let current_route = route.get();
                            component_nav.iter()
                                .filter(|(name, _)| name.to_lowercase().contains(&query))
                                .map(|(name, route_val)| {
                                    let route_val = *route_val;
                                    view! {
                                        <button 
                                            on:click=move |_| set_route.set(route_val)
                                            class=move || format!(
                                                "w-full flex items-center px-4 py-3 text-sm font-medium rounded-xl hover:bg-slate-800 hover:text-white transition-all text-left {}",
                                                if current_route == route_val { "bg-indigo-600 text-white shadow-lg shadow-indigo-200" } else { "" }
                                            )
                                        >
                                            <span class="mr-3 opacity-40">"•"</span>
                                            {name.to_string()}
                                        </button>
                                    }
                                }).collect_view()
                        }}
                    </div>
                </div>
                <div>
                    <p class="px-4 text-xs font-semibold text-slate-500 uppercase tracking-wider mb-2">"Hooks"</p>
                    <div class="space-y-1">
                        {move || {
                            let query = search.get().to_lowercase();
                            let current_route = route.get();
                            hooks_nav.iter()
                                .filter(|(name, _)| name.to_lowercase().contains(&query))
                                .map(|(name, route_val)| {
                                    let route_val = *route_val;
                                    view! {
                                        <button 
                                            on:click=move |_| set_route.set(route_val)
                                            class=move || format!(
                                                "w-full flex items-center px-4 py-3 text-sm font-medium rounded-xl hover:bg-slate-800 hover:text-white transition-all text-left {}",
                                                if current_route == route_val { "bg-indigo-600 text-white shadow-lg shadow-indigo-200" } else { "" }
                                            )
                                        >
                                            <span class="mr-3 opacity-40">"•"</span>
                                            {name.to_string()}
                                        </button>
                                    }
                                }).collect_view()
                        }}
                    </div>
                </div>
            </nav>
            <div class="p-4 border-t border-slate-800">
                <div class="flex items-center p-3 bg-slate-800/50 rounded-2xl">
                    <div class="w-9 h-9 rounded-full bg-indigo-500 mr-3 border-2 border-slate-700"></div>
                    <div class="text-xs">
                        <p class="text-white font-semibold">"Admin User"</p>
                        <p class="text-slate-400">"Plan: Enterprise"</p>
                    </div>
                </div>
            </div>
        </aside>
    }
}
