# Hooks

The `src/hooks` directory contains custom reactive hooks to encapsulate common logic.

## Common Hooks

### use_toggle
A simple boolean toggle.
- **Returns**: `(ReadSignal<bool>, WriteSignal<bool>)`.

### use_is_mounted
Tracks whether a component is currently mounted in the DOM.
- **Returns**: `ReadSignal<bool>`.

### use_clipboard
Handles copying text to the system clipboard.
- **Returns**: `(ReadSignal<bool>, WriteSignal<String>)`.
- **Behavior**: Sets a copied flag to true for 2 seconds after the text is updated.

### use_theme_context
Manages the application theme (Light/Dark).
- **Returns**: `ThemeContext` containing `theme: ReadSignal<Theme>` and `set_theme: WriteSignal<Theme>`.
- **Persistence**: Automatically saves and loads the theme from `localStorage`.
