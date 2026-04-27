# get a transfer

```
{
  "transferId": "transfer_af2937b0-9846-4fe7-bfe9-ccc22d935114",
  "status": "quoted",
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
  "expiresAt": "2023-10-08T14:45:00Z",
  "createdAt": "2023-10-08T14:30:00Z",
  "updatedAt": "2023-10-08T14:30:00Z",
  "metadata": {
    "invoiceId": "12345",
    "reference": "Payment for invoice #12345"
  }
}
```