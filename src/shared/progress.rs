use leptos::prelude::*;

#[component]
pub fn Progress(
    value: ReadSignal<f64>,
    max: f64,
    class: Option<String>,
) -> impl IntoView {
    let percentage = Memo::new(move |_| {
        ((value.get() / max) * 100.0).clamp(0.0, 100.0)
    });

    view! {
        <div class=move || format!("w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2.5 {}", class.clone().unwrap_or_default())>
            <div 
                class="bg-blue-600 h-2.5 rounded-full transition-all duration-300 ease-out"
                style=move || format!("width: {}%", percentage.get())
            ></div>
        </div>
    }
}
