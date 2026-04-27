# get onramp user limits

#### Authorizations

A JWT signed using your CDP API Key Secret, encoded in base64. Refer to the [Generate Bearer Token](https://docs.cdp.coinbase.com/api-reference/v2/authentication#2-generate-bearer-token) section of our Authentication docs for information on how to generate your Bearer Token.

#### Body

The type of payment method to be used to complete an onramp order.

Available options

:

`GUEST_CHECKOUT_APPLE_PAY`

Example:

`"GUEST_CHECKOUT_APPLE_PAY"`

The user identifier value. For `phone_number` type, this must be in E.164 format.

The type of user identifier:

-   `phone_number`: A phone number in E.164 format associated with an onramp user.

Available options

:

`phone_number`

#### Response

Successfully retrieved user limits.

The list of limits applicable to the user.