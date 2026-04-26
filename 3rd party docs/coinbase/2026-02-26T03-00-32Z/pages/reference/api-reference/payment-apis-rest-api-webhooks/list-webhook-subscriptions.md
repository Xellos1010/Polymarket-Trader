# list webhook subscriptions

List webhook subscriptions

```
{  
  "subscriptions": [  
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
  ],  
  "nextPageToken": "eyJsYXN0X2lkIjogImFiYzEyMyIsICJ0aW1lc3RhbXAiOiAxNzA3ODIzNzAxfQ=="  
}
```

#### Authorizations

A JWT signed using your CDP API Key Secret, encoded in base64. Refer to the [Generate Bearer Token](https://docs.cdp.coinbase.com/api-reference/v2/authentication#2-generate-bearer-token) section of our Authentication docs for information on how to generate your Bearer Token.

#### Query Parameters

The number of subscriptions to return per page.

Required range

: `1 <= x <= 100`

The token for the next page of subscriptions, if any.

#### Response

Webhook subscriptions retrieved successfully.

Response containing a list of webhook subscriptions.

The list of webhook subscriptions.

The token for the next page of items, if any.

Example:

`"eyJsYXN0X2lkIjogImFiYzEyMyIsICJ0aW1lc3RhbXAiOiAxNzA3ODIzNzAxfQ=="`