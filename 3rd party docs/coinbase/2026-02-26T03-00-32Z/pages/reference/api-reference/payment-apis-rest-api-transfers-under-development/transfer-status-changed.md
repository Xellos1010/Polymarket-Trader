# transfer status changed

```
{
  "eventType": "payments.transfers.completed",
  "eventId": "123e4567-e89b-12d3-a456-426614174000",
  "timestamp": "2025-01-01T00:05:00Z",
  "data": {
    "transferId": "transfer_af2937b0-9846-4fe7-bfe9-ccc22d935114",
    "status": "completed",
    "source": {
      "accountId": "account_af2937b0-9846-4fe7-bfe9-ccc22d935114",
      "asset": "usd"
    },
    "target": {
      "address": "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913",
      "network": "base",
      "asset": "usdc"
    },
    "amount": "100.00",
    "asset": "usd",
    "sourceAmount": "103.50",
    "sourceAsset": "usd",
    "targetAmount": "100.00",
    "targetAsset": "usdc",
    "exchangeRate": {
      "sourceAsset": "usd",
      "targetAsset": "usdc",
      "rate": "1"
    },
    "fees": [
      {
        "type": "bank",
        "amount": "2.50",
        "asset": "usd"
      },
      {
        "type": "conversion",
        "amount": "1.00",
        "asset": "usd"
      }
    ],
    "onchainTransactions": [
      {
        "txHash": "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
        "network": "ethereum"
      }
    ],
    "completedAt": "2025-01-01T00:05:00Z",
    "executedAt": "2025-01-01T00:01:30Z",
    "createdAt": "2025-01-01T00:00:00Z",
    "updatedAt": "2025-01-01T00:05:00Z"
  }
}
```