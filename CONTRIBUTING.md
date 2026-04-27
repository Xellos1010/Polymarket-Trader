# Contributing

## Workflow
- Create short-lived branches: `feat/<scope>-<title>` or `fix/<scope>-<issue>`.
- Use Conventional Commits (`feat`, `fix`, `docs`, `test`, `chore`, `ci`, `perf`, `refactor`).
- Keep PRs focused and small enough for effective review.

Install local hooks once per clone:

```bash
./scripts/install_git_hooks.sh
```

## Local Quality Gates

Run the canonical local-first ladder before pushing changes that affect runtime behavior, replay or paper workflows, or merge readiness:

```bash
./scripts/local_validation_ladder.sh
```

Reference guide:
- `docs/LOCAL_VALIDATION.md`

At minimum, every change should still satisfy the core Rust checks:

```bash
cargo fmt --all
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
```

## Definition of Done
- Acceptance criteria pass.
- Tests added or updated.
- Docs updated (README/docs/adr as needed).
- `docs/SDLC_CHECKLIST.md` updated when setup or process changes.

## Security
- Never commit secrets.
- Use `config/config.toml` locally and keep secrets out of VCS.
- Prefer environment injection in deployment runtime.
- Do not enable live mode as part of local validation.