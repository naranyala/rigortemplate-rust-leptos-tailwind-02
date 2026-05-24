use leptos::prelude::*;
use wasm_bindgen::JsCast;
use regex::Regex;

#[derive(Clone, Debug)]
pub struct ValidationRule {
    pub pattern: Regex,
    pub message: String,
}

#[component]
pub fn FormInput(
    #[prop(into)] label: String,
    #[prop(into)] value: RwSignal<String>,
    #[prop(into)] placeholder: String,
    #[prop(optional)] rules: Vec<ValidationRule>,
    #[prop(into)] input_type: String,
) -> impl IntoView {
    let error = Memo::new(move |_| {
        let val = value.get();
        for rule in &rules {
            if !rule.pattern.is_match(&val) {
                return Some(rule.message.clone());
            }
        }
        None
    });

    view! {
        <div class="flex flex-col gap-1.5 w-full">
            <label class="text-sm font-medium text-gray-700 dark:text-gray-300">
                {label}
            </label>
            <input 
                type=input_type
                placeholder=placeholder
                class=move || format!(
                    "px-3 py-2 bg-white dark:bg-gray-900 border rounded-md outline-none transition-colors focus:ring-2 focus:ring-blue-500 {}",
                    if error.get().is_some() { "border-red-500 focus:ring-red-500" } else { "border-gray-300 dark:border-gray-700" }
                )
                on:input=move |ev| {
                    value.set(event_target_value(&ev));
                }
                prop:value=move || value.get()
            />
            {move || {
                if let Some(msg) = error.get() {
                    view! { <span class="text-xs text-red-500">{msg}</span> }.into_any()
                } else {
                    view! { <div /> }.into_any()
                }
            }}
        </div>
    }
}

fn event_target_value(ev: &web_sys::Event) -> String {
    ev.target()
        .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
        .map(|i| i.value())
        .unwrap_or_default()
}
