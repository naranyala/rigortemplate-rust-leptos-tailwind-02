use leptos::prelude::{ElementChild, ClassAttribute};
use leptos::prelude::*;
use crate::stdlib::ui::*;
use std::sync::Arc;

#[derive(Clone)]
struct ComponentDemo {
    name: &'static str,
    full_source: &'static str,
    view: Arc<dyn Fn() -> AnyView + Send + Sync>,
}

#[component]
pub fn CatalogView() -> impl IntoView {
    let components = get_ui_components();
    
    let demos = vec![
        ComponentDemo {
            name: "Button",
            full_source: components[0].full_source,
            view: Arc::new(|| view! { <Button variant="primary">"Click Me"</Button> }.into_any()),
        },
        ComponentDemo {
            name: "Badge",
            full_source: components[1].full_source,
            view: Arc::new(|| view! { <Badge text="Active" color="green" /> }.into_any()),
        },
        ComponentDemo {
            name: "Card",
            full_source: components[2].full_source,
            view: Arc::new(|| view! { <Card title="Hello">"This is a card content area."</Card> }.into_any()),
        },
        ComponentDemo {
            name: "Input",
            full_source: components[3].full_source,
            view: Arc::new(|| {
                let (val, set_val) = signal("".to_string());
                view! { <Input label="Username" placeholder="Enter name..." value=val set_value=set_val error=Some("Required".to_string()) /> }.into_any()
            }),
        },
        ComponentDemo {
            name: "Modal",
            full_source: components[4].full_source,
            view: Arc::new(|| {
                let (open, set_open) = signal(true);
                view! { <Modal is_open=open on_close=move || set_open.set(false)>"Modal Content"</Modal> }.into_any()
            }),
        },
        ComponentDemo {
            name: "Toast",
            full_source: components[5].full_source,
            view: Arc::new(|| view! { <Toast message="Action successful!" toast_type="success" /> }.into_any()),
        },
    ];

    view! {
        <div class="p-6 lg:p-12 max-w-6xl mx-auto space-y-16">
            <div class="text-center space-y-4">
                <div class="inline-flex items-center px-3 py-1 rounded-full bg-indigo-50 text-indigo-600 text-xs font-bold uppercase tracking-wider mb-4">
                    "UI Library"
                </div>
                <h2 class="text-4xl font-extrabold text-slate-900 tracking-tight">"Component Catalog"</h2>
                <p class="text-lg text-slate-500 max-w-2xl mx-auto leading-relaxed">
                    "A collection of high-quality, reusable UI components built with Leptos and Tailwind CSS for fast and accessible web interfaces."
                </p>
            </div>
            
            <div class="space-y-24">
                {demos.into_iter().map(|demo| {
                    view! {
                        <div class="space-y-6">
                            <div class="flex items-center gap-3">
                                <div class="w-1.5 h-8 bg-indigo-600 rounded-full"></div>
                                <h3 class="text-2xl font-bold text-slate-800">{demo.name}</h3>
                            </div>
                            <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
                                <div class="bg-white p-8 rounded-3xl border border-slate-200 shadow-sm flex items-center justify-center min-h-[240px]">
                                    {(demo.view)() }
                                </div>
                                <div class="bg-slate-900 rounded-3xl p-6 shadow-xl overflow-hidden">
                                    <div class="flex items-center justify-between mb-4">
                                        <span class="text-xs font-mono text-slate-400 uppercase tracking-widest">"Component Source"</span>
                                    </div>
                                    <CodeBlock code=demo.full_source />
                                </div>
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}
