use leptos::prelude::*;
use crate::ui::shared::code_block::CodeBlock;
use crate::hooks::use_toggle;

#[component]
pub fn ToggleView() -> impl IntoView {
    let (is_open, set_open) = use_toggle(false);

    view! {
        <div class="p-6 lg:p-12 max-w-4xl mx-auto space-y-12">
            <div class="space-y-4">
                <h2 class="text-3xl font-extrabold text-slate-900 tracking-tight">"use_toggle"</h2>
                <p class="text-lg text-slate-500 leading-relaxed">
                    "A simple hook to manage a boolean state toggle, ideal for modals, accordions, or sidebars."
                </p>
            </div>

            <div class="flex items-center justify-center py-12">
                <button 
                    on:click=move |_| set_open.update(|v| *v = !*v)
                    class="px-8 py-4 bg-indigo-600 text-white rounded-2xl font-bold shadow-xl shadow-indigo-200 hover:bg-indigo-700 transition-all hover:scale-105 active:scale-95"
                >
                    {move || if is_open.get() { "Hide Content" } else { "Show Content" }}
                </button>
            </div>

            <div class="transition-all duration-300 overflow-hidden">
                {move || if is_open.get() {
                    view! {
                        <div class="p-8 bg-emerald-50 border border-emerald-100 rounded-3xl text-emerald-800 font-medium text-center animate-in fade-in slide-in-from-top-4">
                            "✨ This content is toggled by the use_toggle hook!"
                        </div>
                    }.into_any()
                } else {
                    view! { <div class="h-0"></div> }.into_any()
                }}
            </div>

            <div class="bg-slate-900 rounded-3xl p-6 shadow-xl overflow-hidden">
                <div class="flex items-center justify-between mb-4">
                    <span class="text-xs font-mono text-slate-400 uppercase tracking-widest">"Usage"</span>
                </div>
                <CodeBlock code=r#"let (is_open, set_open) = use_toggle(false);

// To toggle the state:
set_open.update(|v| *v = !*v);"# />
            </div>
        </div>
    }
}
