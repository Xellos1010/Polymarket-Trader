# Deployment (Single EC2 Host)

## 1. Provision
- EC2 `t3.small` (or `t3.micro` for lighter paper mode), Ubuntu 22.04, `us-east-1`.
- Security group:
  - SSH: `22` from your IP.
  - Dashboard: `8080` only from your IP/VPN.
  - TradingView listener (optional): `8090` only from trusted sources.

## 2. Install + bootstrap (first-time host prep)

```bash
chmod +x scripts/bootstrap_ubuntu.sh
./scripts/bootstrap_ubuntu.sh
```

## 3. Configure secrets
Prefer environment injection for secrets (`.env.example` variables), then keep
`config/config.toml` free of private key material where possible.

If needed, edit `config/config.toml`:
- `venues.coinbase.*`
- `venues.polymarket.private_key` (for future live signing integration)
- `signals.tradingview.endpoint_secret`

## 4. Build + deploy release bundle (recommended)

```bash
# Build local release bundle into ./dist
./scripts/build_release_bundle.sh

# Deploy to EC2 host and restart service
./scripts/deploy_ec2.sh <ec2-host-or-ip> [ubuntu] [~/.ssh/key.pem] [/opt/Polymarket-Trader]
```

The deploy script performs post-restart health checks:
- `GET /healthz`
- `GET /ready`

## 5. Start service (manual path)

```bash
sudo systemctl daemon-reload
sudo systemctl enable pt-engine
sudo systemctl restart pt-engine
sudo journalctl -u pt-engine -f
```

## 6A. Maker-first market selection (before enabling live)
Use the scanner to evaluate spread-rich markets and generate maker-biased entry levels from real-time orderbook tops:

```bash
cargo run -p pt-cli -- scan-markets --config config/config.toml --limit 60 --top 15
```

Tune assumptions to match your EC2/live environment and hedge friction:

```bash
cargo run -p pt-cli -- scan-markets --config config/config.toml   --adverse-sel-est 0.0025 --hedge-cost-est 0.0008 --gas-amortized-est 0.0003
```

The live Polymarket executor already enforces `post_only=true`, which keeps entries maker-side when the venue accepts the quote.

## 6. Verify

```bash
curl http://127.0.0.1:8080/health
curl http://127.0.0.1:8080/healthz
curl http://127.0.0.1:8080/ready
curl http://127.0.0.1:8080/state/risk
curl http://127.0.0.1:8080/metrics
```

## 7. Operator controls

```bash
curl -X POST http://127.0.0.1:8080/ops/halt
curl -X POST http://127.0.0.1:8080/ops/resume
curl -X POST http://127.0.0.1:8080/ops/flatten
```

## 8. Rollback
- Keep previous deploy archives in `dist/` and on host staging paths.
- Restore prior bundle files under `/opt/Polymarket-Trader/` and restart `pt-engine`.
- Runtime emergency action: `POST /ops/halt` before rollback.
