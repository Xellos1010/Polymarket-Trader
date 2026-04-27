# list payment methods

```
{
  "paymentMethods": [
    {
      "paymentMethodId": "paymentMethod_8e03978e-40d5-43e8-bc93-6894a57f9324",
      "paymentRail": "fedwire",
      "active": true,
      "createdAt": "2024-01-15T10:30:00Z",
      "updatedAt": "2024-01-15T10:30:00Z",
      "fedwire": {
        "asset": "usd",
        "bankName": "ALLY BANK",
        "accountLast4": "1234",
        "routingNumber": "124003116"
      }
    },
    {
      "paymentMethodId": "paymentMethod_def45678-1234-5678-9abc-def012345678",
      "paymentRail": "swift",
      "active": true,
      "createdAt": "2024-01-15T10:30:00Z",
      "updatedAt": "2024-01-15T10:30:00Z",
      "swift": {
        "asset": "eur",
        "bankName": "Deutsche Bank",
        "ibanLast4": "5678",
        "bic": "DEUTDEFF"
      }
    }
  ],
  "nextPageToken": "eyJsYXN0X2lkIjogImFiYzEyMyJ9"
}
```