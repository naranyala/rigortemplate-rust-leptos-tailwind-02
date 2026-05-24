use leptos::prelude::*;
use web_sys::{window, MediaQueryList, IntersectionObserver, IntersectionObserverInit, ResizeObserver};
use wasm_bindgen::closure::Closure;
use gloo_storage::{LocalStorage, SessionStorage, Storage};

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn is_keyword(word: &str) -> bool {
    matches!(
        word,
        "fn" | "let" | "mut" | "pub" | "use" | "mod" | "impl" | "struct"
            | "enum" | "match" | "return" | "if" | "else" | "for" | "in"
            | "while" | "true" | "false" | "Some" | "None" | "Ok" | "Err"
            | "move" | "ref" | "as" | "where" | "type" | "trait" | "Self"
            | "const" | "static" | "unsafe" | "async" | "await" | "dyn"
            | "self" | "super" | "crate" | "break" | "continue" | "loop"
            | "extern" | "box" | "default" | "union" | "cfg"
    )
}

fn is_pascal_case(word: &str) -> bool {
    word.starts_with(|c: char| c.is_ascii_uppercase())
}

/// Simple boolean toggle hook.
pub fn use_toggle(initial: bool) -> (ReadSignal<bool>, WriteSignal<bool>) {
    signal(initial)
}

/// Tracks current window size reactively.
pub fn use_window_size() -> (ReadSignal<u32>, ReadSignal<u32>) {
    let (width, set_width) = signal(0);
    let (height, set_height) = signal(0);

    Effect::new(move |_| {
        let window = window().expect("no global window found");
        let handle_resize = Closure::wrap(Box::new(move |_e: web_sys::Event| {
            if let Some(w) = window().and_then(|w| w.inner_width().ok().flatten()) {
                set_width.set(w as u32);
            }
            if let Some(h) = window().and_then(|w| w.inner_height().ok().flatten()) {
                set_height.set(h as u32);
            }
        }) as Box<dyn FnMut(_)>);

        window.add_event_listener_with_callback("resize", handle_resize.as_ref().unchecked_ref())
            .expect("failed to add resize listener");
        
        if let Some(w) = window.inner_width().ok().flatten() { set_width.set(w as u32); }
        if let Some(h) = window.inner_height().ok().flatten() { set_height.set(h as u32); }

        on_cleanup(move || {
            window().expect("no global window found")
                .remove_event_listener_with_callback("resize", handle_resize.as_ref().unchecked_ref())
                .expect("failed to remove resize listener");
        });
    });

    (width, height)
}

/// Tracks the previous value of a signal.
pub fn use_previous<T>(signal: ReadSignal<T>) -> ReadSignal<T>
where
    T: Clone + 'static,
{
    let (prev, set_prev) = signal(signal.get());
    Effect::new(move |_| {
        let current = signal.get();
        let set_prev_clone = set_prev;
        set_timeout(move || {
            set_prev_clone.set(current.clone());
        }, 0);
    });
    prev
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
        let window = window().expect("no global window found");
        let _ = window.navigator().clipboard().write_text(&val);
        set_copied.set(true);
        let set_copied_reset = set_copied;
        set_timeout(move || { set_copied_reset.set(false); }, 2000);
    });
    (copied, set_text)
}

/// Reactively tracks a CSS media query.
pub fn use_media_query(query: &'static str) -> ReadSignal<bool> {
    let (matches, set_matches) = signal(false);
    Effect::new(move |_| {
        let window = window().expect("no global window found");
        let mq_list = window.match_media(query).expect("failed to match media query");
        set_matches.set(mq_list.matches());
        let handle_change = Closure::wrap(Box::new(move |_e: web_sys::MediaQueryListEvent| {
            if let Some(w) = window() {
                if let Ok(mq_list) = w.match_media(query) { set_matches.set(mq_list.matches()); }
            }
        }) as Box<dyn FnMut(_)>);
        mq_list.add_event_listener_with_callback("change", handle_change.as_ref().unchecked_ref())
            .expect("failed to add mq listener");
        on_cleanup(move || {
            mq_list.remove_event_listener_with_callback("change", handle_change.as_ref().unchecked_ref())
                .expect("failed to remove mq listener");
        });
    });
    matches
}

/// Debounces a signal value.
pub fn use_debounce<T>(signal: ReadSignal<T>, delay: u32) -> ReadSignal<T>
where
    T: Clone + 'static,
{
    let (debounced, set_debounced) = signal(signal.get());
    Effect::new(move |_| {
        let val = signal.get();
        let set_debounced_clone = set_debounced;
        set_timeout(move || {
            set_debounced_clone.set(val.clone());
        }, delay as i32);
    });
    debounced
}

