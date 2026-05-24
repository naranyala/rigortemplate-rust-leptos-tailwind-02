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
            name: "use_window_size",
            description: "Tracks the window dimensions reactively.",
            code: r#"let (width, height) = use_window_size();
// width and height are ReadSignal<u32>"#,
        },
        HookDemo {
            name: "use_previous",
            description: "Remembers the previous value of a signal.",
            code: r#"let (count, set_count) = signal(0);
let prev_count = use_previous(count);
// prev_count will be one step behind count"#,
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
        HookDemo {
            name: "use_media_query",
            description: "Reactively tracks a CSS media query.",
            code: r#"let is_mobile = use_media_query("(max-width: 768px)");
// is_mobile is a ReadSignal<bool>"#,
        },
        HookDemo {
            name: "use_debounce",
            description: "Debounces a signal's value to prevent rapid updates.",
            code: r#"let (search, set_search) = signal("".to_string());
let debounced_search = use_debounce(search, 300);
// debounced_search updates 300ms after search stops changing"#,
        },
        HookDemo {
            name: "use_interval",
            description: "A reactive interval timer that can be enabled or disabled.",
            code: r#"let (active, set_active) = signal(true);
use_interval(|| { println!("Tick!"); }, 1000, active);"#,
        },
        HookDemo {
            name: "use_session_storage",
            description: "Syncs state with the browser's Session Storage.",
            code: r#"let (user, set_user) = use_session_storage("user_pref", "Guest".to_string());
//Persists for the duration of the page session"#,
        },
        HookDemo {
            name: "use_click_outside",
            description: "Triggers a callback when a click occurs outside the target element.",
            code: r#"let my_node = create_node_ref::<html::Div>();
use_click_outside(my_node, || { println!("Clicked outside!"); });"#,
        },
        HookDemo {
            name: "use_element_size",
            description: "Tracks the dimensions of a specific DOM element via ResizeObserver.",
            code: r#"let my_node = create_node_ref::<html::Div>();
let (width, height) = use_element_size(my_node);"#,
        },
        HookDemo {
            name: "use_intersection_observer",
            description: "Detects when an element enters or leaves the viewport.",
            code: r#"let my_node = create_node_ref::<html::Div>();
let is_visible = use_intersection_observer(my_node, IntersectionObserverInit::default());"#,
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
