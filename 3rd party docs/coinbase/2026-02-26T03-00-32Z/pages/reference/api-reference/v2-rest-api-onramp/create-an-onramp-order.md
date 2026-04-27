# create an onramp order

```
curl --request POST \
  --url https://api.cdp.coinbase.com/platform/v2/onramp/orders \
  --header 'Authorization: Bearer <token>' \
  --header 'Content-Type: application/json' \
  --data '
{
  "agreementAcceptedAt": "2025-04-24T00:00:00Z",
  "destinationAddress": "0x742d35Cc6634C0532925a3b844Bc454e4438f44e",
  "destinationNetwork": "base",
  "email": "test@example.com",
  "partnerUserRef": "user-1234",
  "paymentCurrency": "USD",
  "paymentMethod": "GUEST_CHECKOUT_APPLE_PAY",
  "phoneNumber": "+12055555555",
  "phoneNumberVerifiedAt": "2025-04-24T00:00:00Z",
  "purchaseCurrency": "USDC",
  "isQuote": false,
  "partnerOrderRef": "order-1234",
  "paymentAmount": "100.00",
  "purchaseAmount": "10.000000",
  "clientIp": "127.0.0.1",
  "domain": "pay.coinbase.com"
}
'
```

```
{  
  "order": {  
    "orderId": "123e4567-e89b-12d3-a456-426614174000",  
    "paymentTotal": "100.75",  
    "paymentSubtotal": "100",  
    "paymentCurrency": "USD",  
    "paymentMethod": "GUEST_CHECKOUT_APPLE_PAY",  
    "purchaseAmount": "100.000000",  
    "purchaseCurrency": "USDC",  
    "fees": [  
      {  
        "type": "FEE_TYPE_EXCHANGE",  
        "amount": "0.5",  
        "currency": "USD"  
      },  
      {  
        "type": "FEE_TYPE_NETWORK",  
        "amount": "0.25",  
        "currency": "USD"  
      }  
    ],  
    "exchangeRate": "1",  
    "destinationAddress": "0x71C7656EC7ab88b098defB751B7401B5f6d8976F",  
    "destinationNetwork": "base",  
    "status": "ONRAMP_ORDER_STATUS_COMPLETED",  
    "createdAt": "2025-04-24T00:00:00Z",  
    "updatedAt": "2025-04-24T00:00:00Z",  
    "txHash": "0x363cd3b3d4f49497cf5076150cd709307b90e9fc897fdd623546ea7b9313cecb",  
    "partnerUserRef": "user123"  
  },  
  "paymentLink": {  
    "url": "https://pay.coinbase.com/v2/api-onramp/apple-pay?sessionToken=MWYwNWQwODktZTZlYy02OTdlLTgzZTYtMTI3NzcyOWJhNjM3",  
    "paymentLinkType": "PAYMENT_LINK_TYPE_APPLE_PAY_BUTTON"  
  }  
}
```

Create a new Onramp order or get a quote for an Onramp order. Either `paymentAmount` or `purchaseAmount` must be provided.

This API currently only supports the payment method `GUEST_CHECKOUT_APPLE_PAY`.

