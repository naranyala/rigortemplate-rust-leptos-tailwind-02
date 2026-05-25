use leptos::prelude::*;
use crate::shared::theme::{use_theme_context, Theme};

#[component]
pub fn Header() -> impl IntoView {
    let theme_ctx = use_theme_context();
    let theme = theme_ctx.theme;
    let set_theme = theme_ctx.set_theme;

    let toggle_theme = move |_| {
        set_theme.update(|t| {
            *t = if *t == Theme::Light {
                Theme::Dark
            } else {
                Theme::Light
            };
        });
    };

    view! {
        <header class="bg-white dark:bg-slate-900 border-b border-slate-200 dark:border-slate-800 h-16 flex items-center justify-between px-4 sm:px-6 lg:px-8 sticky top-0 z-10 transition-colors duration-300">
            <div class="flex items-center">
                <button class="md:hidden p-2 mr-2 text-slate-600 dark:text-slate-400">
                    "☰"
                </button>
                <h1 class="text-lg font-semibold text-slate-800 dark:text-slate-100">"Overview"</h1>
            </div>
            <div class="flex items-center space-x-4">
                <button 
                    on:click=toggle_theme
                    class="p-2 text-slate-400 hover:text-slate-600 dark:hover:text-slate-200 transition-colors rounded-full hover:bg-slate-100 dark:hover:bg-slate-800"
                >
                    {move || if theme.get() == Theme::Light {
                        "🌙"
                    } else {
                        "☀️"
                    }}
                </button>
                <button class="p-2 text-slate-400 hover:text-slate-600 dark:hover:text-slate-200">"🔔"</button>
                <div class="w-8 h-8 rounded-full bg-indigo-500 border-2 border-white dark:border-slate-900 shadow-sm"></div>
            </div>
        </header>
    }
}
