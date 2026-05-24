use leptos::prelude::{ElementChild, ClassAttribute};
use leptos::prelude::*;

#[component]
pub fn Toast(
    #[prop(into)] message: String,
    #[prop(into)] toast_type: String, // "success", "error", "info"
) -> impl IntoView {
    let color_class = match toast_type.as_str() {
        "success" => "bg-green-500",
        "error" => "bg-red-500",
        _ => "bg-indigo-500",
    };

    view! {
        <div class=format!("{} text-white px-4 py-3 rounded-2xl shadow-lg flex items-center space-x-3 animate-in slide-in-from-right duration-300", color_class)>
            <span class="text-lg">"🔔"</span>
            <span class="text-sm font-medium">{message}</span>
        </div>
    }
}