For detailed integration instructions and to get access to this API, refer to the [Apple Pay Onramp API docs](https://docs.cdp.coinbase.com/onramp/headless-onramp/overview).

```
curl --request POST \
  --url https://api.cdp.coinbase.com/platform/v2/onramp/orders \
  --header 'Authorization: Bearer <token>' \
  --header 'Content-Type: application/json' \
  --data '
{
  "agreementAcceptedAt": "2025-04-24T00:00:00Z",
  "destinationAddress": "0x742d35Cc6634C0532925a3b844Bc454e4438f44e",
  "destinationNetwork": "base",
  "email": "test@example.com",
  "partnerUserRef": "user-1234",
  "paymentCurrency": "USD",
  "paymentMethod": "GUEST_CHECKOUT_APPLE_PAY",
  "phoneNumber": "+12055555555",
  "phoneNumberVerifiedAt": "2025-04-24T00:00:00Z",
  "purchaseCurrency": "USDC",
  "isQuote": false,
  "partnerOrderRef": "order-1234",
  "paymentAmount": "100.00",
  "purchaseAmount": "10.000000",
  "clientIp": "127.0.0.1",
  "domain": "pay.coinbase.com"
}
'
```

```
{  
  "order": {  
    "orderId": "123e4567-e89b-12d3-a456-426614174000",  
    "paymentTotal": "100.75",  
    "paymentSubtotal": "100",  
    "paymentCurrency": "USD",  
    "paymentMethod": "GUEST_CHECKOUT_APPLE_PAY",  
    "purchaseAmount": "100.000000",  
    "purchaseCurrency": "USDC",  
    "fees": [  
      {  
        "type": "FEE_TYPE_EXCHANGE",  
        "amount": "0.5",  
        "currency": "USD"  
      },  
      {  
        "type": "FEE_TYPE_NETWORK",  
        "amount": "0.25",  
        "currency": "USD"  
      }  
    ],  
    "exchangeRate": "1",  
    "destinationAddress": "0x71C7656EC7ab88b098defB751B7401B5f6d8976F",  
    "destinationNetwork": "base",  
    "status": "ONRAMP_ORDER_STATUS_COMPLETED",  
    "createdAt": "2025-04-24T00:00:00Z",  
    "updatedAt": "2025-04-24T00:00:00Z",  
    "txHash": "0x363cd3b3d4f49497cf5076150cd709307b90e9fc897fdd623546ea7b9313cecb",  
    "partnerUserRef": "user123"  
  },  
  "paymentLink": {  
    "url": "https://pay.coinbase.com/v2/api-onramp/apple-pay?sessionToken=MWYwNWQwODktZTZlYy02OTdlLTgzZTYtMTI3NzcyOWJhNjM3",  
    "paymentLinkType": "PAYMENT_LINK_TYPE_APPLE_PAY_BUTTON"  
  }  
}
```

#### Authorizations

A JWT signed using your CDP API Key Secret, encoded in base64. Refer to the [Generate Bearer Token](https://docs.cdp.coinbase.com/api-reference/v2/authentication#2-generate-bearer-token) section of our Authentication docs for information on how to generate your Bearer Token.

#### Body

agreementAcceptedAt

string<date-time>

required

Example:

`"2025-04-24T00:00:00Z"`

The address the purchased crypto will be sent to.

Required string length: `1 - 128`

Example:

`"0x742d35Cc6634C0532925a3b844Bc454e4438f44e"`

The name of the crypto network the purchased currency will be sent on.

Use the [Onramp Buy Options API](https://docs.cdp.coinbase.com/api-reference/rest-api/onramp-offramp/get-buy-options) to discover the supported networks for your user's location.

The verified email address of the user requesting the onramp transaction. This email must be verified by your app (via OTP) before being used with the Onramp API.

Example:

`"test@example.com"`

A unique string that represents the user in your app. This can be used to link individual transactions together so you can retrieve the transaction history for your users. Prefix this string with “sandbox-” (e.g. "sandbox-user-1234") to perform a sandbox transaction which will allow you to test your integration without any real transfer of funds.

This value can be used with with [Onramp User Transactions API](https://docs.cdp.coinbase.com/api-reference/rest-api/onramp-offramp/get-onramp-transactions-by-id) to retrieve all transactions created by the user.

The fiat currency to be converted to crypto.

The type of payment method to be used to complete an onramp order.

Available options

:

`GUEST_CHECKOUT_APPLE_PAY`

Example:

`"GUEST_CHECKOUT_APPLE_PAY"`

The phone number of the user requesting the onramp transaction in E.164 format. This phone number must be verified by your app (via OTP) before being used with the Onramp API.

Please refer to the [Onramp docs](https://docs.cdp.coinbase.com/onramp/headless-onramp/overview) for more details on phone number verification requirements and best practices.

phoneNumberVerifiedAt

string<date-time>

required

Timestamp of when the user's phone number was verified via OTP. User phone number must be verified every 60 days. If this timestamp is older than 60 days, an error will be returned.

Example:

`"2025-04-24T00:00:00Z"`

The ticker (e.g. `BTC`, `USDC`, `SOL`) or the Coinbase UUID (e.g. `d85dce9b-5b73-5c3c-8978-522ce1d1c1b4`) of the crypto asset to be purchased.

Use the [Onramp Buy Options API](https://docs.cdp.coinbase.com/api-reference/rest-api/onramp-offramp/get-buy-options) to discover the supported purchase currencies for your user's location.

If true, this API will return a quote without creating any transaction.

Optional partner order reference ID.

A string representing the amount of fiat the user wishes to pay in exchange for crypto. When using this parameter, the returned quote will be inclusive of fees i.e. the user will pay this exact amount of the payment currency.

A string representing the amount of crypto the user wishes to purchase. When using this parameter the returned quote will be exclusive of fees i.e. the user will receive this exact amount of the purchase currency.

The IP address of the end user requesting the onramp transaction.

The domain that the Apple Pay button will be rendered on. Required when using the `GUEST_CHECKOUT_APPLE_PAY` payment method and embedding the payment link in an iframe.

Example:

`"pay.coinbase.com"`

#### Response

Successfully created an onramp order.

A payment link to pay for an order.

Please refer to the [Onramp docs](https://docs.cdp.coinbase.com/onramp/headless-onramp/overview) for details on how to integrate with the different payment link types.