#!/usr/bin/env bash
set -euo pipefail

sudo apt-get update
sudo apt-get install -y curl git build-essential pkg-config libssl-dev

if ! command -v rustup >/dev/null 2>&1; then
  curl https://sh.rustup.rs -sSf | sh -s -- -y
fi

source "$HOME/.cargo/env"
rustup default stable

cd /opt
if [ ! -d Polymarket-Trader ]; then
  sudo git clone https://github.com/Xellos1010/Polymarket-Trader.git || true
fi

sudo mkdir -p /opt/Polymarket-Trader/data/output/parquet
sudo chown -R "$USER":"$USER" /opt/Polymarket-Trader

cd /opt/Polymarket-Trader
cargo build --release -p pt-cli

cat <<'SERVICE' | sudo tee /etc/systemd/system/pt-engine.service
[Unit]
Description=Polymarket Rust Trader Engine
After=network.target

[Service]
Type=simple
User=ubuntu
WorkingDirectory=/opt/Polymarket-Trader
Environment=RUST_LOG=info
ExecStart=/opt/Polymarket-Trader/target/release/pt-cli run --config /opt/Polymarket-Trader/config/config.toml
Restart=always
RestartSec=3

[Install]
WantedBy=multi-user.target
SERVICE

sudo systemctl daemon-reload
sudo systemctl enable pt-engine
sudo systemctl restart pt-engine
sudo systemctl status pt-engine --no-pager