/// Reactive interval wrapper.
pub fn use_interval<F>(callback: F, delay: u32, active: ReadSignal<bool>)
where
    F: Fn() + 'static,
{
    Effect::new(move |_| {
        if active.get() {
            let cb = Closure::wrap(Box::new(callback) as Box<dyn FnMut()>);
            let id = window().expect("no window")
                .set_interval_with_callback_and_timeout_and_arguments_0(
                    cb.as_ref().unchecked_ref(),
                    delay as i32,
                ).expect("failed to set interval");
            
            cb.forget(); // In a real app, we'd store the closure to clean up
            on_cleanup(move || {
                window().expect("no window").clear_interval_with_handle(id);
            });
        }
    });
}

/// Simple session storage hook.
pub fn use_session_storage<T>(key: &'static str, initial_value: T) -> (ReadSignal<T>, WriteSignal<T>)
where
    T: Clone + serde::Serialize + serde::de::DeserializeOwned + 'static + Send + Sync,
{
    let initial = SessionStorage::get(key).unwrap_or_else(|_| initial_value.clone());
    let (get, set) = signal(initial);
    Effect::new(move |_| {
        let val = get.with(|v| v.clone());
        let _ = SessionStorage::set(key, val);
    });
    (get, set)
}

/// Detects clicks outside a specific element.
pub fn use_click_outside<T: Into<web_sys::Element>>(
    node_ref: NodeRef<T>, 
    callback: impl Fn() + 'static
) {
    Effect::new(move |_| {
        let node = node_ref.get().expect("node not yet mounted");
        let cb = Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let target = e.target().and_then(|t| t.dyn_into::<web_sys::Node>().ok());
            if let Some(target_node) = target {
                if !node.contains(Some(&target_node)) {
                    callback();
                }
            }
        }) as Box<dyn FnMut(_)>);

        window().expect("no window")
            .add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())
            .expect("failed to add click listener");
        
        on_cleanup(move || {
            window().expect("no window")
                .remove_event_listener_with_callback("click", cb.as_ref().unchecked_ref())
                .expect("failed to remove click listener");
        });
    });
}

/// Reactively tracks the size of a DOM element.
pub fn use_element_size<T: Into<web_sys::Element>>(
    node_ref: NodeRef<T>
) -> (ReadSignal<u32>, ReadSignal<u32>) {
    let (width, set_width) = signal(0);
    let (height, set_height) = signal(0);

    Effect::new(move |_| {
        let node = node_ref.get().expect("node not yet mounted");
        let observer = ResizeObserver::new(Closure::wrap(Box::new(move |entries: Vec<web_sys::ResizeObserverEntry>| {
            if let Some(entry) = entries.first() {
                let rect = entry.content_rect();
                set_width.set(rect.width() as u32);
                set_height.set(rect.height() as u32);
            }
        }) as Box<dyn FnMut(_)>)).expect("failed to create ResizeObserver");

        observer.observe(&node);
        on_cleanup(move || {
            observer.disconnect();
        });
    });

    (width, height)
}

/// Tracks visibility of an element using IntersectionObserver.
pub fn use_intersection_observer<T: Into<web_sys::Element>>(
    node_ref: NodeRef<T>,
    options: IntersectionObserverInit,
) -> ReadSignal<bool> {
    let (is_intersecting, set_is_intersecting) = signal(false);

    Effect::new(move |_| {
        let node = node_ref.get().expect("node not yet mounted");
        let observer = IntersectionObserver::new_with_options(
            Closure::wrap(Box::new(move |entries: Vec<web_sys::IntersectionObserverEntry>| {
                if let Some(entry) = entries.first() {
                    set_is_intersecting.set(entry.is_intersecting());
                }
            }) as Box<dyn FnMut(_)>),
            &options,
        ).expect("failed to create IntersectionObserver");

        observer.observe(&node);
        on_cleanup(move || {
            observer.disconnect();
        });
    });

    is_intersecting
}

fn set_timeout<F>(f: F, ms: i32) 
where F: FnOnce() + 'static 
{
    let closure = Closure::wrap(Box::new(f) as Box<dyn FnMut()>);
    window().expect("no global window found")
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(), 
            ms
        ).expect("failed to set timeout");
    closure.forget();
}
