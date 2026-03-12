# create a transfer

#### Authorizations

A JWT signed using your CDP API Key Secret, encoded in base64. Refer to the [Generate Bearer Token](https://docs.cdp.coinbase.com/api-reference/v2/authentication#2-generate-bearer-token) section of our Authentication docs for information on how to generate your Bearer Token.

#### Headers

An optional [UUID v4](https://www.uuidgenerator.net/version4) request header for making requests safely retryable. When included, duplicate requests with the same key will return identical responses. Refer to our [Idempotency docs](https://docs.cdp.coinbase.com/api-reference/v2/idempotency) for more information on using idempotency keys.

Required string length: `36`

#### Body

A request to create a transfer.

The source of the transfer.

-   Account
    
-   Payment Method
    

Example:

```
{  
  "accountId": "account_af2937b0-9846-4fe7-bfe9-ccc22d935114",  
  "asset": "usd"  
}
```

The target of the transfer.

-   Account
    
-   Payment Method
    
-   Onchain Address
    
-   Email Instrument
    

Example:

```
{  
  "accountId": "account_af2937b0-9846-4fe7-bfe9-ccc22d935114",  
  "asset": "usd"  
}
```

The amount of the transfer in atomic units of the asset specified by `asset`.

The symbol of the asset for the amount. This must be one of the assets of the source or target.

Required string length: `1 - 42`

Whether to immediately execute the transfer. If false, the transfer will be created in quoted status and must be executed manually via the /execute endpoint.

amountType

enum<string>

default:source

Specifies whether the given amount is to be received by the target or taken from the source.

-   `target`: The transfer `target` receives the exact value specified in `amount`. Fees are added to the amount taken from the transfer `source`.
-   `source`: The transfer `target` receives the value specified in `amount`, minus any fees.

Available options

:

`target`,

`source`

If true, validates the transfer without initiating it. If the request is valid, a 2xx will be returned. If the request is invalid, a 4xx error will be returned. The response will include an errorType, for e.g. invalid\_target if the specified target cannot receive funds.

Optional metadata as key-value pairs. Use this to store additional structured information on a resource, such as customer IDs, order references, or any application-specific data. Up to 50 key/value pairs may be provided. Keys and values are both strings. Keys must be ≤ 40 characters; values must be ≤ 500 characters.

Example:

```
{  
  "customer_id": "cust_12345",  
  "order_reference": "order-67890"  
}
```

(Preview, not yet supported) Required Travel Rule fields differ by region. These requirements are determined based on which Coinbase entity the customer has signed the service agreement for.

Example:

```
{  
  "isSelf": false,  
  "isIntermediary": true,  
  "originator": {  
    "name": "John Doe",  
    "address": {  
      "line1": "123 Main St",  
      "line2": "Unit 201",  
      "city": "Luxembourg",  
      "postCode": "L-1234",  
      "countryCode": "LU"  
    },  
    "financialInstitution": "PayPal, Inc.",  
    "vasp": {  
      "name": "Fidelity Digital Asset Services, LLC",  
      "address": {  
        "line1": "123 Market St",  
        "line2": "Suite 400",  
        "city": "San Francisco",  
        "state": "California",  
        "postCode": "94105",  
        "countryCode": "US"  
      },  
      "identifier": "5493001KJTIIGC8Y1R17"  
    }  
  },  
  "beneficiary": {  
    "name": "Jane Smith",  
    "address": {  
      "line1": "456 Oak Ave",  
      "city": "Paris",  
      "postCode": "75001",  
      "countryCode": "FR"  
    },  
    "walletType": "custodial"  
  }  
}
```

#### Response

Successfully created transfer.

A Transfer represents all the information needed to execute a transfer and tracks the lifecycle of a transfer from initiation through completion or failure.

The source of the transfer.

-   Account
    
-   Payment Method
    
-   Onchain Address
    

Example:

```
{  
  "accountId": "account_af2937b0-9846-4fe7-bfe9-ccc22d935114",  
  "asset": "usd"  
}
```

The target of the transfer.

-   Account
    
-   Payment Method
    
-   Onchain Address
    
-   Email Instrument
    

Example:

```
{  
  "accountId": "account_af2937b0-9846-4fe7-bfe9-ccc22d935114",  
  "asset": "usd"  
}
```

The ID of the transfer. Required when validateOnly is false.

Example:

`"transfer_af2937b0-9846-4fe7-bfe9-ccc22d935114"`

The current status of the transfer, indicating what action you need to take next. Required when validateOnly is false.

Available options

:

`quoted`,

`processing`,

`completed`,

`failed`

The amount of the source asset that will be transferred out in atomic units.

The asset symbol of the source amount.

Required string length: `1 - 42`

The amount of the target asset that will be transferred in atomic units.

The asset symbol of the target amount.

Required string length: `1 - 42`

Exchange rate information for currency conversion. The rate indicates how much of the target asset is equivalent to one unit of the source asset.

Example:

```
{  
  "sourceAsset": "usdc",  
  "targetAsset": "btc",  
  "rate": "0.00001"  
}
```

The fees associated with this transfer. Different transfer types have different fee structures.

NOTE: These examples are not exhaustive.

Common examples:

-   Crypto transfers: Network fees (gas) paid in the native token
-   Fiat conversions: Processing fees + exchange fees in USD
-   Wire transfers: Wire fees ($15) + processing fees ($5) in USD
-   Crypto conversions: Spread fees paid in the source asset.

Example:

```
[  
  {  
    "type": "bank",  
    "amount": "20",  
    "asset": "usd"  
  },  
  {  
    "type": "conversion",  
    "amount": "1.00",  
    "asset": "usdc"  
  },  
  {  
    "type": "network",  
    "amount": "0.03",  
    "asset": "eth"  
  }  
]
```

The date and time the transfer was completed.

Example:

`"2025-01-01T00:05:00Z"`

The reason for failure, if the transfer failed. Only present when status is `failed`.

Example:

`"Insufficient balance to complete this transfer."`

Detailed information about the last error that occurred while processing this transfer. Only present when status is `failed`.

Example:

```
{  
  "code": "insufficient_funds",  
  "message": "The source account does not have sufficient funds for this transfer.",  
  "param": "source.accountId",  
  "details": {  
    "availableBalance": "50.00",  
    "requiredAmount": "100.00"  
  }  
}
```

The date and time when this transfer will expire if not executed. Only present for `quoted` status. A new transfer must be created to obtain an updated quote after expiration. Required when validateOnly is false.

Example:

`"2025-01-01T00:15:00Z"`

The date and time the transfer was executed and moved to processing. Only present when status has progressed beyond `quoted`.

Example:

`"2025-01-01T00:01:30Z"`

The date and time the transfer was created. Required when validateOnly is false.

Example:

`"2025-01-01T00:00:00Z"`

The date and time the transfer was last updated. Required when validateOnly is false.

Example:

`"2025-01-01T00:00:00Z"`

If true, validates the transfer without initiating it. If the request is valid, a 2xx will be returned. If the request is invalid, a 4xx error will be returned. The response will include an errorType, for e.g. invalid\_target if the specified target cannot receive funds.

Optional metadata as key-value pairs. Use this to store additional structured information on a resource, such as customer IDs, order references, or any application-specific data. Up to 50 key/value pairs may be provided. Keys and values are both strings. Keys must be ≤ 40 characters; values must be ≤ 500 characters.

Example:

```
{  
  "customer_id": "cust_12345",  
  "order_reference": "order-67890"  
}
```

Additional details about the transfer. For example, if the transfer was sent to a deposit destination, the information about that destination will be included in this field.

Example:

```
{  
  "depositDestination": {  
    "id": "depositDestination_af2937b0-9846-4fe7-bfe9-ccc22d935114"  
  },  
  "onchainTransactions": [  
    {  
      "transactionHash": "0x363cd3b3d4f49497cf5076150cd709307b90e9fc897fdd623546ea7b9313cecb",  
      "network": "ethereum"  
    }  
  ]  
}
```