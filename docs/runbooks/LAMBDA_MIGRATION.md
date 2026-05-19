# Lambda Migration Runbook

Migrate the TradingView webhook + signal ingestion layer from Raspberry Pi
to AWS Lambda + API Gateway, with zero-downtime cutover via dual-write.

## Prerequisites

- AWS account with Lambda + API Gateway permissions
- Pi currently serving webhooks at `PI_WEBHOOK_URL` (set in environment)
- Lambda function deployed at `LAMBDA_WEBHOOK_URL` (set in environment)
- `jq`, `curl` available locally

## Architecture (dual-write phase)

```
TradingView alert
      │
      ├──► Pi endpoint (current)         → pt-engine signal pipeline
      └──► Lambda endpoint (new)         → same payload, same response contract
```

Both endpoints receive every alert during the dual-write window. Compare
responses to validate Lambda parity before cutting over DNS.

## Lambda Deployment

1. Package the webhook handler:
   ```bash
   cd lambda/webhook-handler && zip -r ../webhook.zip .
   aws lambda create-function \
     --function-name pt-webhook \
     --runtime python3.12 \
     --handler handler.lambda_handler \
     --zip-file fileb://../webhook.zip \
     --role arn:aws:iam::ACCOUNT_ID:role/lambda-basic-exec
   ```

2. Create API Gateway HTTP API and integrate with `pt-webhook`.

3. Set `LAMBDA_WEBHOOK_URL` in your local `.env` to the Gateway invoke URL.

## Dual-Write Test

Run before cutover to validate parity:

```bash
PI_WEBHOOK_URL=https://your-pi.example.com/webhook \
LAMBDA_WEBHOOK_URL=https://api.example.com/webhook \
./scripts/webhook_dual_write_test.sh
```

Exits 0 if both endpoints return HTTP 200 and identical JSON response bodies.
Exits non-zero and prints diff if responses diverge.

## Cutover Steps

1. Run `webhook_dual_write_test.sh` — must exit 0 for 3 consecutive runs.
2. Update TradingView alert webhook URL to Lambda endpoint.
3. Monitor Lambda CloudWatch logs for 30 minutes.
4. If stable: stop `pt-engine` on Pi webhook handler, keep other services running.
5. Run `pi-dev-down` only after 24h of stable Lambda operation.

## Rollback

Revert TradingView alert URL to Pi endpoint. Pi service should still be running
(do not stop until step 5 above).

## Success Criteria

- Dual-write test exits 0 for 3 consecutive runs.
- Lambda p99 latency ≤ 500ms (CloudWatch metric: `pt-webhook` invocation duration).
- No signal processing errors in `pt-engine` log for 30 minutes post-cutover.
- Dashboard shows signals flowing normally after cutover.
