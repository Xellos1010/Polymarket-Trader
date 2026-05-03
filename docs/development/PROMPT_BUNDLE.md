# Prompt Bundle Export

Exports a compact single text file containing non-generated project context for external AI tools.

## Files

- Export tool: `tools/export_prompt_bundle.py`
- Wrapper script: `scripts/export_prompt_bundle.sh`
- Config example: `config/prompt_bundle.example.json`
- Config schema: `schemas/prompt_bundle.schema.json`

## Quick Start

```bash
cp config/prompt_bundle.example.json config/prompt_bundle.json
./scripts/export_prompt_bundle.sh
```

Default outputs:
- Bundle: `data/output/prompt_bundle.min.txt`
- Manifest: `data/output/prompt_bundle.manifest.json`

## What gets included

The defaults include:
- instructions/rules (`AGENTS.md`, `.cursor/rules/**`)
- docs, schemas, config
- code (`crates/**`, `tools/**`, `scripts/**`, `pine-scripts/**`)
- workflow files (`.github/workflows/**`)

The defaults exclude generated/cached artifacts:
- `target/**`, `node_modules/**`, `data/output/**`, `data/strategy_lab/**`, caches, binaries, archives.

## Customization

Adjust include/exclude globs and size caps in `config/prompt_bundle.json`.

You can also run directly:

```bash
python3 tools/export_prompt_bundle.py \
  --root . \
  --config config/prompt_bundle.json \
  --out data/output/prompt_bundle.min.txt \
  --include-untracked \
  --manifest data/output/prompt_bundle.manifest.json
```
