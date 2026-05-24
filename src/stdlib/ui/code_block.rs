use leptos::prelude::*;

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

pub(crate) fn highlight_code(code: &str) -> String {
    let bytes = code.as_bytes();
    let len = bytes.len();
    let mut i = 0;
    let mut result = String::new();

    while i < len {
        let c = bytes[i];

        // Line comment: //
        if c == b'/' && i + 1 < len && bytes[i + 1] == b'/' {
            let start = i;
            while i < len && bytes[i] != b'\n' {
                i += 1;
            }
            let text = escape_html(&code[start..i]);
            result.push_str(&format!(
                "<span class=\"text-slate-500 italic\">{}</span>",
                text
            ));
            continue;
        }

        // Raw string: r#"..."#
        if c == b'r' && i + 2 < len && bytes[i + 1] == b'#' && bytes[i + 2] == b'"' {
            let start = i;
            i += 3;
            let mut hash_count = 1;
            while i + 1 < len {
                if bytes[i] == b'"' && bytes[i + 1] == b'#' {
                    let mut j = i + 2;
                    let mut ok = true;
                    for _ in 0..hash_count {
                        if j >= len || bytes[j] != b'#' {
                            ok = false;
                            break;
                        }
                        j += 1;
                    }
                    if ok {
                        i = j;
                        break;
                    }
                }
                if bytes[i] == b'#' {
                    hash_count += 1;
                }
                i += 1;
            }
            let text = escape_html(&code[start..i]);
            result.push_str(&format!(
                "<span class=\"text-emerald-400\">{}</span>",
                text
            ));
            continue;
        }

        // Regular string: "..."
        if c == b'"' {
            let start = i;
            i += 1;
            while i < len && bytes[i] != b'"' {
                if bytes[i] == b'\\' {
                    i += 1;
                    if i < len {
                        i += 1;
                    }
                    continue;
                }
                i += 1;
            }
            if i < len {
                i += 1;
            }
            let text = escape_html(&code[start..i]);
            result.push_str(&format!(
                "<span class=\"text-emerald-400\">{}</span>",
                text
            ));
            continue;
        }

        // Numbers
        if c.is_ascii_digit() {
            let start = i;
            while i < len
                && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'.')
            {
                if bytes[i] == b'.' && i + 1 < len && !bytes[i + 1].is_ascii_digit() {
                    break;
                }
                i += 1;
            }
            let text = escape_html(&code[start..i]);
            result.push_str(&format!(
                "<span class=\"text-amber-400\">{}</span>",
                text
            ));
            continue;
        }

        // Identifiers and keywords
        if c.is_ascii_alphabetic() || c == b'_' {
            let start = i;
            while i < len
                && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_')
            {
                i += 1;
            }
            let word = &code[start..i];
            let text = escape_html(word);

            // Macro: name!
            if i < len && bytes[i] == b'!' {
                result.push_str(&format!(
                    "<span class=\"text-yellow-300\">{}!</span>",
                    text
                ));
                i += 1;
                continue;
            }

            // Keyword
            if is_keyword(word) {
                result.push_str(&format!(
                    "<span class=\"text-purple-400\">{}</span>",
                    text
                ));
                continue;
            }

            // PascalCase -> component/type name
            if is_pascal_case(word) {
                result.push_str(&format!(
                    "<span class=\"text-sky-300\">{}</span>",
                    text
                ));
                continue;
            }

            // Function call: name(
            if i < len && bytes[i] == b'(' {
                result.push_str(&format!(
                    "<span class=\"text-cyan-400\">{}</span>",
                    text
                ));
                continue;
            }

            result.push_str(&text);
            continue;
        }

        // Arrow: ->
        if c == b'-' && i + 1 < len && bytes[i + 1] == b'>' {
            result.push_str("<span class=\"text-slate-400\">-&gt;</span>");
            i += 2;
            continue;
        }

        // Fat arrow: =>
        if c == b'=' && i + 1 < len && bytes[i + 1] == b'>' {
            result.push_str("<span class=\"text-slate-400\">=&gt;</span>");
            i += 2;
            continue;
        }

        // Path separator: ::
        if c == b':' && i + 1 < len && bytes[i + 1] == b':' {
            result.push_str("<span class=\"text-slate-400\">::</span>");
            i += 2;
            continue;
        }

        // Attribute: #[...]
        if c == b'#' && i + 1 < len && bytes[i + 1] == b'[' {
            let start = i;
            i += 2;
            let mut depth = 1;
            while i < len && depth > 0 {
                match bytes[i] {
                    b'[' => depth += 1,
                    b']' => depth -= 1,
                    _ => {}
                }
                if depth == 0 {
                    i += 1;
                    break;
                }
                i += 1;
            }
            let text = escape_html(&code[start..i]);
            result.push_str(&format!(
                "<span class=\"text-orange-400\">{}</span>",
                text
            ));
            continue;
        }

        result.push_str(&escape_html(&code[i..i + 1]));
        i += 1;
    }

    result
}

#[component]
pub fn CodeBlock(code: &'static str) -> impl IntoView {
    let highlighted = highlight_code(code);
    view! {
        <pre class="text-sm font-mono overflow-x-auto p-6 bg-slate-950 rounded-xl leading-relaxed border border-slate-800">
            <code inner_html=highlighted />
        </pre>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highlight_keywords() {
        let code = "fn main() { let x = 5; }";
        let highlighted = highlight_code(code);
        assert!(highlighted.contains("text-purple-400"));
    }

    #[test]
    fn test_highlight_strings() {
        let code = r#"let s = "hello";"#;
        let highlighted = highlight_code(code);
        assert!(highlighted.contains("text-emerald-400"));
    }

    #[test]
    fn test_highlight_raw_strings() {
        let code = r###"let s = r##"raw string"##;"###;
        let highlighted = highlight_code(code);
        assert!(highlighted.contains("text-emerald-400"));
    }

    #[test]
    fn test_highlight_comments() {
        let code = "// this is a comment";
        let highlighted = highlight_code(code);
        assert!(highlighted.contains("text-slate-500 italic"));
    }

    #[test]
    fn test_highlight_attributes() {
        let code = "#[component]";
        let highlighted = highlight_code(code);
        assert!(highlighted.contains("text-orange-400"));
    }

    #[test]
    fn test_highlight_operators() {
        let code = "fn foo() -> i32 { 1 => 2 }";
        let highlighted = highlight_code(code);
        assert!(highlighted.contains("text-slate-400"));
    }

    #[test]
    fn test_highlight_pascal_case() {
        let code = "let x = Button {};";
        let highlighted = highlight_code(code);
        assert!(highlighted.contains("text-sky-300"));
    }
}
