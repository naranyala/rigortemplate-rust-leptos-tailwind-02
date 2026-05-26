use leptos::prelude::*;
use web_sys::window;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    pub fn as_str(&self) -> &'static str {
        match self {
            Theme::Light => "light",
            Theme::Dark => "dark",
        }
    }
}

#[derive(Clone, Debug)]
pub struct ThemeContext {
    pub theme: ReadSignal<Theme>,
    pub set_theme: WriteSignal<Theme>,
}

pub fn provide_theme_context() {
     let initial_theme = window()
         .and_then(|win| win.local_storage().ok().flatten())
         .and_then(|storage| storage.get_item("theme").ok().flatten())
         .map(|t| if t == "dark" { Theme::Dark } else { Theme::Light })
         .unwrap_or(Theme::Dark);

    let (theme, set_theme) = signal(initial_theme);

    Effect::new(move |_| {
        let current_theme = theme.get();
        if let Some(win) = window() {
            if let Some(doc) = win.document() {
                if let Some(html) = doc.document_element() {
                    let _ = html.class_list().remove_1("light");
                    let _ = html.class_list().remove_1("dark");
                    let _ = html.class_list().add_1(current_theme.as_str());
                }
            }
            if let Ok(Some(storage)) = win.local_storage() {
                let _ = storage.set_item("theme", current_theme.as_str());
            }
        }
    });

    provide_context(ThemeContext { theme, set_theme });
}

pub fn use_theme_context() -> ThemeContext {
    use_context::<ThemeContext>().expect("ThemeContext not provided")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_as_str() {
        assert_eq!(Theme::Light.as_str(), "light");
        assert_eq!(Theme::Dark.as_str(), "dark");
    }

    #[test]
    fn test_theme_equality() {
        assert_eq!(Theme::Light, Theme::Light);
        assert_eq!(Theme::Dark, Theme::Dark);
        assert_ne!(Theme::Light, Theme::Dark);
    }
}
