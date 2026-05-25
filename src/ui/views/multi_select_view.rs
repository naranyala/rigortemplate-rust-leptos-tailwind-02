use leptos::prelude::*;
use crate::ui::shared::multi_select::MultiSelect;
use crate::ui::shared::demo_page::DemoPage;

#[component]
pub fn MultiSelectView() -> impl IntoView {
    let selected = RwSignal::new(vec!["rust".to_string(), "leptos".to_string()]);
    let options = vec![
        ("rust".to_string(), "Rust".to_string()),
        ("leptos".to_string(), "Leptos".to_string()),
        ("tailwind".to_string(), "Tailwind CSS".to_string()),
        ("wasm".to_string(), "WebAssembly".to_string()),
        ("web-sys".to_string(), "web-sys".to_string()),
    ];

    view! {
        <div class="min-h-screen p-4 lg:p-8 max-w-7xl mx-auto space-y-6">
            <DemoPage 
                title="Multi Select Component" 
                description="A flexible dropdown for selecting multiple options from a curated list."
                code=r#"view! {
    let selected = RwSignal::new(vec!["rust".to_string()]);
    let options = vec![
        ("rust".to_string(), "Rust".to_string()),
        ("tailwind".to_string(), "Tailwind CSS".to_string()),
    ];
    <MultiSelect 
        options=options 
        selected=selected 
        placeholder="Select technologies..." 
    />
}"#
            >
                <div class="max-w-sm">
                    <MultiSelect 
                        options=options 
                        selected=selected 
                        placeholder="Select technologies..." 
                    />
                </div>
            </DemoPage>
        </div>
    }
}
