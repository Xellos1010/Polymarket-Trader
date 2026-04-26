#!/usr/bin/env bash
set -euo pipefail

if [[ "${EUID}" -ne 0 ]]; then
  echo "Run as root: sudo $0"
  exit 1
fi

echo "[1/6] Setting CPU governor to performance"
if command -v cpupower >/dev/null 2>&1; then
  cpupower frequency-set -g performance || true
fi

echo "[2/6] Disabling irqbalance (optional)"
systemctl stop irqbalance >/dev/null 2>&1 || true
systemctl disable irqbalance >/dev/null 2>&1 || true

echo "[3/6] Raising network socket buffers"
sysctl -w net.core.rmem_max=134217728
sysctl -w net.core.wmem_max=134217728
sysctl -w net.ipv4.tcp_rmem='4096 87380 134217728'
sysctl -w net.ipv4.tcp_wmem='4096 65536 134217728'

echo "[4/6] Enabling TCP low latency hints"
sysctl -w net.ipv4.tcp_low_latency=1 || true

echo "[5/6] Persisting sysctl profile"
cat >/etc/sysctl.d/99-pt-trader.conf <<CONF
net.core.rmem_max=134217728
net.core.wmem_max=134217728
net.ipv4.tcp_rmem=4096 87380 134217728
net.ipv4.tcp_wmem=4096 65536 134217728
net.ipv4.tcp_low_latency=1
CONF
sysctl --system >/dev/null

echo "[6/6] Completed baseline tuning"
echo "Next: configure taskset/cgroup cpuset for feed/strategy/execution threads via runtime.affinity"
