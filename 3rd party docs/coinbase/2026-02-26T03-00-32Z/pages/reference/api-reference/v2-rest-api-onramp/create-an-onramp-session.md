# create an onramp session

Returns a single-use URL for an Onramp session. This API provides flexible functionality based on the parameters provided, supporting three cases:

**Important**: The returned URL is single-use only. Once a user visits the URL, no one else can access it.

## Use Cases

### 1\. Basic Session (Minimum Parameters)

**Required**: `destinationAddress`, `purchaseCurrency`, `destinationNetwork`

**Returns**: Basic single-use onramp URL. The `quote` object will not be included in the response.

### 2\. One-Click Onramp URL

**Required**: Basic parameters + (`paymentAmount` OR `purchaseAmount`), `paymentCurrency`

**Returns**: One-click onramp URL for streamlined checkout. The `quote` object will not be included in the response.

### 3\. One-Click Onramp URL with Quote

**Required**: One-Click Onramp parameters + `paymentMethod`, `country`, `subdivision`

**Returns**: Complete pricing quote and one-click onramp URL. Both `session` and `quote` objects will be included in the response.

**Note**: Only one of `paymentAmount` or `purchaseAmount` should be provided, not both. Providing both will result in an error. When `paymentAmount` is provided, the quote shows how much crypto the user will receive for the specified fiat amount (fee-inclusive). When `purchaseAmount` is provided, the quote shows how much fiat the user needs to pay for the specified crypto amount (fee-exclusive).