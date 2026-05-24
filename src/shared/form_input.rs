use leptos::prelude::*;
use wasm_bindgen::JsCast;
use regex::Regex;

/// A rule used to validate the input of a [`FormInput`].
#[derive(Clone, Debug)]
pub struct ValidationRule {
    /// The regular expression to match against the input value.
    pub pattern: Regex,
    /// The error message to display if the pattern does not match.
    pub message: String,
}

/// Validates an input string against a set of [`ValidationRule`]s.
/// 
/// Returns the message of the first rule that fails, or `None` if all rules pass.
pub fn validate_input(value: &str, rules: &[ValidationRule]) -> Option<String> {
    for rule in rules {
        if !rule.pattern.is_match(value) {
            return Some(rule.message.clone());
        }
    }
    None
}

/// A form input component with built-in validation.
/// 
/// # Arguments
/// * `label` - The label text for the input.
/// * `value` - An `RwSignal` containing the current value of the input.
/// * `placeholder` - The placeholder text.
/// * `rules` - An optional list of [`ValidationRule`]s to validate the input.
/// * `input_type` - The HTML input type (e.g., "text", "password", "email").
#[component]
pub fn FormInput(
    #[prop(into)] label: String,
    #[prop(into)] value: RwSignal<String>,
    #[prop(into)] placeholder: String,
    #[prop(optional)] rules: Vec<ValidationRule>,
    #[prop(into)] input_type: String,
) -> impl IntoView {
    let error = Memo::new(move |_| {
        validate_input(&value.get(), &rules)
    });

    view! {
        <div class="flex flex-col gap-1.5 w-full">
            <label class="text-sm font-medium text-gray-700 dark:text-gray-300">
                {label}
            </label>
            <input 
                type=input_type
                placeholder=placeholder
                class=move || format!(
                    "px-3 py-2 bg-white dark:bg-gray-900 border rounded-md outline-none transition-colors focus:ring-2 focus:ring-blue-500 {}",
                    if error.get().is_some() { "border-red-500 focus:ring-red-500" } else { "border-gray-300 dark:border-gray-700" }
                )
                on:input=move |ev| {
                    value.set(event_target_value(&ev));
                }
                prop:value=move || value.get()
            />
            {move || {
                if let Some(msg) = error.get() {
                    view! { <span class="text-xs text-red-500">{msg}</span> }.into_any()
                } else {
                    view! { <div /> }.into_any()
                }
            }}
        </div>
    }
}

fn event_target_value(ev: &web_sys::Event) -> String {
    ev.target()
        .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
        .map(|i| i.value())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_validate_input_empty_rules(val in ".*") {
            let rules = vec![];
            assert_eq!(validate_input(&val, &rules), None);
        }

        #[test]
        fn test_validate_input_numeric_only(val in ".*") {
            let rules = vec![ValidationRule {
                pattern: Regex::new(r"^\d+$").unwrap(),
                message: "Must be numeric".to_string(),
            }];
            
            if val.chars().all(|c| c.is_ascii_digit()) && !val.is_empty() {
                assert_eq!(validate_input(&val, &rules), None);
            } else {
                assert_eq!(validate_input(&val, &rules), Some("Must be numeric".to_string()));
            }
        }

        #[test]
        fn test_validate_input_email_format(val in ".*") {
            let rules = vec![ValidationRule {
                pattern: Regex::new(r"^\S+@\S+\.\S+$").unwrap(),
                message: "Invalid email".to_string(),
            }];
            
            let is_valid = Regex::new(r"^\S+@\S+\.\S+$").unwrap().is_match(&val);
            if is_valid {
                assert_eq!(validate_input(&val, &rules), None);
            } else {
                assert_eq!(validate_input(&val, &rules), Some("Invalid email".to_string()));
            }
        }
    }
}
