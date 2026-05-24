use leptos::prelude::{ElementChild, ClassAttribute};
use leptos::prelude::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="bg-white border-b border-slate-200 h-16 flex items-center justify-between px-4 sm:px-6 lg:px-8 sticky top-0 z-10">
            <div class="flex items-center">
                <button class="md:hidden p-2 mr-2 text-slate-600">
                    "☰"
                </button>
                <h1 class="text-lg font-semibold text-slate-800">"Overview"</h1>
            </div>
            <div class="flex items-center space-x-4">
                <button class="p-2 text-slate-400 hover:text-slate-600">"🔔"</button>
                <div class="w-8 h-8 rounded-full bg-indigo-500 border-2 border-white shadow-sm"></div>
            </div>
        </header>
    }
}
