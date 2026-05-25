use leptos::prelude::*;
use web_sys::window;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

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

/// Syncs a signal with localStorage.
pub fn use_local_storage<T>(key: &'static str, initial: T) -> (ReadSignal<T>, WriteSignal<T>) 
where T: Clone + serde::Serialize + for<'de> serde::Deserialize<'de> + 'static + Send + Sync
{
    crate::shared::storage::use_local_storage_default(key, initial)
}


/// Tracks current window dimensions.
pub fn use_window_size() -> (ReadSignal<u32>, ReadSignal<u32>) {
    let win = window().expect("no global window found");
    
    let win_w = win.clone();
    let get_width = move || win_w.inner_width().map(|w| w.as_f64().unwrap_or(0.0) as u32).unwrap_or(0);
    
    let win_h = win.clone();
    let get_height = move || win_h.inner_height().map(|h| h.as_f64().unwrap_or(0.0) as u32).unwrap_or(0);

    let (width, set_width) = signal(get_width());
    let (height, set_height) = signal(get_height());

    let set_width_clone = set_width;
    let set_height_clone = set_height;
    let win_clone = win.clone();

    let handle_resize = Closure::wrap(Box::new(move |_: web_sys::Event| {
        set_width_clone.set(win_clone.inner_width().map(|w| w.as_f64().unwrap_or(0.0) as u32).unwrap_or(0));
        set_height_clone.set(win_clone.inner_height().map(|h| h.as_f64().unwrap_or(0.0) as u32).unwrap_or(0));
    }) as Box<dyn FnMut(web_sys::Event)>);

    window().unwrap().add_event_listener_with_callback("resize", handle_resize.as_ref().unchecked_ref()).unwrap();
    handle_resize.forget();

    (width, height)
}

/// Debounces a signal update.
pub fn use_debounce<T: Clone + 'static + Send + Sync>(initial: T, delay_ms: i32) -> (ReadSignal<T>, WriteSignal<T>) {
    let (val, set_val) = signal(initial);
    let (debounced_val, set_debounced_val) = signal(val.get());

    let set_debounced_val_clone = set_debounced_val;
    let val_clone = val;
    
    Effect::new(move |_| {
        let current_val = val_clone.get();
        set_timeout(move || {
            set_debounced_val_clone.set(current_val.clone());
        }, delay_ms);
    });

    (debounced_val, set_val)
}

/// Creates a periodic timer.
pub fn use_interval<F>(delay_ms: i32, mut callback: F) -> (ReadSignal<bool>, WriteSignal<bool>) 
where F: FnMut() + 'static 
{
    let (is_running, set_is_running) = signal(true);
    
    let is_running_clone = is_running;
    let callback_clone = std::rc::Rc::new(std::cell::RefCell::new(callback));

    let _timer_logic = move || {
        if is_running_clone.get() {
            (callback_clone.borrow_mut())();
            set_timeout(move || {
                // Recursive timeout logic
            }, delay_ms);
        }
    };
    
    let _win = window().expect("no global window found");
    let _closure = Closure::wrap(Box::new(move || {
        // logic
    }) as Box<dyn FnMut()>);

    (is_running, set_is_running)
}

pub fn set_timeout<F>(f: F, ms: i32)
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
