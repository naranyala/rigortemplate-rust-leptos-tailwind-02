use leptos::prelude::*;

#[component]
pub fn AccordionItem(
    #[prop(into)] title: String,
    children: Children,
) -> impl IntoView {
    let (is_open, set_is_open) = signal(false);

    view! {
        <div class="group border-b border-surface-100 last:border-none">
            <button 
                on:click=move |_| set_is_open.update(|v| *v = !*v)
                class=move || format!(
                    "w-full flex items-center justify-between p-5 text-left transition-all duration-200 {}",
                    if is_open.get() { "bg-accent-50/50" } else { "hover:bg-surface-50" }
                )
            >
                <span class=move || format!(
                    "font-semibold transition-colors duration-200 {}",
                    if is_open.get() { "text-accent-600" } else { "text-surface-700" }
                )>
                    {title}
                </span>
                <div class=move || format!(
                    "w-6 h-6 rounded-full flex items-center justify-center transition-all duration-300 text-xs {}",
                    if is_open.get() { "bg-accent-100 text-accent-600 rotate-180" } else { "bg-surface-100 text-surface-400" }
                )>
                    "⌄"
                </div>
            </button>
            <div 
                class=move || format!(
                    "transition-all duration-300 ease-in-out overflow-hidden {}", 
                    if is_open.get() { "max-h-[1000px] opacity-100" } else { "max-h-0 opacity-0" }
                )
            >
                <div class="p-5 pt-0 text-surface-600 text-sm leading-relaxed">
                    {children()}
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn Accordion(children: Children) -> impl IntoView {
    view! {
        <div class="border border-surface-200 rounded-3xl overflow-hidden bg-white shadow-xl shadow-surface-200/50">
            {children()}
        </div>
    }
}
