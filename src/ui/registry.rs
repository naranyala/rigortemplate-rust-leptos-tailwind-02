use leptos::prelude::*;

#[derive(PartialEq)]
pub enum DemoCategory {
    Component,
    Hook,
}

pub struct Demo {
    pub name: &'static str,
    pub path: &'static str,
    pub category: DemoCategory,
}

pub const ALL_DEMOS: &[Demo] = &[
    // Components
    Demo { name: "Button Demo", path: "/button", category: DemoCategory::Component },
    Demo { name: "Badge Demo", path: "/badge", category: DemoCategory::Component },
    Demo { name: "Card Demo", path: "/card", category: DemoCategory::Component },
    Demo { name: "Modal Demo", path: "/modal", category: DemoCategory::Component },
    Demo { name: "Progress Demo", path: "/progress", category: DemoCategory::Component },
    Demo { name: "Stats Card Demo", path: "/stats", category: DemoCategory::Component },
    Demo { name: "Data Table Demo", path: "/data-table", category: DemoCategory::Component },
    Demo { name: "Multi Select Demo", path: "/multi-select", category: DemoCategory::Component },
    Demo { name: "Accordion Demo", path: "/accordion", category: DemoCategory::Component },
    Demo { name: "Sliding Panel Demo", path: "/panel", category: DemoCategory::Component },
    // Hooks
    Demo { name: "use_toggle", path: "/toggle", category: DemoCategory::Hook },
    Demo { name: "use_is_mounted", path: "/mounted", category: DemoCategory::Hook },
    Demo { name: "use_clipboard", path: "/clipboard", category: DemoCategory::Hook },
    Demo { name: "use_debounce", path: "/debounce", category: DemoCategory::Hook },
    Demo { name: "use_interval", path: "/interval", category: DemoCategory::Hook },
    Demo { name: "use_local_storage", path: "/local-storage", category: DemoCategory::Hook },
    Demo { name: "use_window_size", path: "/window-size", category: DemoCategory::Hook },
];
