use leptos::prelude::*;
use crate::ui::layout::{Header, Sidebar};

use crate::ui::shared::toasts::use_toasts;
use crate::ui::shared::toast::Toast;

#[component]
pub fn MainLayout(children: Children) -> impl IntoView {
    let toast_ctx = use_toasts();

    view! {
        <div class="flex h-screen overflow-hidden bg-surface-50 dark:bg-surface-950 text-surface-900 dark:text-surface-100 font-sans transition-colors duration-300">
            <Sidebar />
            <div class="flex-1 flex flex-col overflow-hidden relative">
                <Header />
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
