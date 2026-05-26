use leptos::prelude::*;
use leptos_router::hooks::use_location;
use crate::shared::theme::{use_theme_context, Theme};

#[component]
pub fn Header(set_is_open: WriteSignal<bool>) -> impl IntoView {
    let theme_ctx = use_theme_context();
    let theme = theme_ctx.theme;
    let set_theme = theme_ctx.set_theme;
    let location = use_location();

    // Function to create a label from a path segment
    let path_label = move |segment: &str| {
        // Replace hyphens with spaces and capitalize each word
        segment.replace('-', " ")
            .split_whitespace()
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    };

    view! {
        <header class="bg-white dark:bg-surface-900 border-b border-surface-200 dark:border-surface-800 h-16 flex items-center justify-between px-4 sm:px-6 lg:px-8 sticky top-0 z-10 transition-colors duration-300">
            <div class="flex items-center space-x-3">
                <button 
                    on:click=move |_| set_is_open.set(true)
                    class="md:hidden p-3 mr-2 text-2xl text-surface-600 dark:text-surface-400 hover:bg-surface-100 dark:hover:bg-surface-800 rounded-xl transition-colors"
                >
                    "☰"
                </button>
                    {/* Breadcrumbs */}
                    <div class="flex items-center space-x-2 text-surface-500 dark:text-surface-400 bg-surface-50 dark:bg-surface-950/50 px-3 py-1.5 rounded-xl">
                        {move || {
                            let pathname = location.pathname.get();
                            let path_segments = pathname.split('/').filter(|s| !s.is_empty()).collect::<Vec<_>>();
                            let extra = if pathname != "/" && pathname != "" {
                                view! {
                                    <>
                                        <span class="mx-2">></span>
                                        <span class="text-surface-700 dark:text-surface-300 font-medium">{path_label(path_segments.last().unwrap_or(&""))}</span>
                                    </>
                                }
                                .into_any()
                            } else {
                                view! { <span></span> }
                                .into_any()
                            };

                            view! {
                                <a href="/" class="hover:text-surface-700 dark:hover:text-surface-300 transition-colors">Home</a>
                                {extra}
                                <span class="sr-only">(current page)</span>
                            }
                        }}
                    </div>
            </div>
            <div class="flex items-center">
                <button 
                    on:click=move |_| {
                        set_theme.update(|t| {
                            *t = if *t == Theme::Light {
                                Theme::Dark
                            } else {
                                Theme::Light
                            };
                        });
                    }
                    class="p-2 text-surface-400 hover:text-surface-600 dark:hover:text-surface-200 transition-colors rounded-full hover:bg-surface-100 dark:hover:bg-surface-800"
                >
                    {move || if theme.get() == Theme::Light {
                        "🌙"
                    } else {
                        "☀️"
                    }}
                </button>
            </div>
        </header>
    }
}
