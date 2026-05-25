# Architecture

This project follows a modular architecture designed for scalability and maintainability in Leptos applications.

## Core Structure

- `src/app.rs`: The root of the application. Handles global context provision (state, theme, services) and top-level routing using leptos_router.
- `src/ui/layout`: Contains reusable layout components like `MainLayout`, `Sidebar`, and `Header`.
- `src/ui/views`: Feature-specific views that are injected into the layout.
- `src/hooks`: Custom reactive hooks encapsulating common browser and state logic.
- `src/core`: Core infrastructure including API client, error types, state providers, and service initialization.
- `src/shared`: Shared utilities including theme management, local storage helpers, and the custom `cn` class merging utility.

## State Management

The project uses a signal-per-concern pattern based on Leptos contexts:
- **User/Notifications State**: Individual `RwSignal` values are provided via `provide_global_state` and accessed using dedicated functions (`use_user_name`, `use_is_authenticated`, `use_notifications`).
- **Theme State**: A dedicated `ThemeContext` handles light/dark mode switching and persists the preference in `localStorage`.

## Service Initialization

To avoid prop-drilling and manage external dependencies:
- **`provide_services()`**: Located in `src/core/provider.rs`, this function initializes services (like `ApiClient`) and provides them via context.
- **Dependency Injection**: Services are wrapped in `Arc` to ensure they are `Send + Sync`, allowing them to be shared across the component tree.
- **Consumption**: Components use helper functions like `use_api()` to retrieve the service instance from the context.

## Routing

The project uses `leptos_router` for URL-based routing:
- **`<Router>`**: Wraps the application in `src/app.rs`.
- **`<Routes>` + `<Route>`**: Maps URL paths to view components (e.g., `/accordion` -> `AccordionView`).
- **Navigation**: Sidebar items use `<a>` tags with `use_navigate()` for client-side navigation with full browser history support.

## UI Library

The `src/ui/shared` directory contains a set of pure, reusable components built with Tailwind CSS. These components are designed to be agnostic of application state and are configured via props.
