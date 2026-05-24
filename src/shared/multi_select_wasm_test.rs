use wasm_bindgen_test::*;
use leptos::prelude::*;
use crate::shared::multi_select::MultiSelect;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_multi_select_render() {
    let options = vec![("1".to_string(), "Option 1".to_string()), ("2".to_string(), "Option 2".to_string())];
    let selected = RwSignal::new(vec![]);
    
    // This is a very minimal test to see if it even runs
    let _view = view! { <MultiSelect options=options selected=selected placeholder="Select...".to_string() /> };
    
    // For now, we just ensure it doesn't panic during creation.
}
