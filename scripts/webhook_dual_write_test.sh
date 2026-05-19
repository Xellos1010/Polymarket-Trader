#!/usr/bin/env bash
# Validate response parity between Pi and Lambda webhook endpoints.
# Both must return HTTP 200 with identical JSON bodies.
#
# Usage:
#   PI_WEBHOOK_URL=https://... LAMBDA_WEBHOOK_URL=https://... ./webhook_dual_write_test.sh
set -euo pipefail

: "${PI_WEBHOOK_URL:?PI_WEBHOOK_URL required}"
: "${LAMBDA_WEBHOOK_URL:?LAMBDA_WEBHOOK_URL required}"

PAYLOAD='{"strategy_name":"dual-write-test","action":"buy","bias":0.8,"confidence":0.9}'

send_request() {
  local url="$1"
  local name="$2"
  local http_code body

  body=$(curl -sfS -X POST "$url" \
    -H "Content-Type: application/json" \
    -d "$PAYLOAD" \
    -o /tmp/dw_response_"$name".json \
    -w "%{http_code}" 2>&1) || {
    echo "FAIL: $name endpoint unreachable or curl error"
    return 1
  }
  http_code="$body"
  body=$(cat /tmp/dw_response_"$name".json 2>/dev/null || echo "")
  echo "$http_code"
}

echo "==> Sending test payload to Pi endpoint..."
PI_CODE=$(send_request "$PI_WEBHOOK_URL" "pi") || exit 1
PI_BODY=$(cat /tmp/dw_response_pi.json 2>/dev/null || echo "")

echo "==> Sending test payload to Lambda endpoint..."
LAMBDA_CODE=$(send_request "$LAMBDA_WEBHOOK_URL" "lambda") || exit 1
LAMBDA_BODY=$(cat /tmp/dw_response_lambda.json 2>/dev/null || echo "")

echo "Pi    HTTP $PI_CODE: $PI_BODY"
echo "Lambda HTTP $LAMBDA_CODE: $LAMBDA_BODY"

if [[ "$PI_CODE" != "200" ]]; then
  echo "FAIL: Pi returned non-200 ($PI_CODE)"
  exit 1
fi
if [[ "$LAMBDA_CODE" != "200" ]]; then
  echo "FAIL: Lambda returned non-200 ($LAMBDA_CODE)"
  exit 1
fi

# Normalize and diff JSON responses
PI_NORM=$(echo "$PI_BODY" | jq -S . 2>/dev/null || echo "$PI_BODY")
LAMBDA_NORM=$(echo "$LAMBDA_BODY" | jq -S . 2>/dev/null || echo "$LAMBDA_BODY")

DIFF=$(diff <(echo "$PI_NORM") <(echo "$LAMBDA_NORM") || true)

if [[ -n "$DIFF" ]]; then
  echo "FAIL: response bodies differ:"
  echo "$DIFF"
  exit 1
fi

echo "PASS: both endpoints returned HTTP 200 with identical bodies."
rm -f /tmp/dw_response_pi.json /tmp/dw_response_lambda.json
