# get a payment method

```
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
}
```

#### Authorizations

A JWT signed using your CDP API Key Secret, encoded in base64. Refer to the [Generate Bearer Token](https://docs.cdp.coinbase.com/api-reference/v2/authentication#2-generate-bearer-token) section of our Authentication docs for information on how to generate your Bearer Token.

#### Path Parameters

The unique identifier of the payment method. The ID of the Payment Method, which is a UUID prefixed by the string `paymentMethod_`.

Example:

`"paymentMethod_8e03978e-40d5-43e8-bc93-6894a57f9324"`

#### Response

Successfully retrieved payment method.

-   FedwirePaymentMethod
    
-   SwiftPaymentMethod
    

A payment method linked to your entity. Payment methods represent external financial instruments that can be used as a target for transfers.

The `paymentRail` field indicates which type-specific details object is present. Type-specific fields are nested under a key matching the rail name (e.g., `fedwire`, `swift`).

The ID of the Payment Method, which is a UUID prefixed by the string `paymentMethod_`.

Example:

`"paymentMethod_8e03978e-40d5-43e8-bc93-6894a57f9324"`

Whether the payment method is active and can be used in transfers. A payment method may be inactive due to verification requirements or entity-level restrictions.

createdAt

string<date-time>

required

The timestamp when the payment method was created.

Example:

`"2024-01-15T10:30:00Z"`

updatedAt

string<date-time>

required

The timestamp when the payment method was last updated.

Example:

`"2024-01-15T10:30:00Z"`

The payment rail for this payment method.

Available options

:

`fedwire`

Fedwire (domestic USD wire) details.

Example:

```
{  
  "asset": "usd",  
  "bankName": "ALLY BANK",  
  "accountLast4": "1234",  
  "routingNumber": "124003116"  
}
```