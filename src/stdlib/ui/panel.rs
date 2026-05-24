use leptos::prelude::*;

#[component]
pub fn SlidingPanel(
    #[prop(into)] is_open: Signal<bool>,
    #[prop(into)] on_close: Callback<()>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="fixed inset-0 z-50 pointer-events-none overflow-hidden">
            // Backdrop
            <div 
                class=move || format!(
                    "absolute inset-0 bg-slate-900/60 backdrop-blur-md transition-all duration-500 pointer-events-auto {}", 
                    if is_open.get() { "opacity-100 visible" } else { "opacity-0 invisible" }
                )
                on:click=move |_| on_close.run(())
            />
            
            // Panel
            <div 
                class=move || format!("absolute bottom-0 left-0 right-0 bg-white rounded-t-[2.5rem] shadow-[0_-10px_40px_-15px_rgba(0,0,0,0.3)] transition-transform duration-500 ease-out pointer-events-auto max-h-[92vh] flex flex-col {}", if is_open.get() { "translate-y-0" } else { "translate-y-full" })
            >

                // Handle / Grabber
                <div class="flex justify-center py-4 cursor-pointer" on:click=move |_| on_close.run(())>
                    <div class="w-12 h-1.5 bg-slate-200 rounded-full hover:bg-slate-300 transition-colors"></div>
                </div>
                
                <div class="px-8 pb-10 pt-2 overflow-y-auto">
                    {children()}
                </div>
            </div>
        </div>
    }
}
