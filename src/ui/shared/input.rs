use leptos::prelude::*;
use wasm_bindgen::JsCast;

/// A simple input component that mirrors the value through getters/setters.
/// This is useful for testing the Input component without complex state management.
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
            <label class="text-sm font-medium text-surface-700">{label}</label>
            <input 
                type="text" 
                placeholder=placeholder
                class=move || {
                    let mut classes = "px-4 py-2 rounded-xl border transition-all outline-none focus:ring-2 focus:ring-accent-500".to_string();
                    if error_clone.is_some() {
                        classes.push_str(" border-red-500 focus:border-red-500");
                    } else {
                        classes.push_str(" border-surface-200 focus:border-accent-500");
                    }
                    classes
                }
                on:input=move |ev| set_value.set(event_target_value(&ev))
                prop:value=value
            />
            {move || error_clone_2.clone().map(|err| view! { 
                <span class="text-xs text-red-500 font-medium">{err}</span> 
            } )}
        </div>
    }
}

fn event_target_value(ev: &web_sys::Event) -> String {
    ev.target()
        .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
        .map(|i| i.value())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to test the class generation logic
    fn get_input_classes(error: Option<String>) -> String {
        let mut classes = "px-4 py-2 rounded-xl border transition-all outline-none focus:ring-2 focus:ring-accent-500".to_string();
        if error.is_some() {
            classes.push_str(" border-red-500 focus:border-red-500");
        } else {
            classes.push_str(" border-surface-200 focus:border-accent-500");
        }
        classes
    }

    #[test]
    fn test_input_classes_with_no_error() {
        let classes = get_input_classes(None);
        assert_eq!(classes, "px-4 py-2 rounded-xl border transition-all outline-none focus:ring-2 focus:ring-accent-500 border-surface-200 focus:border-accent-500");
    }

    #[test]
    fn test_input_classes_with_error() {
        let classes = get_input_classes(Some("Test error".to_string()));
        assert_eq!(classes, "px-4 py-2 rounded-xl border transition-all outline-none focus:ring-2 focus:ring-accent-500 border-red-500 focus:border-red-500");
    }
}
