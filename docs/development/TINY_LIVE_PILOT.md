# Tiny Live Pilot

## Goal
Run a constrained `$10-$50` live pilot focused on safety and process verification, not income scale.

## Preconditions
1. Live credentials injected via environment variables (`.env.example` names).
2. Config risk caps at or below tiny-pilot thresholds.
3. `preflight-live` passes.

## Pilot Command
```bash
./scripts/tiny_live_pilot.sh config/config.toml 3000
```

## Thresholds Enforced
- `risk.daily_loss_limit_pct <= 0.02`
- `risk.max_notional_per_market <= 5`
- `risk.max_total_open_notional <= 20`
- `risk.max_markets_quoted_simultaneously <= 2`
- `risk.max_unhedged_delta <= 10`

## During Pilot
- Keep dashboard open (`/`), watch `kill_switch`, `daily_pnl`, and `unhedged_delta`.
- If behavior deviates, call `POST /ops/halt` immediately.

## Exit Criteria
- No unexpected auto-halts.
- No unhedged delta breaches.
- Daily loss cap respected.
- Clear post-run notes captured in `docs/SESSION_CONTEXT.md`.
