use leptos::prelude::*;

#[component]
pub fn Button(#[prop(optional)] variant: Option<&'static str>, children: Children) -> impl IntoView {
    let base_style = "px-4 py-2 rounded-lg font-medium transition-all active:scale-95";
    let variant_style = match variant {
        Some("primary") => "bg-accent-600 text-white hover:bg-accent-700 dark:bg-accent-500 dark:hover:bg-accent-600",
        Some("secondary") => "bg-surface-200 text-surface-800 hover:bg-surface-300 dark:bg-surface-800 dark:text-surface-200 dark:hover:bg-surface-700",
        Some("outline") => "border border-surface-300 text-surface-700 hover:bg-surface-50 dark:border-surface-700 dark:text-surface-300 dark:hover:bg-surface-800",
        _ => "bg-accent-600 text-white hover:bg-accent-700 dark:bg-accent-500 dark:hover:bg-accent-600",
    };

    view! {
        <button class=format!("{} {}", base_style, variant_style)>
            {children()}
        </button>
    }
}
