use leptos::prelude::*;
use crate::ui::layout::{Header, Sidebar};

use crate::ui::shared::toasts::use_toasts;
use crate::ui::shared::toast::Toast;


#[component]
pub fn MainLayout(children: Children) -> impl IntoView {
    let (toasts, _set_toasts) = use_toasts();

    view! {
        <div class="flex h-screen overflow-hidden bg-slate-50 dark:bg-slate-950 text-slate-900 dark:text-slate-100 font-sans transition-colors duration-300">
            <Sidebar />
            <div class="flex-1 flex flex-col overflow-hidden relative">
                <Header />
                <main class="flex-1 overflow-y-auto">
                    {children()}
                </main>
                
                // Global Toast Container
                <div class="fixed bottom-6 right-6 z-[100] flex flex-col gap-3 pointer-events-none">
                    <For 
                        each=move || toasts.get()
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
