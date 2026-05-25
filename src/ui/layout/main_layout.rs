use leptos::prelude::*;
use leptos_router::hooks::use_location;
use crate::ui::layout::{Header, Sidebar};

use crate::ui::shared::toasts::use_toasts;
use crate::ui::shared::toast::Toast;

#[component]
pub fn MainLayout(children: Children) -> impl IntoView {
    let toast_ctx = use_toasts();
    let (is_sidebar_open, set_is_sidebar_open) = signal(false);
    
    let location = use_location();

    Effect::new(move |_| {
        let _ = location.pathname.get(); // Depend on pathname changes
        if let Some(window) = web_sys::window() {
            window.scroll_to_with_x_and_y(0.0, 0.0);
        }
    });

    view! {
        <div class="flex h-screen overflow-hidden bg-surface-50 dark:bg-surface-950 text-surface-900 dark:text-surface-100 font-sans transition-colors duration-300">
            <Sidebar is_open=is_sidebar_open set_is_open=set_is_sidebar_open />
            
            // Mobile Backdrop
            <div 
                class=move || format!(
                    "fixed inset-0 bg-black/50 z-40 transition-opacity duration-300 md:hidden {}",
                    if is_sidebar_open.get() { "opacity-100 pointer-events-auto" } else { "opacity-0 pointer-events-none" }
                )
                on:click=move |_| set_is_sidebar_open.set(false)
            />

            <div class="flex-1 flex flex-col overflow-hidden relative">
                <Header set_is_open=set_is_sidebar_open />
                <main class="flex-1 overflow-y-auto">
                    {children()}
                </main>
                <div class="fixed bottom-6 right-6 z-[100] flex flex-col gap-3 pointer-events-none">
                    <For
                        each=move || toast_ctx.toasts.get()
                        key=|t| t.id
                        children=move |toast| {
                            view! {
                                <div class="pointer-events-auto animate-in slide-in-from-right duration-300">
                                    <Toast
                                        message=toast.message.clone()
                                        toast_type=toast.toast_type.clone()
                                    />
                                </div>
                            }
                        }
                    />
                </div>
            </div>
        </div>
    }
}
