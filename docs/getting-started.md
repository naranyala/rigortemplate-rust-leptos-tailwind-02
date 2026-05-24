# Getting Started

This guide will help you get the Leptos Tailwind Boilerplate up and running on your local machine.

## Prerequisites

Before you begin, ensure you have the following installed:
- Rust (latest stable version)
- WASM target: `rustup target add wasm32-unknown-unknown`
- Trunk: `cargo install trunk`
- Bun (recommended for managing Tailwind and other JS tools)

## Installation

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd rigortemplate-rust-leptos-tailwind-02
   ```

2. Install JS dependencies:
   ```bash
   bun install
   ```

## Running in Development Mode

The easiest way to start the development server is using the provided `run.sh` script or the `npm run dev` command.

### Using run.sh
```bash
chmod +x run.sh
./run.sh
```

### Using npm/bun
```bash
bun run dev
```

The application will be available at `http://localhost:8080` (or the next available port if 8080 is occupied).

## Building for Production

To create an optimized production build:
```bash
bun run build
```
The output will be located in the `dist` directory.
