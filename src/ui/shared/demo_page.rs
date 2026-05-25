use leptos::prelude::*;
use crate::ui::shared::code_block::CodeBlock;

#[component]
pub fn DemoPage(
    title: &'static str,
    description: &'static str,
    code: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="p-4 lg:p-8 max-w-7xl mx-auto space-y-6">
            <div class="space-y-2">
                <h2 class="text-2xl font-extrabold text-surface-900 tracking-tight">{title}</h2>
                <p class="text-base text-surface-500 leading-relaxed">{description}</p>
            </div>
            <div class="space-y-6">
                <div class="bg-surface-900 rounded-2xl p-4 border border-surface-800 shadow-sm overflow-hidden">
                    <div class="flex items-center justify-between mb-3">
                        <span class="text-xs font-mono text-surface-400 uppercase tracking-widest">"Preview"</span>
                    </div>
                    <div class="flex items-center justify-center py-6">
                        {children()}
                    </div>
                </div>
                <div class="bg-surface-900 rounded-2xl p-4 border border-surface-800 shadow-xl overflow-hidden">
                    <div class="flex items-center justify-between mb-3">
                        <span class="text-xs font-mono text-surface-400 uppercase tracking-widest">"Usage"</span>
                    </div>
                    <CodeBlock code=code />
                </div>
            </div>
        </div>
    }
}
