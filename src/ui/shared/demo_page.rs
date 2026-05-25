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
        <div class="p-6 lg:p-12 max-w-4xl mx-auto space-y-12">
            <div class="space-y-4">
                <h2 class="text-3xl font-extrabold text-surface-900 tracking-tight">{title}</h2>
                <p class="text-lg text-surface-500 leading-relaxed">{description}</p>
            </div>
            {children()}
            <div class="bg-surface-900 rounded-3xl p-6 shadow-xl overflow-hidden">
                <div class="flex items-center justify-between mb-4">
                    <span class="text-xs font-mono text-surface-400 uppercase tracking-widest">"Usage"</span>
                </div>
                <CodeBlock code=code />
            </div>
        </div>
    }
}
