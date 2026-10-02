# Project Guidance

## Stack

Tauri 2 with a Rust core and React/TypeScript/Vite/Tailwind v4 frontend. Durable state and Jira access belong in Rust; React owns presentation and transient UI state. Follow `.specify/memory/constitution.md` and `specs/001-jira-personal-desktop/`.

## Commands

```bash
pnpm install
pnpm dev
pnpm tauri dev
pnpm format:check
pnpm lint
pnpm typecheck
pnpm test
pnpm build
cargo fmt --manifest-path src-tauri/Cargo.toml --all -- --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml --all-features
pnpm tauri build --no-bundle
```

Run `pnpm check`, the frontend build, Rust checks, and relevant platform build before completing a milestone.

## Conventions

- Use named TypeScript exports, strict types, explicit public return types, and `@/` imports.
- Keep React code feature-oriented and global state minimal.
- Use semantic Tailwind tokens from `src/styles/globals.css`; do not add `tailwind.config.ts`.
- Keep Tauri commands thin; application logic belongs under `src-tauri/src/application/`.
- Never expose Jira credentials or raw authorization data to React, logs, SQLite, errors, or fixtures.
- Add tests before behavior changes where infrastructure exists.
