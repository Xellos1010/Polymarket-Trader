#!/usr/bin/env python3
"""Export non-generated repository context into a compact single text bundle.

The bundle is intended for external AI prompt iteration.
"""

from __future__ import annotations

import argparse
import datetime as dt
import fnmatch
import json
import pathlib
import subprocess
from dataclasses import dataclass
from typing import Dict, Iterable, List, Optional, Sequence, Set, Tuple


DEFAULT_EXCLUDE_GLOBS = [
    ".git/**",
    "target/**",
    "node_modules/**",
    "dist/**",
    "build/**",
    "coverage/**",
    ".venv/**",
    "venv/**",
    "__pycache__/**",
    ".pytest_cache/**",
    ".mypy_cache/**",
    ".next/**",
    ".turbo/**",
    "artifacts/**",
    "data/output/**",
    "data/strategy_lab/**",
    "data/replay/**",
    "*.pyc",
    "*.pyo",
    "*.png",
    "*.jpg",
    "*.jpeg",
    "*.gif",
    "*.pdf",
    "*.zip",
    "*.gz",
    "*.tar",
    "*.7z",
    "*.woff",
    "*.woff2",
    "*.ttf",
    "*.otf",
    "*.db",
    "*.sqlite",
]

DEFAULT_INCLUDE_GLOBS = [
    "AGENTS.md",
    ".cursor/rules/**",
    "README.md",
    "CONTRIBUTING.md",
    "DEPLOYMENT.md",
    "Cargo.toml",
    "Cargo.lock",
    "crates/**",
    "scripts/**",
    "tools/**",
    "config/**",
    "schemas/**",
    "docs/**",
    "pine-scripts/**",
    ".env.example",
    ".gitignore",
    ".github/workflows/**",
]


@dataclass
class BundleConfig:
    include_globs: List[str]
    exclude_globs: List[str]
    max_file_bytes: int
    max_total_bytes: int
    collapse_blank_runs: bool


def read_json(path: pathlib.Path) -> dict:
    return json.loads(path.read_text(encoding="utf-8"))


def load_config(path: Optional[pathlib.Path]) -> BundleConfig:
    include_globs = list(DEFAULT_INCLUDE_GLOBS)
    exclude_globs = list(DEFAULT_EXCLUDE_GLOBS)
    max_file_bytes = 300_000
    max_total_bytes = 10_000_000
    collapse_blank_runs = True

    if path and path.exists():
        raw = read_json(path)
        include_globs = list(raw.get("include_globs", include_globs))
        exclude_globs = list(raw.get("exclude_globs", exclude_globs))
        max_file_bytes = int(raw.get("max_file_bytes", max_file_bytes))
        max_total_bytes = int(raw.get("max_total_bytes", max_total_bytes))
        collapse_blank_runs = bool(raw.get("collapse_blank_runs", collapse_blank_runs))

    return BundleConfig(
        include_globs=include_globs,
        exclude_globs=exclude_globs,
        max_file_bytes=max_file_bytes,
        max_total_bytes=max_total_bytes,
        collapse_blank_runs=collapse_blank_runs,
    )


def run_git(root: pathlib.Path, args: Sequence[str]) -> Optional[bytes]:
    cmd = ["git", "-C", str(root), *args]
    try:
        out = subprocess.check_output(cmd, stderr=subprocess.DEVNULL)
        return out
    except (subprocess.CalledProcessError, FileNotFoundError):
        return None


def listed_files_from_git(root: pathlib.Path, include_untracked: bool) -> Optional[Set[str]]:
    tracked_raw = run_git(root, ["ls-files", "-z"])
    if tracked_raw is None:
        return None

    def split_null(data: bytes) -> List[str]:
        return [x for x in data.decode("utf-8", errors="ignore").split("\0") if x]

    files: Set[str] = set(split_null(tracked_raw))
    if include_untracked:
        others_raw = run_git(root, ["ls-files", "--others", "--exclude-standard", "-z"])
        if others_raw is not None:
            files.update(split_null(others_raw))
    return files


def listed_files_walk(root: pathlib.Path) -> Set[str]:
    files: Set[str] = set()
    for path in root.rglob("*"):
        if path.is_file():
            files.add(path.relative_to(root).as_posix())
    return files


def matches_any(path: str, globs: Sequence[str]) -> bool:
    return any(fnmatch.fnmatch(path, pat) for pat in globs)


def looks_binary(data: bytes) -> bool:
    if b"\x00" in data:
        return True
    # Heuristic: many control chars usually means non-text/binary.
    if not data:
        return False
    control = sum(1 for b in data[:4096] if b < 9 or (13 < b < 32))
    return control > 32


