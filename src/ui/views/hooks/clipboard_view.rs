use leptos::prelude::*;
use crate::ui::shared::DemoPage;
use crate::hooks::use_clipboard;

#[component]
pub fn ClipboardView() -> impl IntoView {
    let (copied, set_text) = use_clipboard();

    view! {
        <DemoPage
            title="use_clipboard"
            description="Handles copying text to the system clipboard with a built-in temporary 'copied' status."
            code=r#"let (copied, set_text) = use_clipboard();

// Update the text to be copied
set_text.set("Your text here".to_string());

// 'copied' will be true for 2 seconds after set_text is called"#
        >
            <div class="flex flex-col items-center justify-center py-12 space-y-6">
                <div class="relative flex items-center">
                    <input 
                        type="text" 
                        class="px-6 py-4 bg-surface-100 rounded-2xl border-2 border-surface-200 focus:border-accent-500 outline-none transition-all text-lg w-80 text-center"
                        value="Hello from Leptos!"
                        on:input=move |ev| {
                            set_text.set(event_target_value(&ev));
                        }
                    />
                    <button 
                        on:click=move |_| set_text.set("Hello from Leptos!".to_string())
                        class=move || format!(
                            "absolute right-2 px-4 py-2 rounded-xl font-bold transition-all active:scale-95 text-sm {}",
                            if copied.get() { "bg-emerald-500 text-white" } else { "bg-accent-600 text-white hover:bg-accent-700" }
                        )
                    >
                        {move || if copied.get() { "Copied!" } else { "Copy" }}
                    </button>
                </div>
                <p class="text-surface-400 text-sm">"Change the text in the input and click Copy"</p>
            </div>
        </DemoPage>
    }
}
