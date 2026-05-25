# Design Patterns

This document details the primary architectural patterns implemented in the boilerplate to promote code reuse and scalability.

## Dependency Injection (Service Provider Pattern)

To prevent prop-drilling and manage long-lived external dependencies (like API clients), the project uses a context-based service initialization pattern.

### Implementation Details
- **`provide_services()`**: A function located in `src/core/provider.rs`. It is responsible for initializing services (e.g., creating an `Arc<ApiClient>`) and providing them to the component tree.
- **Context Injection**: Services are injected using `provide_context` at the root of the application (`src/app.rs`).
- **Consumption**: Components access services using type-safe helper functions (e.g., `use_api()`), which internally call `use_context`.

### Advantages
- **Testability**: By providing different implementations via context, services can be easily swapped with mocks during testing.
- **Decoupling**: Components depend on a service interface rather than a concrete implementation.
- **Lifecycle Management**: Services are initialized once at application startup and exist for the lifetime of the application.

## Global State Management

The project manages application-wide state (user authentication, global notifications, theme) using individual signal contexts.

- **Pattern**: Each state concern (user name, authentication, notifications) is provided as its own `RwSignal` via context.
- **Provision**: Signals are provided via `provide_context` in the `App` component through `provide_global_state()`.
- **Reactivity**: Components subscribe only to the specific signals they need, avoiding unnecessary re-renders.

## Layout Composition

The project employs a composition-based layout strategy for maximum flexibility.

- **`MainLayout`**: Acts as a generic shell. Instead of being coupled to specific routes, it accepts `children: Children`.
- **Routing Decoupling**: The `App` component uses `<Routes>` to match URL paths and render the appropriate view inside `MainLayout`.
- **Flexibility**: This allows for easy creation of different layout types without duplicating the core structure.

## Utility Class Merging

The project includes a custom `cn` utility in `src/shared/utils/cn.rs` that combines `clsx` and `tailwind-merge` functionality:

- **Conditional Classes**: Accepts a slice of `Option<&str>`, filtering out `None` values.
- **Conflict Resolution**: Merges conflicting Tailwind classes using a category-based system, ensuring the last class in a conflict group wins.
- **Modifier Support**: Handles responsive and state modifiers (`hover:`, `md:`, `dark:`) as separate scopes.
