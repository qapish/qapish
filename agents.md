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


## Code Comments & Justification Policy

- **Keep comments functional, not opinionated.** Comments exist to clarify *what the code does*, *how to use it*, and *any invariants/safety contracts*. Do **not** write essays justifying design or technology choices inline.
- **No rationales/diatribes in code.** If you feel compelled to record the “why,” put it in **`.docs/<topic>.md`** (gitignored) and reference it briefly in the PR description. Avoid polluting the codebase with subjective commentary or history.
- **Rustdoc is for APIs.** Use `///` to document public types/functions with purpose, inputs/outputs, errors, invariants, and examples. Keep it concise and factual.
- **Allowed short comments:** brief clarity, invariants, and safety notes.
  - ✅ `// SAFETY: buffer is initialized; length checked above`
  - ✅ `// TODO(#123): replace temp file with in-mem store`
  - ✅ `// Precondition: id is UUIDv4`
- **Not allowed:** technology rants, personal anecdotes, competitive takes, or historical justifications.
  - ❌ `// We chose tokio because async-std is slower in our tests…`
  - ❌ `// In 2021 we tried Nginx and it was a mess, so now…`
  - ❌ `// I prefer this pattern because it feels cleaner`
- **Architecture/design notes:** put ADR-style writeups in `.docs/*.md` if truly necessary. The code should remain clean and free of long-form explanations.


## Privilege & Debugging Policy (No `sudo`)

- **Absolutely no `sudo` in debugging or agent runs.** Agentic debuggers/tools must **not** attempt to execute any command requiring elevated privileges on the workstation.
- **Do not auto-elevate.** No `sudo`, no `pkexec`, no `su`, no setuid helpers, no prompts for passwords. If a task *would* need root, stop and propose a rootless alternative.
- **Write scripts to be rootless by default.**
  - Provide `--dry-run` and `--print` modes that show intended actions without executing them.
  - If privileged steps are unavoidable, **fail fast** with a clear message (e.g., “requires root; provide a rootless equivalent or run on a CI/ops host”).
  - Detect privilege safely and exit:
    ```bash
    if [ "${EUID:-$(id -u)}" -ne 0 ]; then
      echo "This step requires root. Exiting per policy."; exit 1
    fi
    ```
- **Use rootless alternatives:**
  - System services → `systemd --user` + Quadlet (rootless Podman).
  - Containers → **rootless Podman** (`podman` only, never Docker).
  - Networking/ports → high ports (≥1024) or reverse proxy front-ends rather than privileged binds.
  - File ops → user-writable paths in `$HOME`, project workspace, or XDG dirs.
- **Deployment & provisioning scripts:**
  - Target CI/ops hosts or documented admin procedures—not developer workstations.
  - Keep privileged instructions in `.docs/*.md` (gitignored) and mark them as **operator-only**.
- **PRs will be rejected** if they contain agent workflows or scripts that attempt to run with elevated privileges on developer machines.


## Clarification Policy (Ask Before You Assume)

- **If guidance is incomplete, ambiguous, or self-contradictory, ask for clarification _before_ implementing.**
- **Do not invent missing requirements** or proceed on major assumptions that can’t be trivially reversed.
- **Surface uncertainties explicitly** and propose 1–2 reasonable options rather than guessing.

### When to ask
- Key details are missing (schemas, API contracts, error handling, auth model, deployment target).
- Conflicting instructions (e.g., “SSR” vs “CSR”, or “no DB” but migrations requested).
- Scope creep or unclear acceptance criteria.
- Security, data retention, or privacy implications are unspecified.

### How to ask (concise template)
> **Clarification needed:** _brief summary of the gap_
> **My default assumption if unblocked:** _Option A_
> **Alternatives:** _Option B (tradeoff)_, _Option C (tradeoff)_
> **Impact of choosing wrong:** _rollback complexity / risk_

### Proceeding without a reply
- If truly blocked, pause and request clarification.
- If partially blocked, implement the **minimal, reversible** path and clearly mark TODOs with a short note to revisit.

### Anti-patterns
- Writing code that bakes in assumptions without review.
- Long speculative commentary in code; keep rationale in the PR description or `.docs/*.md` if needed.


## Build Quality (Must Compile; Prefer No Warnings)

- **Completed contributions must compile** at the workspace root on **stable** Rust:
  - `cargo build --workspace --all-targets --release` **succeeds**.
  - The web crate also builds to WASM: `cd web && trunk build --release`.
- **Prefer zero warnings.** Strive for a clean build. If a warning is truly unavoidable:
  - Scope any `#[allow(...)]` as narrowly as possible and include a 1-line reason.
  - Do **not** suppress warnings repo-wide to hide issues.
- **Clippy & fmt (recommended):**
  - `cargo clippy --workspace --all-targets -- -D warnings` (aim to pass)
  - `cargo fmt --all --check`
- **Tests (when present) must pass:** `cargo test --workspace -q`.
- **Third-party warnings:** Tolerated if they originate in dependencies; **our code** should be warnings-free.
- **CI:** Do not mark a PR “complete” if any required build/test job is red.
- **Reproducibility:** No environment-specific hacks. Default features should build without local secrets or root privileges.


## Acknowledgement of This Guide

Before starting work or before marking a contribution complete, acknowledge you’ve read and understood this document.
