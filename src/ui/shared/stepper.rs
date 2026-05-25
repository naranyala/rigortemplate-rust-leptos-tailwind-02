use leptos::prelude::*;

#[derive(Clone, Debug)]
pub struct Step {
    pub label: String,
    pub description: String,
}

#[component]
pub fn Stepper(
    steps: Vec<Step>,
    current_step: ReadSignal<usize>,
    _on_step_change: WriteSignal<usize>,
) -> impl IntoView {
    view! {
        <div class="flex items-center w-full">
                {steps.iter().enumerate().map(|(i, step)| {
                    let is_completed = move || current_step.get() > i;
                    let is_active = move || current_step.get() == i;
                    let steps_len = steps.len();

                
                view! {
                    <div class="flex items-center flex-1">
                        <div class="flex flex-col items-center relative">
                            <div 
                                class=move || format!(
                                    "w-10 h-10 rounded-full flex items-center justify-center font-bold transition-colors duration-300 z-10 {}",
                                    if is_active() { "bg-accent-600 text-white" } 
                                    else if is_completed() { "bg-green-500 text-white" } 
                                    else { "bg-surface-200 dark:bg-surface-700 text-surface-500 dark:text-surface-400" }
                                )
                            >
                                {move || if is_completed() { "✓".to_string() } else { i.to_string() }}
                            </div>
                                <span class="absolute top-12 text-xs font-medium text-center w-24">
                                    {step.label.clone()}
                                </span>

                        </div>
                            {if i < steps_len - 1 {
                                view! {
                                    <div class="flex-1 h-1 mx-4 bg-surface-200 dark:bg-surface-700 relative">
                                        <div 
                                            class="absolute top-0 left-0 h-full bg-accent-600 transition-all duration-300"
                                            style=move || format!("width: {}%", if is_completed() { 100 } else { 0 })
                                        ></div>
                                    </div>
                                }.into_any()
                            } else {
                                view! { <div /> }.into_any()
                            }}


                    </div>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}
