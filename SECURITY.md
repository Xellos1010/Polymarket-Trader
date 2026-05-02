# Security policy

## Reporting a vulnerability

**Please do not open a public GitHub issue** for undisclosed security vulnerabilities.

Preferred:

1. Open a **draft GitHub Security Advisory** for this repository (**Security** → **Advisories** → **Report a vulnerability**), or  
2. If you cannot use GitHub advisories, contact the maintainers through the same private channel you already use for this repo (for example organization Slack or email agreed out-of-band).

Include:

- A short description of the issue and its impact  
- Steps to reproduce (proof-of-concept, commit range, or file paths)  
- Any suggested fix or mitigation you have in mind  

We aim to acknowledge receipt within **7 business days** and to coordinate disclosure and patch timing with you.

## Scope

In scope: this repository’s code, scripts, default configurations **as shipped in git** (excluding operator-owned `config/config.toml` and other ignored secrets), and documented operator HTTP surfaces when used as intended.

Out of scope: third-party exchange or wallet policies, social engineering against individuals, or issues in dependencies unless they affect this project in a exploitable way (still report dependency issues; we may route to upstream).

## Secure development expectations

- Follow [CONTRIBUTING.md](CONTRIBUTING.md): no secrets in commits, prefer environment injection for credentials.  
- Run `cargo audit` and the local validation ladder before proposing changes that touch auth, signing, or network surfaces.

## Supported versions

Security fixes are applied to the **default branch** (`main`). Tags and releases may trail; check release notes when upgrading.

## GitHub repository settings (automation)

On `github.com/Xellos1010/Polymarket-Trader`, the following are enabled for coordinated disclosure and supply-chain signal: **private vulnerability reporting**, **Dependabot / dependency vulnerability alerts**, **secret scanning**, and **secret scanning push protection**. Reconcile these under **Settings → Code security and analysis** if you fork or transfer the repository.
