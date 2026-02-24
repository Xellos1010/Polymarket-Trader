#!/usr/bin/env bash
set -euo pipefail

OUT_DIR="${1:-artifacts}"
mkdir -p "$OUT_DIR"

if cargo cyclonedx --version >/dev/null 2>&1; then
  echo '[sbom] generating CycloneDX SBOM'
  cargo cyclonedx --all-features --format json --override-filename sbom.cdx >/dev/null
  found=0
  while IFS= read -r file; do
    found=1
    crate_name="$(basename "$(dirname "$file")")"
    cp "$file" "$OUT_DIR/${crate_name}.sbom.cdx.json"
    rm -f "$file"
  done < <(find crates -maxdepth 2 -type f -name 'sbom.cdx*.json*' | sort)

  if [[ "$found" -eq 0 ]]; then
    echo '[sbom] warning: cyclonedx finished but no sbom files were found, falling back to cargo metadata'
    cargo metadata --format-version=1 > "$OUT_DIR/sbom.cargo-metadata.json"
  fi
else
  echo '[sbom] cargo-cyclonedx not installed; using cargo metadata fallback'
  cargo metadata --format-version=1 > "$OUT_DIR/sbom.cargo-metadata.json"
fi

echo "[sbom] artifacts in $OUT_DIR"
ls -la "$OUT_DIR"
