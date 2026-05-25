# Testing Strategy

This project employs a multi-layered testing strategy to ensure reliability and stability across the application.

## Testing Tiers

### 1. Unit Testing (Rust Logic)
Core logic, such as state management, service business logic, and pure helper functions, is tested using standard `cargo test`.

- **State Management**: Tests verify that signals update correctly and that notifications are handled properly.
- **Services**: Tests in `src/core/api.rs` utilize the `mockito` crate to simulate HTTP responses. This tests the `ApiClient` without an actual running server, covering success and failure scenarios.
- **Utilities**: The `cn` class merging utility in `src/shared/utils/cn.rs` has comprehensive tests for merging, modifiers, text categories, arbitrary values, and edge cases.

### 2. WASM/Browser Testing (UI & DOM)
Components and hooks that interact with the DOM require a browser environment. These tests are implemented using `wasm-bindgen-test`.

## Tools and Libraries

- **[tokio](https://tokio.rs/)**: Used as the async runtime for testing asynchronous services like `ApiClient` (dev-dependency only).
- **[mockito](https://github.com/stevearc/mockito)**: Provides a mock HTTP server for testing service layers.
- **[wasm-bindgen-test](https://github.com/rustwasm/wasm-bindgen-test)**: Allows running Rust tests in a headless browser.

## Running Tests

### Run all Rust tests
```bash
cargo test
```

### Run WASM tests (Requires browser environment)
```bash
wasm-pack test --headless --chrome
```
