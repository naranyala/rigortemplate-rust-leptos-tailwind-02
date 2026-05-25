use leptos::prelude::*;

#[component]
pub fn Tooltip(
    children: Children,
    text: String,
) -> impl IntoView {
    let (is_visible, set_is_visible) = signal(false);

    view! {
        <div 
            class="relative inline-block"
            on:mouseenter=move |_| set_is_visible.set(true)
            on:mouseleave=move |_| set_is_visible.set(false)
        >
            {children()}
            {move || {
                if is_visible.get() {
                    view! {
                        <div class="absolute bottom-full left-1/2 -translate-x-1/2 mb-2 px-2 py-1 text-xs text-white bg-surface-800 rounded shadow-lg whitespace-nowrap z-50 pointer-events-none animate-in fade-in zoom-in duration-150">
                            {text.clone()}
                            <div class="absolute top-full left-1/2 -translate-x-1/2 border-4 border-transparent border-t-surface-800"></div>
                        </div>
                    }.into_any()
                } else {
                    view! { <div /> }.into_any()
                }
            }}
        </div>
    }
}