def minify_text(text: str, collapse_blank_runs: bool) -> str:
    lines = [line.rstrip() for line in text.replace("\r\n", "\n").replace("\r", "\n").split("\n")]
    if not collapse_blank_runs:
        return "\n".join(lines).strip() + "\n"

    out: List[str] = []
    blank_run = 0
    for line in lines:
        if line.strip() == "":
            blank_run += 1
            if blank_run <= 1:
                out.append("")
            continue
        blank_run = 0
        out.append(line)

    return "\n".join(out).strip() + "\n"


def should_include(rel_path: str, cfg: BundleConfig) -> bool:
    if matches_any(rel_path, cfg.exclude_globs):
        return False
    if not cfg.include_globs:
        return True
    return matches_any(rel_path, cfg.include_globs)


def collect_bundle(root: pathlib.Path, cfg: BundleConfig, include_untracked: bool) -> Tuple[List[Tuple[str, str]], List[str]]:
    files = listed_files_from_git(root, include_untracked=include_untracked)
    if files is None:
        files = listed_files_walk(root)

    included: List[Tuple[str, str]] = []
    skipped: List[str] = []
    total_bytes = 0

    for rel_path in sorted(files):
        if not should_include(rel_path, cfg):
            continue

        abs_path = root / rel_path
        if not abs_path.exists() or not abs_path.is_file():
            continue

        size = abs_path.stat().st_size
        if size > cfg.max_file_bytes:
            skipped.append(f"{rel_path} (file too large: {size} bytes)")
            continue

        data = abs_path.read_bytes()
        if looks_binary(data):
            skipped.append(f"{rel_path} (binary)")
            continue

        try:
            text = data.decode("utf-8")
        except UnicodeDecodeError:
            skipped.append(f"{rel_path} (non-utf8)")
            continue

        minified = minify_text(text, cfg.collapse_blank_runs)
        block_bytes = len(minified.encode("utf-8"))
        if total_bytes + block_bytes > cfg.max_total_bytes:
            skipped.append(f"{rel_path} (bundle size cap reached)")
            continue

        total_bytes += block_bytes
        included.append((rel_path, minified))

    return included, skipped


def write_bundle(
    out_path: pathlib.Path,
    root: pathlib.Path,
    cfg: BundleConfig,
    files: Sequence[Tuple[str, str]],
    skipped: Sequence[str],
) -> None:
    header = [
        "# PROMPT_BUNDLE v1",
        f"# generated_at_utc={dt.datetime.now(dt.timezone.utc).isoformat()}",
        f"# root={root}",
        f"# included_files={len(files)}",
        f"# skipped_files={len(skipped)}",
        f"# include_globs={json.dumps(cfg.include_globs)}",
        f"# exclude_globs={json.dumps(cfg.exclude_globs)}",
        "",
    ]

    parts = ["\n".join(header)]
    for rel_path, content in files:
        parts.append(f"===== FILE: {rel_path} =====\n{content}===== END FILE =====\n")

    if skipped:
        parts.append("===== SKIPPED FILES =====\n")
        for item in skipped:
            parts.append(f"{item}\n")

    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text("".join(parts), encoding="utf-8")


def parse_args() -> argparse.Namespace:
    p = argparse.ArgumentParser(description="Export prompt bundle for external AI tools")
    p.add_argument("--root", default=".", help="repository root")
    p.add_argument("--out", default="data/output/prompt_bundle.min.txt", help="bundle output file")
    p.add_argument(
        "--config",
        default="config/prompt_bundle.json",
        help="optional bundle config json (falls back to defaults if missing)",
    )
    p.add_argument("--include-untracked", action="store_true", help="include untracked files")
    p.add_argument("--manifest", default=None, help="optional JSON manifest output path")
    return p.parse_args()


def main() -> int:
    args = parse_args()

    root = pathlib.Path(args.root).resolve()
    config_path = pathlib.Path(args.config)
    if not config_path.is_absolute():
        config_path = root / config_path

    cfg = load_config(config_path if config_path.exists() else None)
    files, skipped = collect_bundle(root, cfg, include_untracked=args.include_untracked)

    out_path = pathlib.Path(args.out)
    if not out_path.is_absolute():
        out_path = root / out_path
    write_bundle(out_path, root, cfg, files, skipped)

    manifest = {
        "generated_at_utc": dt.datetime.now(dt.timezone.utc).isoformat(),
        "root": str(root),
        "out": str(out_path),
        "included_files": len(files),
        "skipped_files": len(skipped),
    }

    if args.manifest:
        manifest_path = pathlib.Path(args.manifest)
        if not manifest_path.is_absolute():
            manifest_path = root / manifest_path
        manifest_path.parent.mkdir(parents=True, exist_ok=True)
        manifest_path.write_text(json.dumps(manifest, indent=2), encoding="utf-8")

    print(json.dumps(manifest, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
