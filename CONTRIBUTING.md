# Contributing

Documentation index (runbooks, ADRs, audit): [docs/README.md](docs/README.md).  
Roadmap: [ROADMAP.md](ROADMAP.md).

This project follows the [Contributor Covenant](CODE_OF_CONDUCT.md). Participation is conditioned on respectful collaboration.

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
- Docs updated (README, `docs/`, ADRs, `ROADMAP.md` when priorities shift).
- `docs/SDLC_CHECKLIST.md` updated when setup or process changes.
- Conduct and security expectations in [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) and [SECURITY.md](SECURITY.md) remain satisfied.

## Security

- Report vulnerabilities privately per [SECURITY.md](SECURITY.md).
- Never commit secrets.
- Use `config/config.toml` locally and keep secrets out of VCS.
- Prefer environment injection in deployment runtime.
- Do not enable live mode as part of local validation.