use leptos::prelude::*;
use crate::hooks::use_interval;
use crate::ui::shared::demo_page::DemoPage;
use crate::ui::shared::button::Button;

#[component]
pub fn IntervalView() -> impl IntoView {
    let (count, set_count) = signal(0);
    let (is_running, set_running) = use_interval(1000, move || {
        set_count.update(|c| *c += 1);
    });

    view! {
        <div class="min-h-screen p-4 lg:p-8 max-w-7xl mx-auto space-y-6">
            <DemoPage 
                title="use_interval Hook" 
                description="Executes a callback function periodically at a specified interval."
                code=r#"view! {
    let (count, set_count) = signal(0);
    let (is_running, set_running) = use_interval(1000, move || {
        set_count.update(|c| *c += 1);
    });

    <div class="flex flex-col items-center gap-6">
        <div class="text-6xl font-bold font-mono">{move || count.get()}</div>
        <Button on:click=move |_| set_running.set(!is_running.get())>
            "Toggle Timer"
        </Button>
    </div>
}"#
            >
                <div class="flex flex-col items-center gap-6">
                    <div class="text-6xl font-bold font-mono text-accent-600">{move || count.get()}</div>
                    <Button on:click=move |_| set_running.set(!is_running.get())>
                        "Toggle Timer"
                    </Button>
                </div>
            </DemoPage>
        </div>
    }
}
