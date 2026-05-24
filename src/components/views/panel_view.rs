use leptos::prelude::*;
use crate::stdlib::ui::*;

#[component]
pub fn PanelView() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);

    view! {
        <div class="min-h-screen p-8 lg:p-16 max-w-3xl mx-auto space-y-12">
            <div class="space-y-4 text-center">
                <div class="inline-flex items-center px-3 py-1 rounded-full bg-indigo-50 text-indigo-600 text-xs font-bold uppercase tracking-wider mb-4">
                    "Mobile UX"
                </div>
                <h2 class="text-4xl font-extrabold text-slate-900 tracking-tight">"Sliding Bottom Sheet"</h2>
                <p class="text-lg text-slate-500 max-w-xl mx-auto leading-relaxed">
                    "Bring native mobile interaction patterns to the web with an elegant sliding-up panel."
                </p>
            </div>

                <div class="flex flex-col items-center justify-center py-20 space-y-8">
                    <div class="relative group">
                        <div class="absolute -inset-1 bg-gradient-to-r from-indigo-500 to-purple-600 rounded-2xl blur opacity-30 group-hover:opacity-50 transition-opacity"></div>
                        <button 
                            on:click=move |_| set_is_open.set(true)
                            class="relative px-8 py-4 bg-indigo-600 text-white rounded-2xl font-bold shadow-xl shadow-indigo-200 hover:bg-indigo-700 transition-all hover:scale-105 active:scale-95"
                        >
                            "Launch Experience"
                        </button>
                    </div>
                    <p class="text-slate-400 text-xs font-medium italic">"Click to trigger the bottom sheet"</p>
                </div>
                
                <div class="space-y-6">
                    <SlidingPanel 
                        is_open=is_open 
                        on_close=move || set_is_open.set(false)
                    >
                        <div class="space-y-8">
                            <div class="space-y-2">
                                <h3 class="text-2xl font-bold text-slate-900">"Interactive Detail View"</h3>
                                <p class="text-slate-500 leading-relaxed">
                                    "The bottom sheet is ideal for presenting context-aware information without 
                                    breaking the user's flow. It creates a focused workspace for secondary tasks."
                                </p>
                            </div>
                            
                            <div class="grid grid-cols-2 gap-4">
                                <div class="p-4 bg-slate-50 rounded-2xl border border-slate-100">
                                    <span class="block text-xs font-bold text-indigo-600 uppercase mb-1">"Speed"</span>
                                    <span class="text-sm text-slate-700 font-medium">"GPU Accelerated"</span>
                                </div>
                                <div class="p-4 bg-slate-50 rounded-2xl border border-slate-100">
                                    <span class="block text-xs font-bold text-indigo-600 uppercase mb-1">"Feel"</span>
                                    <span class="text-sm text-slate-700 font-medium">"Native-like"</span>
                                </div>
                            </div>
                            
                            <div class="p-6 bg-indigo-50 rounded-3xl border border-indigo-100 text-indigo-900 space-y-3">
                                <div class="flex items-center space-x-2">
                                    <span class="text-xl">"💡"</span>
                                    <span class="font-bold">"Pro Tip"</span>
                                </div>
                                <p class="text-sm opacity-80 leading-relaxed">
                                    "Combine this with gesture libraries to allow users to swipe the panel down to close it, 
                                    further enhancing the native feel."
                                </p>
                            </div>
                            
                            <button 
                                on:click=move |_| set_is_open.set(false)
                                class="w-full py-4 bg-slate-900 text-white rounded-2xl font-bold hover:bg-slate-800 transition-all shadow-lg"
                            >
                                "Got it, thanks!"
                            </button>
                        </div>
                    </SlidingPanel>
                    <div class="bg-slate-900 rounded-3xl p-6 shadow-xl overflow-hidden">
                        <div class="flex items-center justify-between mb-4">
                            <span class="text-xs font-mono text-slate-400 uppercase tracking-widest">"Source Code"</span>
                        </div>
                        <CodeBlock code=r#"view! {
    <SlidingPanel 
        is_open=is_open 
        on_close=move || set_is_open.set(false)
    >
        <div class="space-y-8">
            <div class="space-y-2">
                <h3 class="text-2xl font-bold text-slate-900">"Interactive Detail View"</h3>
                <p class="text-slate-500 leading-relaxed">
                    "The bottom sheet is ideal for presenting context-aware information without 
                    breaking the user's flow. It creates a focused workspace for secondary tasks."
                </p>
            </div>
            
            <div class="grid grid-cols-2 gap-4">
                <div class="p-4 bg-slate-50 rounded-2xl border border-slate-100">
                    <span class="block text-xs font-bold text-indigo-600 uppercase mb-1">"Speed"</span>
                    <span class="text-sm text-slate-700 font-medium">"GPU Accelerated"</span>
                </div>
                <div class="p-4 bg-slate-50 rounded-2xl border border-slate-100">
                    <span class="block text-xs font-bold text-indigo-600 uppercase mb-1">"Feel"</span>
                    <span class="text-sm text-slate-700 font-medium">"Native-like"</span>
                </div>
            </div>
            
            <div class="p-6 bg-indigo-50 rounded-3xl border border-indigo-100 text-indigo-900 space-y-3">
                <div class="flex items-center space-x-2">
                    <span class="text-xl">"💡"</span>
                    <span class="font-bold">"Pro Tip"</span>
                </div>
                <p class="text-sm opacity-80 leading-relaxed">
                    "Combine this with gesture libraries to allow users to swipe the panel down to close it, 
                    further enhancing the native feel."
                </p>
            </div>
            
            <button 
                on:click=move |_| set_is_open.set(false)
                class="w-full py-4 bg-slate-900 text-white rounded-2xl font-bold hover:bg-slate-800 transition-all shadow-lg"
            >
                "Got it, thanks!"
            </button>
        </div>
    </SlidingPanel>
}"# />
                    </div>
                </div>
        </div>
    }
}
