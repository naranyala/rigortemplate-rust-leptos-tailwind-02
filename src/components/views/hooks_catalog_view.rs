use leptos::prelude::*;
use crate::stdlib::ui::CodeBlock;

struct HookDemo {
    name: &'static str,
    description: &'static str,
    code: &'static str,
}

#[component]
pub fn HooksCatalogView() -> impl IntoView {
    let hooks = vec![
        HookDemo {
            name: "use_toggle",
            description: "A simple hook to manage a boolean state toggle.",
            code: r#"let (is_open, set_open) = use_toggle(false);
// Use set_open.update(|v| *v = !*v) to toggle"#,
        },
        HookDemo {
            name: "use_is_mounted",
            description: "Tracks whether the component is currently mounted in the DOM.",
            code: r#"let is_mounted = use_is_mounted();
// Useful for avoiding state updates after unmounting"#,
        },
        HookDemo {
            name: "use_clipboard",
            description: "Handles copying text to the clipboard with a 'copied' status.",
            code: r#"let (copied, set_text) = use_clipboard();
// set_text.set("Hello World".to_string());
// 'copied' becomes true for 2 seconds"#,
        },
    ];

    view! {
        <div class="p-6 lg:p-12 max-w-6xl mx-auto space-y-16">
            <div class="text-center space-y-4">
                <div class="inline-flex items-center px-3 py-1 rounded-full bg-emerald-50 text-emerald-600 text-xs font-bold uppercase tracking-wider mb-4">
                    "Utility Hooks"
                </div>
                <h2 class="text-4xl font-extrabold text-slate-900 tracking-tight">"Hooks Library"</h2>
                <p class="text-lg text-slate-500 max-w-2xl mx-auto leading-relaxed">
                    "A collection of reusable reactive primitives to handle common browser APIs and state patterns."
                </p>
            </div>
            
            <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
                {hooks.into_iter().map(|hook| {
                    view! {
                        <div class="bg-white rounded-3xl p-6 border border-slate-200 shadow-sm space-y-4">
                            <div class="flex items-center gap-3">
                                <div class="w-1.5 h-6 bg-emerald-500 rounded-full"></div>
                                <h3 class="text-lg font-bold text-slate-800">{hook.name}</h3>
                            </div>
                            <p class="text-sm text-slate-600 leading-relaxed">
                                {hook.description}
                            </p>
                            <div class="bg-slate-900 rounded-2xl p-4 shadow-inner overflow-hidden">
                                <div class="flex items-center justify-between mb-3">
                                    <span class="text-[10px] font-mono text-slate-500 uppercase tracking-widest">"Usage Example"</span>
                                </div>
                                <CodeBlock code=hook.code />
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}
