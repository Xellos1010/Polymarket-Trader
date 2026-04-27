# create webhook subscription

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

#### Authorizations

A JWT signed using your CDP API Key Secret, encoded in base64. Refer to the [Generate Bearer Token](https://docs.cdp.coinbase.com/api-reference/v2/authentication#2-generate-bearer-token) section of our Authentication docs for information on how to generate your Bearer Token.

#### Body

Request to create a new transfer webhook subscription.

Types of transfer events to subscribe to. Valid values are: `payments.transfers.quoted`, `payments.transfers.processing`, `payments.transfers.completed`, `payments.transfers.failed`.

Example:

```
["payments.transfers.completed"]
```

Whether the subscription is enabled.

Target configuration for webhook delivery. Specifies the destination URL and any custom headers to include in webhook requests.

Example:

```
{  
  "url": "https://api.example.com/webhooks",  
  "headers": {  
    "Authorization": "Bearer token123",  
    "Content-Type": "application/json"  
  }  
}
```

Description of the webhook subscription.

Maximum string length: `500`

Example:

`"Transfer status change notifications"`

Optional metadata as key-value pairs. Use this to store additional structured information on a resource, such as customer IDs, order references, or any application-specific data. Up to 50 key/value pairs may be provided. Keys and values are both strings. Keys must be ≤ 40 characters; values must be ≤ 500 characters.

Example:

```
{  
  "customer_id": "cust_12345",  
  "order_reference": "order-67890"  
}
```

Labels are not supported for transfer webhooks.

#### Response

Webhook subscription created successfully.

Response containing webhook subscription details.

createdAt

string<date-time>

required

When the subscription was created.

Example:

`"2025-01-15T10:30:00Z"`

Types of events to subscribe to. Event types follow a three-part dot-separated format: service.resource.verb (e.g., "onchain.activity.detected", "wallet.activity.detected", "onramp.transaction.created").

Example:

```
["onchain.activity.detected"]
```

Whether the subscription is enabled.

Secret for webhook signature validation.

Example:

`"123e4567-e89b-12d3-a456-426614174000"`

Unique identifier for the subscription.

Example:

`"123e4567-e89b-12d3-a456-426614174000"`

Target configuration for webhook delivery. Specifies the destination URL and any custom headers to include in webhook requests.

Example:

```
{  
  "url": "https://api.example.com/webhooks",  
  "headers": {  
    "Authorization": "Bearer token123",  
    "Content-Type": "application/json"  
  }  
}
```

Description of the webhook subscription.

Maximum string length: `500`

Example:

`"Subscription for token transfer events"`

Additional metadata for the subscription.

Example:

```
{  
  "customer_id": "cust_12345",  
  "order_reference": "order-67890",  
  "secret": "123e4567-e89b-12d3-a456-426614174000"  
}
```

Multi-label filters using total overlap logic. Total overlap means the subscription only triggers when events contain ALL these key-value pairs. Present when subscription uses multi-label format.

Example:

```
{  
  "env": "dev",  
  "team": "payments",  
  "contract_address": "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913"  
}
```