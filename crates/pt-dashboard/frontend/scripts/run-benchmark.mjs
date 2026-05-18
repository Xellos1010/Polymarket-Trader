import { mkdir } from "node:fs/promises";
import { dirname, join, resolve } from "node:path";
import { spawnSync } from "node:child_process";

function parseArgs(argv) {
  const args = { outDir: null };
  for (let i = 0; i < argv.length; i += 1) {
    const value = argv[i];
    if (value === "--out-dir") {
      args.outDir = argv[i + 1] ?? null;
      i += 1;
    }
  }
  return args;
}

const args = parseArgs(process.argv.slice(2));
const stamp = new Date().toISOString().slice(0, 10);
const outDir = resolve(args.outDir ?? join(process.cwd(), "../../../artifacts/benchmarks", stamp));
await mkdir(outDir, { recursive: true });

const jsonOut = join(outDir, "frontend-benchmark.json");
const mdOut = join(outDir, "frontend-benchmark.md");

const result = spawnSync(
  "pnpm",
  ["exec", "vitest", "run", "src/benchmark.harness.test.tsx"],
  {
    cwd: process.cwd(),
    stdio: "inherit",
    env: {
      ...process.env,
      PT_FRONTEND_BENCH_OUT: jsonOut,
      PT_FRONTEND_BENCH_MD_OUT: mdOut,
    },
  },
);

if (result.status !== 0) {
  process.exit(result.status ?? 1);
}

console.log(jsonOut);
console.log(mdOut);
