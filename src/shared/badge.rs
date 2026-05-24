use leptos::prelude::{ElementChild, ClassAttribute};
use leptos::prelude::*;

#[component]
pub fn Badge(text: &'static str, color: &'static str) -> impl IntoView {
    let color_style = match color {
        "green" => "bg-green-100 text-green-700",
        "red" => "bg-red-100 text-red-700",
        "blue" => "bg-blue-100 text-blue-700",
        _ => "bg-slate-100 text-slate-700",
    };

    view! {
        <span class=format!("px-2 py-0.5 rounded-full text-xs font-medium {}", color_style)>
            {text}
        </span>
    }
}
