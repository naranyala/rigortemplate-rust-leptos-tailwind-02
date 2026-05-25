# Project Roadmap & TODOs

This document tracks the ongoing development and planned improvements for the Leptos Tailwind Boilerplate.

## High Priority (Core Infrastructure)

- [x] Implement Official Routing: Replace the manual AppRoute signal with leptos_router for proper URL synchronization and nested routing.
- [x] Global Theme Provider: Implement a ThemeProvider hook and context to support Light/Dark mode switching.
- [ ] API Integration Layer: Expand core/api.rs to include generic request/response handling and interceptors.
- [x] State Management: Implement a centralized state store using provide_context with RwSignal for complex application state.

## Medium Priority (UI & UX)

- [ ] Expand UI Library: Add the following components to src/ui/shared/:
    - [x] Data Table with sorting and filtering.
    - [x] Tooltips and Popovers.
    - [x] Form Input validation components.
    - [x] Stepper and Progress indicators.
    - [x] Multi-select dropdowns.
- [ ] A11y Audit: Ensure all components follow WAI-ARIA guidelines for accessibility.
- [ ] Responsive Refinement: Optimize the views for mobile devices.

## Low Priority (Developer Experience)

- [ ] Automated Testing Suite:
    - [ ] Implement wasm-bindgen-test for all UI components.
    - [ ] Add unit tests for every hook in src/hooks/mod.rs.
- [ ] CI/CD Pipeline: Setup GitHub Actions for automated linting, type-checking, and build verification.
- [ ] Documentation Site: Generate a static documentation site from CatalogView and Hooks views.
- [ ] Custom Tailwind Plugin: Create a project-specific Tailwind plugin for consistent spacing and color palettes.

## Maintenance

- [ ] Upgrade to latest Leptos patches.
- [ ] Optimize WASM binary size using wasm-opt.
- [ ] Remove old registry.rs reference.
