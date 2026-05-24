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

### src/stdlib/ (The Foundation)
The standard library contains reusable utilities:
- ui/: A collection of atomic UI components and a registry for discovery.
- hooks/: A set of reusable reactive hooks.
- services/: API clients and a global service provider pattern.

### src/components/ (The Application)
- layout/: Global shell components (Header, Sidebar, MainLayout).
- views/: Page-level components.
- shared/: Application-specific shared components.

## Documentation

Detailed documentation for the built-in library is available:
- [UI Library Reference](docs/ui-library.md)
- [Hooks Reference](docs/hooks.md)

## Configuration

- Tailwind: Configured in tailwind.config.js.
- Builds: Managed via package.json scripts and Trunk.toml.
- Routing: Signal-based routing implemented in app.rs.

## License

MIT
