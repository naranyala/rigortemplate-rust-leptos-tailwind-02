pub struct ComponentMetadata {
    #[allow(dead_code)]
    pub name: &'static str,
    pub full_source: &'static str,
}

pub fn get_ui_components() -> Vec<ComponentMetadata> {
    vec![
        ComponentMetadata {
            name: "Button",
            full_source: include_str!("button.rs"),
        },
        ComponentMetadata {
            name: "Badge",
            full_source: include_str!("badge.rs"),
        },
        ComponentMetadata {
            name: "Card",
            full_source: include_str!("card.rs"),
        },
        ComponentMetadata {
            name: "Input",
            full_source: include_str!("input.rs"),
        },
        ComponentMetadata {
            name: "Modal",
            full_source: include_str!("modal.rs"),
        },
        ComponentMetadata {
            name: "Toast",
            full_source: include_str!("toast.rs"),
        },
    ]
}
