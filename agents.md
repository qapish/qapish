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

## Containerization & Runtime Policy

- **Runtime:** Rootless **Podman** only. **No Docker** anywhere (no `docker` CLI, no Docker Desktop, no docker-compose).
- **Files:** Use `Containerfile` (Dockerfile syntax is fine). Do not add Docker-specific files or README snippets.
- **Orchestration:** Prefer **Quadlet** for systemd integration. Keep unit files under `orchestration/quadlet/` and document runtime env in `.env.example` (if needed).
- **Build & Push:** Use `podman build` / `podman push`. Example:
  - `podman build -t <registry>/<org>/qapish-api:<tag> -f Containerfile .`
  - `podman push <registry>/<org>/qapish-api:<tag>`
- **Networking:** Default to rootless network (slirp4netns/pasta). Publish ports via Quadlet `PublishPort=` or `-p host:container`. Avoid `--net=host`.
- **Security:** No `--privileged`. Avoid `--cap-add` unless strictly required; justify in the PR. Mount volumes read-only by default; least-privilege FS and env.
- **Ports:** Prefer high host ports (≥1024). If low ports are needed, terminate TLS at the host or a reverse proxy and forward to the container.
- **CI/CD:** Use Podman in pipelines. Do not add scripts/targets that assume `docker` exists (and do not alias `docker` → `podman` in repo scripts).
- **Alternatives:** If a third-party provides Docker/Docker-Compose examples, translate them to Podman/Quadlet and store under `orchestration/quadlet/`. Any rationale belongs in `.docs/*.md`, not inline.


## Dependency Versioning Policy

- **Use the latest stable release** of any new dependency. No pre-releases (`-alpha`, `-beta`, `-rc`), no unreleased `git` refs, and no yanked/abandoned crates.
- **SemVer constraints:**
  - Prefer caret ranges that track the latest compatible **stable**:
    - ✅ `leptos = "0.6"` (gets latest 0.6.x)
    - ✅ `sqlx = "0.7"`
  - Avoid pinning to exact patch versions unless required for a hotfix:
    - 🚫 `leptos = "=0.6.4"` (too brittle)
  - Avoid overly broad ranges or wildcards:
    - 🚫 `leptos = "*"`, `>=0.6`
- **Rust channel:** Everything must build on **stable** Rust (current toolchain). If a crate needs nightly/unstable features, pick a different crate or redesign.
- **Crates.io only:** New deps must come from crates.io. No `git = "..."` unless there is a critical upstream fix **and** you open an issue to remove it. Document such exceptions in `.docs/deps-exceptions.md`.
- **Feature hygiene:** Enable only the features you use. Prefer `rustls` over `native-tls` where applicable. Keep default features off when they drag in unnecessary deps.
- **Validation checklist (required in PR description):**
  - Ran `cargo update -p <new-dep>` and `cargo check` at workspace root.
  - Verified `cargo test -q` (if tests exist) and `cd web && trunk build` (for WASM) succeed.
  - Checked for duplicate/conflicting versions: `cargo tree -d`.
  - No MSRV bumps beyond stable without prior approval.

> Goal: avoid yak shaving and flaky builds. If you must deviate (pin exact version, use a fork, etc.), explain why in `.docs/deps-exceptions.md` and propose a path back to a normal stable crates.io release.


## Platform & Packaging Policy (Fedora-first)

- **OS baseline:** Fedora (Workstation/Server/CoreOS). This project develops and deploys on Fedora because it tracks recent ecosystem changes reliably (e.g., **liboqs** packaging) and stays easy to update via the system package manager.
- **Package manager:** **dnf/rpm** only. **Do not** include `apt`, `apt-get`, `deb` packaging, or Ubuntu/Debian instructions in docs, scripts, or examples.
- **Docs & scripts:** All setup guides, snippets, and automation **must** target Fedora with `dnf`/`rpm` (and `systemd`/`podman` where relevant). If upstream docs show `apt`, translate them to `dnf`. Do not leave mixed examples.
- **Containers:** Base images should be Fedora/ubi/rpm-based. Avoid Debian/Ubuntu bases. Keep runtime parity with host expectations (rootless Podman).
- **Repositories:** Prefer official Fedora repos; COPR is acceptable when necessary and should be documented clearly. Avoid `curl | bash` style installers; package or pin via rpm when possible.
- **Exceptions:** None by default. If a dependency is temporarily unavailable on Fedora, open an issue proposing a Fedora-compatible path (COPR/spec, alternative crate, or vendoring) and document the temporary workaround in `.docs/*.md`. Do **not** merge `apt`/Debian instructions unless they are in addition or complimentary to comprehensive Fedora instructions.
