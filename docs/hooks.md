# Hooks

The `src/stdlib/hooks` directory contains custom reactive hooks to encapsulate common logic.

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

### use_theme
Manages the application theme (Light/Dark).
- **Returns**: `(ReadSignal<Theme>, WriteSignal<Theme>)`.
- **Persistence**: Automatically saves and loads the theme from `localStorage`.
