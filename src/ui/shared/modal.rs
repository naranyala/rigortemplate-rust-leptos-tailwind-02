use leptos::prelude::*;

#[component]
pub fn Modal(
    #[prop(into)] is_open: Signal<bool>, 
    #[prop(into)] on_close: Callback<()>, 
    children: Children
) -> impl IntoView {
    view! {
        <div class=move || format!("fixed inset-0 z-50 flex items-center justify-center p-4 sm:p-6 transition-opacity duration-200 {}", if is_open.get() { "opacity-100 visible" } else { "opacity-0 invisible pointer-events-none" })>
            <div 
                on:click=move |_| on_close.run(())
                class="absolute inset-0 bg-slate-900/50 backdrop-blur-sm" 
            />
            <div class="relative bg-white rounded-3xl shadow-2xl w-full max-w-lg overflow-hidden animate-in fade-in zoom-in duration-200">
                <div class="flex items-center justify-between p-6 border-b border-slate-100">
                    <h3 class="text-xl font-bold text-slate-800">"Modal Title"</h3>
                    <button 
                        on:click=move |_| on_close.run(())
                        class="p-2 text-slate-400 hover:text-slate-600 transition-colors"
                    >
                        "✕"
                    </button>
                </div>
                <div class="p-6">
                    {children()}
                </div>
            </div>
        </div>
    }
}
