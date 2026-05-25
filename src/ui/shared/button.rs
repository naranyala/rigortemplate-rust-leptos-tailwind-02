use leptos::prelude::{ElementChild, ClassAttribute};
use leptos::prelude::*;

#[component]
pub fn Button(#[prop(optional)] variant: Option<&'static str>, children: Children) -> impl IntoView {
    let base_style = "px-4 py-2 rounded-lg font-medium transition-all active:scale-95";
    let variant_style = match variant {
        Some("primary") => "bg-indigo-600 text-white hover:bg-indigo-700 dark:bg-indigo-500 dark:hover:bg-indigo-600",
        Some("secondary") => "bg-slate-200 text-slate-800 hover:bg-slate-300 dark:bg-slate-800 dark:text-slate-200 dark:hover:bg-slate-700",
        Some("outline") => "border border-slate-300 text-slate-700 hover:bg-slate-50 dark:border-slate-700 dark:text-slate-300 dark:hover:bg-slate-800",
        _ => "bg-indigo-600 text-white hover:bg-indigo-700 dark:bg-indigo-500 dark:hover:bg-indigo-600",
    };

    view! {
        <button class=format!("{} {}", base_style, variant_style)>
            {children()}
        </button>
    }
}
