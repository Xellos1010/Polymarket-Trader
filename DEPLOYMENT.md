# Deployment (Single EC2 Host)

## 1. Provision
- EC2 `t3.small` (or `t3.micro` for lighter paper mode), Ubuntu 22.04, `us-east-1`.
- Security group:
  - SSH: `22` from your IP.
  - Dashboard: `8080` only from your IP/VPN.
  - TradingView listener (optional): `8090` only from trusted sources.

## 2. Install + bootstrap

```bash
chmod +x scripts/bootstrap_ubuntu.sh
./scripts/bootstrap_ubuntu.sh
```

## 3. Configure secrets
Edit `config/config.toml`:
- `venues.coinbase.*`
- `venues.polymarket.private_key` (for future live signing integration)
- `signals.tradingview.endpoint_secret`

## 4. Start service

```bash
sudo systemctl daemon-reload
sudo systemctl enable pt-engine
sudo systemctl restart pt-engine
sudo journalctl -u pt-engine -f
```

## 5. Verify

```bash
curl http://127.0.0.1:8080/health
curl http://127.0.0.1:8080/state/risk
curl http://127.0.0.1:8080/metrics
```

## 6. Operator controls

```bash
curl -X POST http://127.0.0.1:8080/ops/halt
curl -X POST http://127.0.0.1:8080/ops/resume
curl -X POST http://127.0.0.1:8080/ops/flatten
```

## 7. Rollback
- Keep prior binary at `/opt/Polymarket-Trader/target/release/pt-cli.prev`.
- Swap symlink or restore binary and restart service.
