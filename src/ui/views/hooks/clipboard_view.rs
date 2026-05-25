use leptos::prelude::*;
use crate::ui::shared::code_block::CodeBlock;
use crate::hooks::use_clipboard;

#[component]
pub fn ClipboardView() -> impl IntoView {
    let (copied, set_text) = use_clipboard();

    view! {
        <div class="p-6 lg:p-12 max-w-4xl mx-auto space-y-12">
            <div class="space-y-4">
                <h2 class="text-3xl font-extrabold text-slate-900 tracking-tight">"use_clipboard"</h2>
                <p class="text-lg text-slate-500 leading-relaxed">
                    "Handles copying text to the system clipboard with a built-in temporary 'copied' status."
                </p>
            </div>

            <div class="flex flex-col items-center justify-center py-12 space-y-6">
                <div class="relative flex items-center">
                    <input 
                        type="text" 
                        class="px-6 py-4 bg-slate-100 rounded-2xl border-2 border-slate-200 focus:border-indigo-500 outline-none transition-all text-lg w-80 text-center"
                        value="Hello from Leptos!"
                        on:input=move |ev| {
                            set_text.set(event_target_value(&ev));
                        }
                    />
                    <button 
                        on:click=move |_| set_text.set("Hello from Leptos!".to_string())
                        class=move || format!(
                            "absolute right-2 px-4 py-2 rounded-xl font-bold transition-all active:scale-95 text-sm {}",
                            if copied.get() { "bg-emerald-500 text-white" } else { "bg-indigo-600 text-white hover:bg-indigo-700" }
                        )
                    >
                        {move || if copied.get() { "Copied!" } else { "Copy" }}
                    </button>
                </div>
                <p class="text-slate-400 text-sm">"Change the text in the input and click Copy"</p>
            </div>

            <div class="bg-slate-900 rounded-3xl p-6 shadow-xl overflow-hidden">
                <div class="flex items-center justify-between mb-4">
                    <span class="text-xs font-mono text-slate-400 uppercase tracking-widest">"Usage"</span>
                </div>
                <CodeBlock code=r#"let (copied, set_text) = use_clipboard();

// Update the text to be copied
set_text.set("Your text here".to_string());

// 'copied' will be true for 2 seconds after set_text is called"# />
            </div>
        </div>
    }
}
