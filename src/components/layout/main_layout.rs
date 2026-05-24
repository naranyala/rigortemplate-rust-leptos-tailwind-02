use leptos::prelude::{ElementChild, ClassAttribute};
use leptos::prelude::*;
use crate::components::layout::{Header, Sidebar};

use crate::components::views::*;
use crate::app::AppRoute;
use crate::stdlib::hooks::toasts::use_toasts;
use crate::stdlib::ui::Toast;

#[component]
pub fn MainLayout(route: Signal<AppRoute>) -> impl IntoView {
    let (toasts, _set_toasts) = use_toasts();

    view! {
        <div class="flex h-screen overflow-hidden bg-slate-50 text-slate-900 font-sans">
            <Sidebar route=route />
            <div class="flex-1 flex flex-col overflow-hidden relative">
                <Header />
                <main class="flex-1 overflow-y-auto">
                    {move || match route.get() {
                        AppRoute::Catalog => view! { <CatalogView /> }.into_any(),
                        AppRoute::Hooks => view! { <HooksCatalogView /> }.into_any(),
                        AppRoute::Accordion => view! { <AccordionView /> }.into_any(),
                        AppRoute::Panel => view! { <PanelView /> }.into_any(),
                    }}
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
