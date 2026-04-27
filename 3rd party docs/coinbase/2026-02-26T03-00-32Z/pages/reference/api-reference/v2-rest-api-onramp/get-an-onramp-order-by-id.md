# get an onramp order by id

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
  }
}
```