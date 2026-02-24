# Contributing

## Workflow
- Create short-lived branches: `feat/<scope>-<title>` or `fix/<scope>-<issue>`.
- Use Conventional Commits (`feat`, `fix`, `docs`, `test`, `chore`, `ci`, `perf`, `refactor`).
- Keep PRs focused and small enough for effective review.

## Local Quality Gates
Run before pushing:

```bash
cargo fmt --all
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
```

## Definition of Done
- Acceptance criteria pass.
- Tests added/updated.
- Docs updated (README/docs/adr as needed).
- `docs/SDLC_CHECKLIST.md` updated when setup/process changes.

## Security
- Never commit secrets.
- Use `config/config.toml` locally and keep secrets out of VCS.
- Prefer environment injection in deployment runtime.
