# get invoice

```
{
  "uuid": "750a84dd-2460-504c-8bb9-f6fa731a2361",
  "invoiceNumber": "INV-1234",
  "contactName": "John Doe",
  "contactEmail": "john.doe@example.com",
  "lineItems": [
    {
      "itemName": "Web Development Services",
      "quantity": 10,
      "unitPrice": {
        "value": "100.50",
        "currency": "USDC"
      }
    }
  ],
  "totalAmountDue": {
    "value": "100.50",
    "currency": "USDC"
  },
  "status": "OPEN",
  "createdAt": "2024-03-20T10:30:00Z",
  "updatedAt": "2024-03-20T10:30:00Z",
  "createdBy": "750a84dd-2460-504c-8bb9-f6fa731a2361",
  "lastUpdatedBy": "750a84dd-2460-504c-8bb9-f6fa731a2361",
  "entityName": "Acme Corporation",
  "contactAddress": {
    "addressLine1": "123 Main Street",
    "city": "San Francisco",
    "country": "US",
    "addressLine2": "Suite 400",
    "state": "CA",
    "postalCode": "94103"
  },
  "purchaseOrderNumber": "PO-5678",
  "dueDate": "2024-03-20T10:30:00Z",
  "sendDate": "2024-03-20T10:30:00Z",
  "invoiceDate": "2024-03-20T10:30:00Z",
  "discount": {
    "value": "100.50",
    "currency": "USDC"
  },
  "tax": {
    "value": "100.50",
    "currency": "USDC"
  },
  "privateNotes": "Internal memo about this invoice",
  "memo": "Thank you for your business!",
  "footer": "Payment terms: Net 30",
  "recurringInvoicePlanId": "68f7a946db0529ea9b6d3a12",
  "paymentMethod": {
    "crypto": {
      "paymentLinkUrl": "https://pay.coinbase.com/pl_01h8441j23abcd1234567890ef",
      "paymentLinkId": "68f7a946db0529ea9b6d3a12",
      "transactionHash": "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
    }
  },
  "entityAddress": {
    "addressLine1": "123 Main Street",
    "city": "San Francisco",
    "country": "US",
    "addressLine2": "Suite 400",
    "state": "CA",
    "postalCode": "94103"
  }
}
```