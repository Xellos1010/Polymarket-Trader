# deactivate payment link

```
{
  "id": "68f7a946db0529ea9b6d3a12",
  "url": "https://pay.coinbase.com/pl_01h8441j23abcd1234567890ef",
  "status": "ACTIVE",
  "amount": "100.50",
  "currency": "USDC",
  "network": "base",
  "address": "0x742d35Cc6634C0532925a3b844Bc454e4438f44e",
  "createdAt": "2024-03-20T10:30:00Z",
  "updatedAt": "2024-03-20T10:30:00Z",
  "tokenAddress": "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913",
  "description": "Payment for order #12345",
  "expiresAt": "2024-03-20T10:30:00Z",
  "metadata": {
    "invoiceId": "12345",
    "reference": "Payment for invoice #12345",
    "customerId": "cust_abc123"
  },
  "successRedirectUrl": "https://example.com/success",
  "failRedirectUrl": "https://example.com/failed",
  "settlement": {
    "totalAmount": "100.00",
    "feeAmount": "1.25",
    "netAmount": "98.75"
  },
  "transactionHash": "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
}
```