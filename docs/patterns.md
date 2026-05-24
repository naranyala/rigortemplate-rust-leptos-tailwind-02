# Design Patterns

This document details the primary architectural patterns implemented in the boilerplate to promote code reuse and scalability.

## Dependency Injection (Service Provider Pattern)

To prevent "prop-drilling" and manage long-lived external dependencies (like API clients), the project uses a Service Provider pattern based on Leptos's context system.

### Implementation Details
- **`ServiceProvider`**: A central registry located in `src/stdlib/services/provider.rs`. It is responsible for initializing services (e.g., creating an `Arc<ApiClient>`) and providing them to the component tree.
- **Context Injection**: Services are injected using `provide_context` at the root of the application (`src/app.rs`).
- **Consumption**: Components access services using type-safe helper functions (e.g., `use_api()`), which internally call `use_context`.

### Advantages
- **Testability**: By using a trait-based approach or providing different implementations via context, services can be easily swapped with mocks during testing.
- **Decoupling**: Components depend on a service interface rather than a concrete implementation.
- **Lifecycle Management**: Services are initialized once at application startup and exist for the lifetime of the application.

## Global State Management

The project manages application-wide state (e.s. user authentication, global notifications, theme) using a centralized context pattern.

- **Pattern**: A single `GlobalState` struct is created, containing various `RwSignal` fields.
- **Provision**: The state is provided via `provide_context` in the `App` component.
- **Reactivity**: Because state is stored in `RwSignal`s, any component consuming the state will automatically re-render when the state changes.

## Layout Composition

The project employs a composition-based layout strategy to allow for maximum flexibility.

- **`MainLayout`**: Acts as a generic shell. Instead of being coupled to specific routes, it accepts `children: Children`.
- **Routing Decoupling**: The responsibility of deciding *which* view to render is moved up to the `App` component. The `App` component matches the current route and passes the appropriate view as a child to the `MainLayout`.
- **Flexibility**: This allows for easy creation of different layout types (e.g., `AuthLayout` for login pages, `DashboardLayout` for authenticated users) without duplicating the core structure.
