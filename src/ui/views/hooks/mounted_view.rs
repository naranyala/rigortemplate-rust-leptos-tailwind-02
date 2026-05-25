use leptos::prelude::*;
use crate::ui::shared::code_block::CodeBlock;
use crate::hooks::use_is_mounted;

#[component]
pub fn MountedView() -> impl IntoView {
    let is_mounted = use_is_mounted();

    view! {
        <div class="p-6 lg:p-12 max-w-4xl mx-auto space-y-12">
            <div class="space-y-4">
                <h2 class="text-3xl font-extrabold text-slate-900 tracking-tight">"use_is_mounted"</h2>
                <p class="text-lg text-slate-500 leading-relaxed">
                    "Tracks whether the component is currently mounted in the DOM. Useful for avoiding state updates after unmounting."
                </p>
            </div>

            <div class="flex items-center justify-center py-12">
                <div class="text-center space-y-4">
                    <div class=move || format!(
                        "text-6xl font-bold {} transition-colors duration-500",
                        if is_mounted.get() { "text-emerald-500" } else { "text-slate-300" }
                    )>
                        {move || if is_mounted.get() { "Mounted" } else { "Unmounting..." }}
                    </div>
                    <p class="text-slate-400 italic">"This value updates automatically once the component enters the DOM"</p>
                </div>
            </div>

            <div class="bg-slate-900 rounded-3xl p-6 shadow-xl overflow-hidden">
                <div class="flex items-center justify-between mb-4">
                    <span class="text-xs font-mono text-slate-400 uppercase tracking-widest">"Usage"</span>
                </div>
                <CodeBlock code=r#"let is_mounted = use_is_mounted();

// Use in an effect to ensure the component is still present
Effect::new(move |_| {
    if is_mounted.get() {
        // Perform DOM operations safely
    }
});"# />
            </div>
        </div>
    }
}
