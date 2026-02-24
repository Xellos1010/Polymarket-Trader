# SDLC Setup Checklist (Mapped to `.cursor/rules`)

This checklist tracks practical compliance against:
- `03-workflow/sdlc-setup.mdc`
- `03-workflow/documentation-standards.mdc`
- `07-apis/api-standards.mdc`
- `08-infrastructure/configuration-standards.mdc`
- `11-security/security-and-secrets-policy.mdc`

## Phase 1: Project Initialization & Planning
- [x] Repository initialized and baseline commit created (`8ed88d5`)
- [x] README with setup and operator endpoints
- [x] License set to MIT in `Cargo.toml`
- [x] `CONTRIBUTING.md` added
- [x] `CODEOWNERS` added
- [x] Acceptance/scope documented across README + deployment docs

## Phase 2: Quality Gates & Standards
- [x] Rust formatting/lint/testing commands documented
- [x] CI quality gates added (`.github/workflows/ci.yml`)
- [x] Strict typed config structures in `pt-core`
- [ ] Pre-commit hooks (optional follow-up)

## Phase 3: Infrastructure & Configuration
- [x] `.env.example` added
- [x] Secrets excluded via `.gitignore`
- [x] Runtime config validation fails fast at boot (`AppConfig::validate`)
- [x] Config schema documented (`schemas/config.schema.json`)

## Phase 4: Testing Strategy
- [x] Unit tests present across core crates
- [x] Config validation tests added
- [ ] Contract tests for OpenAPI schema conformance (follow-up)
- [ ] Mutation testing (deferred)

## Phase 5: Release
- [x] Deployment baseline exists (`DEPLOYMENT.md`, systemd service)
- [x] Live preflight gate in CLI (`preflight-live`)
- [ ] Branch protection/manual approvals (hosted VCS setting)

## Phase 6: Observability & Operations
- [x] Health endpoint + metrics endpoint
- [x] Dashboard with risk/ops controls
- [x] Runbook/context persistence docs
- [ ] External error tracking integration (Sentry/OTel exporter) (follow-up)

## Phase 7: Documentation & Knowledge
- [x] Architecture diagram added (`docs/architecture/system-overview.md`)
- [x] Data/API schemas documented (`docs/data/SCHEMA.md`, `docs/api/dashboard-openapi.yaml`)
- [x] ADR created (`docs/adr/001-rust-first-polymarket-engine.md`)
- [x] Pine tuning/evaluator workflow documented

## Current Gaps / Next Hardening Items
1. Add hosted branch protections and required status checks.
2. Add contract tests that validate `/state/*` responses against OpenAPI schemas.
3. Add pre-commit automation for fmt/clippy/test.
4. Add dependency scanning + SBOM generation in CI.
