# 🚀 Leptos Tailwind Boilerplate

A modern, high-performance frontend boilerplate built with **Leptos 0.7**, **Tailwind CSS v4**, and **Trunk**. This project is designed as a foundation for building scalable, reactive web applications with a strong focus on reusable UI primitives and custom hooks.

## 🛠 Tech Stack

- **Framework**: [Leptos 0.7](https://leptos.dev/) (Client-Side Rendering)
- **Styling**: [Tailwind CSS v4](https://tailwindcss.com/)
- **Bundler**: [Trunk](https://trunkrs.dev/)
- **Runtime/Tooling**: [Bun](https://bun.sh/)
- **Language**: Rust (Nightly recommended)

## 🚀 Getting Started

### Prerequisites

1. **Rust Nightly**:
   ```bash
   rustup toolchain install nightly
   rustup default nightly
   rustup target add wasm32-unknown-unknown
   ```

2. **Trunk**:
   ```bash
   cargo install trunk
   ```

3. **Bun**:
   Install via [bun.sh](https://bun.sh)

### Installation & Development

1. **Clone the repository**:
   ```bash
   git clone <repo-url>
   cd rigortemplate-rust-leptos-tailwind-02
   ```

2. **Install JS dependencies**:
   ```bash
   bun install
   ```

3. **Run the development server**:
   ```bash
   bun run dev
   ```
   This command concurrently runs the Tailwind CSS watcher and the Trunk development server.

4. **Open the app**:
   Navigate to [http://localhost:8080](http://localhost:8080) in your browser.

## 🏗 Project Architecture

The project is organized into a clean separation between application logic and reusable primitives:

### `src/stdlib/` (The Foundation)
The standard library contains framework-agnostic utilities:
- **`ui/`**: A collection of atomic UI components (Button, Badge, Modal, etc.) and a `registry` for the catalog.
- **`hooks/`**: A rich set of reusable reactive hooks (e.g., `use_local_storage`, `use_window_size`, `use_debounce`).
- **`services/`**: API clients and error handling logic.

### `src/components/` (The Application)
- **`layout/`**: Global shell components (Header, Sidebar, MainLayout).
- **`views/`**: Page-level components.
- **`shared/`**: Application-specific shared components.

## 📚 Component & Hooks Catalog

This project includes a built-in interactive documentation system:
- **Component Catalog**: Browse all UI primitives, see them in action, and copy the full Rust source code.
- **Hooks Gallery**: Explore the available reactive hooks with usage examples and descriptions.

## 🛠 Configuration

- **Tailwind**: Configured in `tailwind.config.js`.
- **Builds**: Managed via `package.json` scripts and `Trunk.toml`.
- **Routing**: Currently uses a signal-based manual router in `app.rs` and `main_layout.rs`.

## 📝 License
MIT
