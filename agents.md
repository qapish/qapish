# Qapish — Agents Guide

This document tells human and AI contributors how to work inside this repo.

## Baseline

- **Frontend:** Vanilla **Leptos (0.6)** + **Leptonic** for layout/style. No alternate UI frameworks.
- **Backend:** Axum API serving JSON and the SPA bundle.
- **Build:** `cargo` stable only. **No nightly**, no unstable features.
- **Bundler:** `trunk` for the `web/` crate (CSR → WASM).
- **Workspace:** Keep **application/library logic in libs** (e.g., `infra`, `ai-common`, or new libs).
  `api/` = API routing/serialization only. `web/` = UI only.

## Golden Rules

1. **Stay in Rust + Leptos + Leptonic.** Don’t add React, Yew, or other UI stacks.
2. **Stable Rust only.** If you need nightly, redesign.
3. **No repo-wide “essay” commits.** If you must write lengthy rationale, put it in **`.docs/*.md`**.
4. **No business logic in `api/` or `web/`.** Move it into a lib crate and call it from API/UI.
5. **Small, composable crates.** New domain areas → add a new lib crate rather than bloating existing ones.

## Repository Layout Policy

- `web/` — Leptos CSR app.
  - Renders views, calls `/api/*`.
  - **Only** view/state glue; **no** domain logic.
  - WASM-only deps guarded with `#[cfg(target_arch = "wasm32")]`.

- `api/` — Axum server.
  - Defines routes, request/response types, auth wiring, error mapping.
  - Delegates to lib crates for all logic.
  - Serves static SPA from `web/dist` via `ServeDir` + SPA fallback.

- `ai-common/` (and peers under `/libs` if you add that folder) — shared models/traits.
  - Domain types, interfaces, serialization types.

- `infra/` — concrete implementations (provisioning, orchestration, adapters).
  - May depend on OS/network crates; keep side effects here.

> If a change touches view + API + logic, split it: **lib first**, then thin API route, then UI call.

## Dependencies & Style

- Prefer well-maintained, popular crates. Avoid experimental/unmaintained deps.
- Error handling: `thiserror` in libs, map to HTTP in `api`. No `.unwrap()`/`.expect()` in non-test code.
- Logging via `tracing`; no `println!` in library code.
- Feature flags: use to separate drivers (e.g., `infra = { features = ["systemd", "k8s"] }`).

## Frontend (Leptos + Leptonic) Conventions

- Wrap app in Leptonic `Root`/theme provider.
- Use Leptonic components for layout (Stack/Grid/AppBar/Card) and controls (Button, Input, etc.).
- Styling comes from Leptonic theme; avoid mixing in other CSS frameworks unless documented in `.docs/`.
- Keep API URLs relative (same origin). If deploying SPA separately, use a small proxy (or enable CORS in API).

## Backend (Axum) Conventions

- One router crate (`api/`). Keep handlers thin:
  - Parse request → call lib → map result to HTTP.
- Health at `/api/health`. Prefer `/api/v1/*` for versioned endpoints.
- Serve SPA: `ServeDir("./web/dist").fallback(ServeFile("./web/dist/index.html"))`.

## Libraries: How to Add Feature Logic

1. **Create/extend a lib crate** (e.g., `cargo new --lib compute-planner`).
2. Define public **traits/types** in `ai-common` or the new crate.
3. Implement in `infra` (or a new `*-driver` crate) to keep effects isolated.
4. Call the lib from `api` handlers; expose minimal DTOs.
5. Call the API from `web` with simple view state.

## Build & Run (recap)

- Frontend bundle:
  - `cd web && trunk build --release`
- Backend:
  - `cargo build -p api --release`
  - `./target/release/api` (serves `/` and `/api/*`)

## PR Checklist

- [ ] New logic landed in a **lib**, not in `api/` or `web/`.
- [ ] Stable Rust only; no nightly flags or unstable features.
- [ ] Leptonic components used for UI; no extra UI frameworks.
- [ ] Tests or examples for new library surfaces.
- [ ] If long-form explanation needed → **`.docs/*.md`**; not in code comments or PR body alone.
