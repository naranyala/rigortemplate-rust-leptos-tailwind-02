use std::collections::HashMap;

/// Defines Tailwind conflict groups.
const CONFLICT_GROUPS: &[(&str, &[&str])] = &[
    ("padding-x", &["px-", "pl-", "pr-"]),
    ("padding-y", &["py-", "pt-", "pb-"]),
    ("padding-all", &["p-"]),
    ("margin-x", &["mx-", "ml-", "mr-"]),
    ("margin-y", &["my-", "mt-", "mb-"]),
    ("margin-all", &["m-"]),
    ("width", &["w-"]),
    ("height", &["h-"]),
    ("rounded", &["rounded-"]),
    ("font-weight", &["font-"]),
    ("bg-color", &["bg-"]),
    ("border-width", &["border-"]),
];

fn is_text_size(class: &str) -> bool {
    let sizes = ["xs", "sm", "base", "lg", "xl", "2xl", "3xl", "4xl", "5xl", "6xl", "7xl", "8xl", "9xl"];
    let value = class.strip_prefix("text-").unwrap_or("");
    sizes.contains(&value)
}

fn get_category(class: &str) -> Option<&'static str> {
    // Handle negative values by stripping the leading minus
    let stripped = class.strip_prefix('-').unwrap_or(class);
    
    if stripped.starts_with("text-") {
        return if is_text_size(stripped) {
            Some("font-size")
        } else {
            Some("text-color")
        };
    }

    for (category, prefixes) in CONFLICT_GROUPS {
        for prefix in *prefixes {
            if stripped.starts_with(prefix) {
                return Some(category);
            }
        }
    }
    None
}

pub fn merge(classes: &str) -> String {
    let mut final_classes = Vec::new();
    let mut category_map: HashMap<(Option<&str>, &str), usize> = HashMap::new();

    for full_class in classes.split_whitespace() {
        // Handle multiple modifiers (e.g., "dark:hover:bg-red-500")
        let (modifier, class_name) = if let Some(last_colon) = full_class.rfind(':') {
            let (mod_part, class_part) = full_class.split_at(last_colon);
            (Some(&mod_part[..]), Some(&class_part[1..]))
        } else {
            (None, Some(full_class))
        };

        if let Some(class_name) = class_name {
            if let Some(category) = get_category(class_name) {
                let key = (modifier, category);
                if let Some(&index) = category_map.get(&key) {
                    final_classes[index] = full_class.to_string();
                } else {
                    category_map.insert(key, final_classes.len());
                    final_classes.push(full_class.to_string());
                }
            } else {
                final_classes.push(full_class.to_string());
            }
        }
    }

    final_classes.join(" ")
}

pub fn cn(classes: &[Option<&str>]) -> String {
    let joined = classes
        .iter()
        .flatten()
        .cloned()
        .collect::<Vec<_>>()
        .join(" ");
    
    merge(&joined)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_basic() {
        assert_eq!(merge("p-2 p-4"), "p-4");
        assert_eq!(merge("m-1 m-2"), "m-2");
        assert_eq!(merge("w-10 w-20"), "w-20");
    }

    #[test]
    fn test_merge_modifiers() {
        assert_eq!(merge("bg-red-500 hover:bg-blue-500"), "bg-red-500 hover:bg-blue-500");
        assert_eq!(merge("hover:bg-red-500 hover:bg-blue-500"), "hover:bg-blue-500");
        assert_eq!(merge("md:p-4 lg:p-8"), "md:p-4 lg:p-8");
    }

    #[test]
    fn test_merge_text_categories() {
        assert_eq!(merge("text-lg text-red-500"), "text-lg text-red-500");
        assert_eq!(merge("text-red-500 text-lg"), "text-red-500 text-lg");
        assert_eq!(merge("text-sm text-lg"), "text-lg");
        assert_eq!(merge("text-red-500 text-blue-500"), "text-blue-500");
    }

    #[test]
    fn test_merge_complex() {
        assert_eq!(
            merge("p-2 m-2 bg-red-500 hover:p-4 hover:bg-blue-500 p-6"), 
            "p-6 m-2 bg-red-500 hover:p-4 hover:bg-blue-500"
        );
        assert_eq!(
            merge("rounded-sm rounded-lg md:rounded-xl"),
            "rounded-lg md:rounded-xl"
        );
    }

    #[test]
    fn test_cn_conditional() {
        let is_active = true;
        let is_disabled = false;
        
        let result = cn(&[
            Some("px-4 py-2"),
            if is_active { Some("bg-accent-500 text-white") } else { None },
            if is_disabled { Some("opacity-50 cursor-not-allowed") } else { None },
            Some("px-6"), 
        ]);
        
        assert_eq!(result, "px-6 py-2 bg-accent-500 text-white");
    }

    #[test]
    fn test_cn_empty_and_none() {
        assert_eq!(cn(&[]), "");
        assert_eq!(cn(&[None, None]), "");
        assert_eq!(cn(&[Some(""), None]), "");
    }

    #[test]
    fn test_non_tailwind_classes() {
        assert_eq!(merge("custom-class-1 custom-class-2"), "custom-class-1 custom-class-2");
        assert_eq!(merge("custom-class-1 p-2 custom-class-1"), "custom-class-1 p-2 custom-class-1");
    }

    #[test]
    fn test_arbitrary_values() {
        assert_eq!(merge("w-[100px] w-[200px]"), "w-[200px]");
        assert_eq!(merge("h-[10rem] h-[20rem]"), "h-[20rem]");
    }

    #[test]
    fn test_negative_values() {
        assert_eq!(merge("-mt-1 -mt-2"), "-mt-2");
        assert_eq!(merge("-ml-4 -ml-2"), "-ml-2");
    }

    #[test]
    fn test_stacked_modifiers() {
        assert_eq!(
            merge("dark:hover:bg-red-500 dark:hover:bg-blue-500"), 
            "dark:hover:bg-blue-500"
        );
        assert_eq!(
            merge("md:dark:bg-surface-800 md:dark:bg-surface-900"),
            "md:dark:bg-surface-900"
        );
    }

    #[test]
    fn test_whitespace_handling() {
        assert_eq!(merge("  p-2    p-4  "), "p-4");
        assert_eq!(merge("\t p-2 \n p-4 "), "p-4");
    }

    #[test]
    fn test_axis_independence() {
        // px and py should not conflict
        assert_eq!(merge("px-4 py-2"), "px-4 py-2");
        assert_eq!(merge("py-2 px-4"), "py-2 px-4");
        // pl and pr should not conflict with py
        assert_eq!(merge("pl-2 py-4"), "pl-2 py-4");
    }
}
