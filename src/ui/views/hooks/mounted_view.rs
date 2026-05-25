use leptos::prelude::*;
use crate::ui::shared::DemoPage;
use crate::hooks::use_is_mounted;

#[component]
pub fn MountedView() -> impl IntoView {
    let is_mounted = use_is_mounted();

    view! {
        <DemoPage
            title="use_is_mounted"
            description="Tracks whether the component is currently mounted in the DOM. Useful for avoiding state updates after unmounting."
            code=r#"let is_mounted = use_is_mounted();

// Use in an effect to ensure the component is still present
Effect::new(move |_| {
    if is_mounted.get() {
        // Perform DOM operations safely
    }
});"#
        >
            <div class="flex items-center justify-center py-12">
                <div class="text-center space-y-4">
                    <div class=move || format!(
                        "text-6xl font-bold {} transition-colors duration-500",
                        if is_mounted.get() { "text-emerald-500" } else { "text-surface-300" }
                    )>
                        {move || if is_mounted.get() { "Mounted" } else { "Unmounting..." }}
                    </div>
                    <p class="text-surface-400 italic">"This value updates automatically once the component enters the DOM"</p>
                </div>
            </div>
        </DemoPage>
    }
}
