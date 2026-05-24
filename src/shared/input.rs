use leptos::prelude::{ElementChild, ClassAttribute};
use leptos::prelude::*;

#[component]
pub fn Input(
    #[prop(into)] label: String, 
    #[prop(into)] placeholder: String, 
    #[prop(into)] value: Signal<String>,
    #[prop(into)] set_value: WriteSignal<String>,
    error: Option<String>,
) -> impl IntoView {
    let error_clone = error.clone();
    let error_clone_2 = error.clone();
    
    view! {
        <div class="flex flex-col space-y-1.5">
            <label class="text-sm font-medium text-slate-700">{label}</label>
            <input 
                type="text" 
                placeholder=placeholder
                class=move || {
                    let mut classes = "px-4 py-2 rounded-xl border transition-all outline-none focus:ring-2 focus:ring-indigo-500".to_string();
                    if error_clone.is_some() {
                        classes.push_str(" border-red-500 focus:border-red-500");
                    } else {
                        classes.push_str(" border-slate-200 focus:border-indigo-500");
                    }
                    classes
                }
                on:input=move |ev| set_value.set(event_target_value(&ev))
                prop:value=value
            />
            {move || error_clone_2.clone().map(|err| view! { 
                <span class="text-xs text-red-500 font-medium">{err}</span> 
            })}
        </div>
    }
}
