# Leptos Tailwind Boilerplate

A modern, high-performance frontend boilerplate built with Leptos, Tailwind CSS, and Trunk. This project is designed as a foundation for building scalable, reactive web applications with a strong focus on reusable UI primitives and custom hooks.

## Tech Stack

- Framework: Leptos (Client-Side Rendering)
- Styling: Tailwind CSS
- Bundler: Trunk
- Runtime/Tooling: Bun
- Language: Rust

## Getting Started

For detailed installation and setup instructions, please refer to the [Getting Started Guide](docs/getting-started.md).

Quick start:
```bash
bun install
bun run dev
```

## Project Architecture

The project is organized into a clean separation between application logic and reusable primitives. For a deep dive into the design patterns used, see the [Architecture Documentation](docs/architecture.md).

### src/hooks/ (The Foundation)
Custom reactive hooks encapsulating common browser and state logic (use_toggle, use_is_mounted, use_clipboard).

### src/core/ (Core Infrastructure)
- api.rs: HTTP client for API communication.
- provider.rs: Service initialization and context provisioning.
- state.rs: Application-wide state signals.
- errors.rs: Error types and user-friendly messages.

### src/ui/ (The Application)
- layout/: Global shell components (Header, Sidebar, MainLayout).
- views/: Page-level components.
- shared/: Application-specific shared components.

### src/shared/ (Shared Utilities)
- theme.rs: Light/Dark theme management with localStorage persistence.
- storage.rs: Reactive wrappers for browser local storage.
- utils/cn.rs: Custom clsx + tailwind-merge utility for conditional class merging.

## Documentation

Detailed documentation is available:
- [Architecture](docs/architecture.md)
- [Getting Started](docs/getting-started.md)
- [UI Library Reference](docs/ui-library.md)
- [Hooks Reference](docs/hooks.md)
- [Design Patterns](docs/patterns.md)
- [Testing Strategy](docs/testing.md)

## Configuration

- Tailwind: Configured in input.css and package.json scripts.
- Builds: Managed via package.json scripts and Trunk.
- Routing: URL-based routing using leptos_router.

## License

MIT
