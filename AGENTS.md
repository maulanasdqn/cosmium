# AGENTS.md

Guidance for AI coding agents working in this repository. Human-facing docs live in `README.md` and at [cosmium.zod.rs](https://cosmium.zod.rs) (`apps/landing`).

## What this is

Cosmium is a patched Chromium (`patches/`, pinned tag in `VERSION`) plus a Rust workspace that turns JSON fingerprint profiles (`profiles/`) into `--cosmium-*` switches, launches the binary, and drives it over CDP for stealth scraping.

- `apps/cli` — the `cosmium` binary; only wires the logger and `engine`.
- `apps/engine` — library in clean-architecture layers: `domain` → `application` → `infrastructure` → `presentation` (`cli`, `http`). Dependencies point inward only; `domain` must not import from outer layers.
- `.config` — env and logging bootstrap (`cosmium-config`).
- `apps/ui` — React + TanStack + Vite frontend (`npm run build`, `npm run lint` via oxlint).
- `apps/landing` — Astro/Starlight docs site.
- `patches/` — Chromium C++ patches applied in `patches/series` order; `scripts/` holds the numbered build pipeline.

## Hard rules

These are enforced by CI (`.github/workflows/ci.yml`) and lefthook (`lefthook.yml`). Do not work around them.

1. **No comments anywhere.** No `//`, `///`, `//!`, `/* */`, `#` comments, or JSDoc in any source file, in any language. Put explanations in commit messages or in `apps/landing` docs. The only exceptions are functional directives: shebangs, `#[expect(..., reason = "...")]`, and tool pragmas that change behavior.
2. **Max 200 lines per `.rs` file.** Split into submodules instead of growing a file.
3. **Strict clippy, zero warnings.** Workspace lints live in `[workspace.lints]` in the root `Cargo.toml` (`all`, `pedantic`, `nursery`, `cargo` at deny, plus restriction lints such as `unwrap_used`, `expect_used`, `indexing_slicing`, `panic`, `print_stdout`). Every crate opts in with `[lints] workspace = true`.
   - Fix the code; do not silence it. `#[allow]` is forbidden (`allow_attributes`).
   - If a lint truly must be suppressed, use `#[expect(clippy::lint, reason = "...")]` on the smallest possible scope.
   - Prefer `?`, `let ... else`, `.get()`, `try_from`, and `Arc::clone(&x)` over `unwrap`, indexing, and `as` casts.
   - `unwrap`, `expect`, `panic`, and indexing are allowed in tests (`clippy.toml`).
4. **`unsafe` is forbidden** (`unsafe_code = "forbid"`).
5. **Modules use `mod.rs`** (`self_named_module_files`). Do not use `foo.rs` next to `foo/`.
6. **MSRV is 1.85, edition 2024** (`rust-toolchain.toml`, `clippy.toml`).

## Frontend (`apps/ui`)

- Stack: React 19, TanStack Router (file-based, `src/routes`), Query, Form, Store, and Table v9, zod, axios, and shadcn on Base UI.
- **No native HTML elements outside `src/components/ui`.** App code (routes, layout, features) composes components only: shadcn components plus the primitives in `components/ui` (`Stack`, `Grid`, `Heading`, `Text`, `CodeBlock`, `Page`, `PageHeader`, `DataTable`, and the `useAppForm` kit). If something is missing, add a primitive to `components/ui`. `npm run check:native` enforces this.
- API modules live in `src/apis/<feature>/` as `types.ts`, `service.ts`, `hooks.ts`, `index.ts`, with query-key factories and invalidation on mutations. Server state uses TanStack Query, client state uses TanStack Store, and forms use TanStack Form with zod.
- Style: single quotes, no semicolons, kebab-case files, `T`/`I`/`E` prefixes (TypeScript enums are disabled, so use `T` unions), no `any`, no comments, at most 200 lines per file.
- Before finishing UI work, run `npm run check` (tsc, oxlint, the native-element check, prettier) and `npm run build`.
- `cosmium serve` serves `apps/ui/dist` (or `--ui-dir`); in development, `npm run dev` proxies `/api` to `:3000`.

## Commands

```sh
cargo fmt --all
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
lefthook install
```

Run fmt, clippy, and tests before you consider a Rust change done.

## Conventions

- Errors: `anyhow` at application and presentation edges, `thiserror` for domain error types. Add context with `.context(...)` rather than panicking.
- Only `presentation::cli` may print to stdout. Everything else logs through `tracing`.
- Commits follow Conventional Commits with a scope, for example `feat(scrape): ...`, `ci: ...`, `chore(docker): ...`.
- Never edit files under `patches/` by hand. Regenerate them with `scripts/make-patch.sh`.
