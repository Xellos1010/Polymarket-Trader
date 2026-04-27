# get webhook subscription details

```
{
  "subscriptionId": "123e4567-e89b-12d3-a456-426614174000",
  "eventTypes": [
    "onchain.activity.detected"
  ],
  "isEnabled": true,
  "labels": {
    "contract_address": "0x833589fcd6edb6e08f4c7c32d4f71b54bda02913",
    "event_name": "Transfer",
    "network": "base-mainnet",
    "transaction_to": "0xf5042e6ffac5a625d4e7848e0b01373d8eb9e222"
  },
  "description": "USDC Transfer events to specific address.",
  "createdAt": "2025-11-12T09:19:52.051Z",
  "metadata": {
    "secret": "a1b2c3d4-e5f6-7890-abcd-ef1234567890"
  },
  "secret": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
  "target": {
    "url": "https://api.example.com/webhooks"
  }
}
```