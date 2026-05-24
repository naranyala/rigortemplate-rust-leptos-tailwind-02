use leptos::prelude::*;
use web_sys::window;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

#[allow(dead_code)]
/// Simple boolean toggle hook.
pub fn use_toggle(initial: bool) -> (ReadSignal<bool>, WriteSignal<bool>) {
    signal(initial)
}

/// Tracks if the component is currently mounted.
pub fn use_is_mounted() -> ReadSignal<bool> {
    let (is_mounted, set_is_mounted) = signal(false);
    Effect::new(move |_| {
        set_is_mounted.set(true);
    });
    is_mounted
}

/// Handles copying text to the clipboard.
pub fn use_clipboard() -> (ReadSignal<bool>, WriteSignal<String>) {
    let (copied, set_copied) = signal(false);
    let (text, set_text) = signal(String::new());
    Effect::new(move |_| {
        let val = text.get();
        if val.is_empty() { return; }
        let win = window().expect("no global window found");
        let _ = win.navigator().clipboard().write_text(&val);
        set_copied.set(true);
        let set_copied_reset = set_copied;
        set_timeout(move || { set_copied_reset.set(false); }, 2000);
    });
    (copied, set_text)
}

fn set_timeout<F>(f: F, ms: i32) 
where F: FnMut() + 'static 
{
    let closure = Closure::wrap(Box::new(f) as Box<dyn FnMut()>);
    window().expect("no global window found")
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(), 
            ms
        ).expect("failed to set timeout");
    closure.forget();
}
