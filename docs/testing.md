# Testing Strategy

This project employs a multi-layered testing strategy to ensure reliability and stability across the application and its standard library.

## Testing Tiers

### 1. Unit Testing (Rust Logic)
Most core logic, such as state management, service business logic, and pure helper functions, is tested using standard `cargo test`.

- **State Management**: Tests in `src/stdlib/state.rs` verify that signals update correctly and that notifications are handled properly.
- **Services**: Tests in `src/stdlib/services/api.rs` utilize the `mockito` crate to simulate HTTP responses. This allows testing of the `ApiClient` without an actual running server, covering success and failure scenarios (network errors, serialization errors, and HTTP error status codes).
- **Hooks**: Core hooks that don't depend on DOM elements are tested using standard Rust unit tests.

### 2. WASM/Browser Testing (UI & DOM)
Components and hooks that interact with the DOM (like `use_clipboard`, `use_is_mounted`, and UI components) require a browser environment for testing. These tests are implemented using `wasm-bindgen-test`.

- **Usage**: Run these tests using `wasm-bindgen-test-runner` or through the `cargo test` command with the WASM target.
- **Scope**: Covers component rendering, event handling (clicks, inputs), and DOM-dependent hooks.

## Tools and Libraries

- **[tokio](https://tokio.rs/)**: Used as the async runtime for testing asynchronous services like `ApiClient`.
- **[mockito](https://github.com/stevearc/mockito)**: Provides a mock HTTP server for testing service layers.
- **[wasm-bindgen-test](https://github.com/rustwasm/wasm-bindgen-test)**: Allows running Rust tests in a headless browser.
- **[serde](https://serde.rs/)**: Used for testing serialization and deserialization logic.

## Running Tests

### Run all Rust tests
```bash
cargo test
```

### Run WASM tests (Requires browser environment)
```bash
wasm-pack test --headless --chrome
```
