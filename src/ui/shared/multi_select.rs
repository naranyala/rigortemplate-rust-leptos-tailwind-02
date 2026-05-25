use leptos::prelude::*;

/// A multi-select dropdown component that allows selecting multiple options from a list.
/// 
/// # Arguments
/// * `options` - A vector of (value, label) tuples.
/// * `selected` - An `RwSignal` containing the currently selected values.
/// * `placeholder` - The text to display when no options are selected.
#[component]
pub fn MultiSelect(
    options: Vec<(String, String)>, // (value, label)
    selected: RwSignal<Vec<String>>,
    #[prop(into)] placeholder: String,
) -> impl IntoView {
    let (is_open, set_is_open) = signal(false);
    
    let toggle_selection = move |val: String| {
        selected.update(|s| {
            if s.contains(&val) {
                s.retain(|x| x != &val);
            } else {
                s.push(val);
            }
        });
    };

    let options_for_top = options.clone();
    let options_for_bottom = options.clone();

    view! {
        <div class="relative w-full">
            <div 
                class="px-3 py-2 bg-white dark:bg-gray-900 border border-gray-300 dark:border-gray-700 rounded-md cursor-pointer flex justify-between items-center"
                on:click=move |_| set_is_open.update(|v| *v = !*v)
            >
                <div class="flex flex-wrap gap-1">
                    {move || {
                        let options_inner = options_for_top.clone();
                        if selected.get().is_empty() {
                            view! { <span class="text-gray-400">{placeholder.clone()}</span> }.into_any()
                        } else {
                            let items = selected.get().into_iter().map(|val| {
                                let label = options_inner.iter().find(|(v, _)| v == &val).map(|(_, l)| l.clone()).unwrap_or(val.clone());
                                view! {
                                    <span class="px-2 py-0.5 bg-blue-100 dark:bg-blue-900 text-blue-700 dark:text-blue-200 text-xs rounded-full">
                                        {label}
                                    </span>
                                }.into_any()
                            }).collect::<Vec<_>>();
                            view! { <div>{items}</div> }.into_any()
                        }
                    }}
                </div>
                <div class="text-gray-400 text-xs">"▼"</div>
            </div>
            
            {move || {
                let options_inner = options_for_bottom.clone();
                if is_open.get() {
                    view! {
                        <div class="absolute z-50 w-full mt-1 bg-white dark:bg-gray-900 border border-gray-300 dark:border-gray-700 rounded-md shadow-lg max-h-60 overflow-auto">
                            {options_inner.iter().map(|(val, label)| {
                                let val_clone = val.clone();
                                let label_clone = label.clone();
                                view! {
                                    <div 
                                        class="px-3 py-2 hover:bg-gray-100 dark:hover:bg-gray-800 cursor-pointer flex items-center gap-2"
                                        on:click=move |_| {
                                            let v = val_clone.clone();
                                            toggle_selection(v)
                                        }
                                    >
                                        <input 
                                            type="checkbox" 
                                            checked={
                                                let v = val_clone.clone();
                                                move || selected.get().contains(&v)
                                            }
                                            class="rounded border-gray-300 dark:border-gray-700"
                                        />
                                        <span class="text-sm text-gray-700 dark:text-gray-300">{label_clone}</span>
                                    </div>
                                }.into_any()
                            }).collect::<Vec<_>>()}
                        </div>
                    }.into_any()
                } else {
                    view! { <div /> }.into_any()
                }
            }}
        </div>
    }
}
