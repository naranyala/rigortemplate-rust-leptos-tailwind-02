use leptos::prelude::*;
use crate::ui::shared::panel::SlidingPanel;
use crate::ui::shared::demo_page::DemoPage;

#[component]
pub fn PanelView() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);

    view! {
        <div class="min-h-screen p-4 lg:p-8 max-w-7xl mx-auto space-y-6">
            <DemoPage 
                title="Sliding Panel" 
                description="An elegant bottom-sheet component for focused, context-aware interactions."
                code=r#"view! {
    <SlidingPanel 
        is_open=is_open 
        on_close=move || set_is_open.set(false)
    >
        <div class="space-y-8">
            <div class="space-y-2">
                <h3 class="text-2xl font-bold text-surface-900">"Interactive Detail View"</h3>
                <p class="text-surface-500 leading-relaxed">
                    "The bottom sheet is ideal for presenting context-aware information without 
                    breaking the user's flow. It creates a focused workspace for secondary tasks."
                </p>
            </div>
            
            <div class="grid grid-cols-2 gap-4">
                <div class="p-4 bg-surface-50 rounded-2xl border border-surface-100">
                    <span class="block text-xs font-bold text-accent-600 uppercase mb-1">"Speed"</span>
                    <span class="text-sm text-surface-700 font-medium">"GPU Accelerated"</span>
                </div>
                <div class="p-4 bg-surface-50 rounded-2xl border border-surface-100">
                    <span class="block text-xs font-bold text-accent-600 uppercase mb-1">"Feel"</span>
                    <span class="text-sm text-surface-700 font-medium">"Native-like"</span>
                </div>
            </div>
            
            <div class="p-6 bg-accent-50 rounded-3xl border border-accent-100 text-accent-900 space-y-3">
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
                class="w-full py-4 bg-surface-900 text-white rounded-2xl font-bold hover:bg-surface-800 transition-all shadow-lg"
            >
                "Got it, thanks!"
            </button>
        </div>
    </SlidingPanel>
}"#
            >
                <div class="flex flex-col items-center justify-center space-y-8 py-12">
                    <div class="relative group">
                        <div class="absolute -inset-1 bg-gradient-to-r from-accent-500 to-purple-600 rounded-2xl blur opacity-30 group-hover:opacity-50 transition-opacity"></div>
                        <button 
                            on:click=move |_| set_is_open.set(true)
                            class="relative px-8 py-4 bg-accent-600 text-white rounded-2xl font-bold hover:bg-accent-700 transition-all hover:scale-105 active:scale-95"
                        >
                            "Launch Experience"
                        </button>
                    </div>
                    <p class="text-surface-400 text-xs font-medium italic">"Click to trigger the bottom sheet"</p>
                    
                    <SlidingPanel 
                        is_open=is_open 
                        on_close=move || set_is_open.set(false)
                    >
                        <div class="space-y-8">
                            <div class="space-y-2">
                                <h3 class="text-2xl font-bold text-surface-900">"Interactive Detail View"</h3>
                                <p class="text-surface-500 leading-relaxed">
                                    "The bottom sheet is ideal for presenting context-aware information without 
                                    breaking the user's flow. It creates a focused workspace for secondary tasks."
                                </p>
                            </div>
                            
                            <div class="grid grid-cols-2 gap-4">
                                <div class="p-4 bg-surface-50 rounded-2xl border border-surface-100">
                                    <span class="block text-xs font-bold text-accent-600 uppercase mb-1">"Speed"</span>
                                    <span class="text-sm text-surface-700 font-medium">"GPU Accelerated"</span>
                                </div>
                                <div class="p-4 bg-surface-50 rounded-2xl border border-surface-100">
                                    <span class="block text-xs font-bold text-accent-600 uppercase mb-1">"Feel"</span>
                                    <span class="text-sm text-surface-700 font-medium">"Native-like"</span>
                                </div>
                            </div>
                            
                            <div class="p-6 bg-accent-50 rounded-3xl border border-accent-100 text-accent-900 space-y-3">
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
                                class="w-full py-4 bg-surface-900 text-white rounded-2xl font-bold hover:bg-surface-800 transition-all shadow-lg"
                            >
                                "Got it, thanks!"
                            </button>
                        </div>
                    </SlidingPanel>
                </div>
            </DemoPage>
        </div>
    }
}
