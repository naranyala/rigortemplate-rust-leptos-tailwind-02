use leptos::prelude::*;
use crate::ui::shared::DemoPage;
use crate::hooks::use_toggle;

#[component]
pub fn ToggleView() -> impl IntoView {
    let (is_open, set_open) = use_toggle(false);

    view! {
        <DemoPage
            title="use_toggle"
            description="A simple hook to manage a boolean state toggle, ideal for modals, accordions, or sidebars."
            code=r#"let (is_open, set_open) = use_toggle(false);

// To toggle the state:
set_open.update(|v| *v = !*v);"#
        >
            <div class="flex items-center justify-center py-12">
                <button
                    on:click=move |_| set_open.update(|v| *v = !*v)
                    class="px-8 py-4 bg-accent-600 text-white rounded-2xl font-bold hover:bg-accent-700 transition-all hover:scale-105 active:scale-95"
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
        </DemoPage>
    }
}
