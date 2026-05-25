use leptos::prelude::*;
use crate::ui::shared::progress::Progress;
use crate::ui::shared::button::Button;
use crate::ui::shared::demo_page::DemoPage;

#[component]
pub fn ProgressView() -> impl IntoView {
    let (val, set_val) = signal(30.0);

    let increment = move |_| set_val.update(|v| *v = f64::min(*v + 10.0, 100.0));
    let decrement = move |_| set_val.update(|v| *v = f64::max(*v - 10.0, 0.0));

    view! {
        <div class="min-h-screen p-4 lg:p-8 max-w-7xl mx-auto space-y-6">
            <DemoPage 
                title="Progress Component" 
                description="A simple, reactive progress bar for tracking completion or loading states."
                code=r#"view! {
    let (val, set_val) = signal(30.0);
    
    <div class="space-y-6 w-full max-w-md">
        <Progress value=val max=100.0 class=None />
        
        <div class="flex gap-4">
            <Button on:click=move |_| set_val.update(|v| *v = (*v - 10.0).max(0.0))>
                "Decrease"
            </Button>
            <Button on:click=move |_| set_val.update(|v| *v = (*v + 10.0).min(100.0))>
                "Increase"
            </Button>
        </div>
    </div>
}"#
            >
                <div class="space-y-6 w-full max-w-md">
                    <Progress value=val max=100.0 class=None />
                    
                    <div class="flex gap-4">
                        <Button on:click=decrement>"Decrease"</Button>
                        <Button on:click=increment>"Increase"</Button>
                    </div>
                </div>
            </DemoPage>
        </div>
    }
}
