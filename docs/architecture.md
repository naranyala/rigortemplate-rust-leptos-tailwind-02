# Architecture

This project follows a modular architecture designed for scalability and maintainability in Leptos applications.

## Core Structure

- `src/app.rs`: The root of the application. Handles global context provision (state, theme, services) and top-level routing.
- `src/components/layout`: Contains reusable layout components like `MainLayout`, `Sidebar`, and `Header`.
- `src/components/views`: Feature-specific views that are injected into the layout.
- `src/stdlib`: The "Standard Library" of the project, containing reusable logic independent of the specific application domain.

## State Management

The project uses a centralized state management pattern based on Leptos contexts:
- **Global State**: Managed via `provide_global_state` and accessed using `use_global_state`. It uses `RwSignal` for reactive updates.
- **Theme State**: A dedicated `ThemeContext` handles light/dark mode switching and persists the preference in `localStorage`.

## Service Provider Pattern

To avoid prop-drilling and manage external dependencies, the project implements a Service Provider pattern:
- **ServiceProvider**: Located in `src/stdlib/services/provider.rs`, this class initializes global services (like `ApiClient`) and provides them via context.
- **Dependency Injection**: Services are wrapped in `Arc` to ensure they are `Send + Sync`, allowing them to be shared across the application.
- **Consumption**: Components use helper functions like `use_api()` to retrieve the service instance from the context.

## UI Library

The `src/stdlib/ui` directory contains a set of pure, reusable components built with Tailwind CSS. These components are designed to be agnostic of the application state and are configured via props.
