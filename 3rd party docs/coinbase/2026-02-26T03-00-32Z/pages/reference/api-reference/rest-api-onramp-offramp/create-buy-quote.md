# create buy quote

#### Authorizations

Enter your JSON Web Token (JWT) here. Refer to the [Generate JWT](https://developer.chrome.com/api-reference/authentication#2-generate-jwt-server-only) section of our Authentication docs for information on how to generate your Bearer Token.

#### Body

Create Buy Quote API request parameters

The ISO 3166-1 two letter country code e.g. `US`

Amount of fiat to be converted to purchase\_currency e.g. `100.00`

Fiat currency for payment\_amount e.g. `USD`

The type of payment method to be used to purchase

Available options

:

`UNSPECIFIED`,

`CARD`,

`ACH_BANK_ACCOUNT`,

`APPLE_PAY`,

`FIAT_WALLET`,

`CRYPTO_ACCOUNT`,

`GUEST_CHECKOUT_CARD`,

`PAYPAL`,

`RTP`,

`GUEST_CHECKOUT_APPLE_PAY`

The ticker (e.g. `BTC`, `USDC`) or the UUID (e.g. `d85dce9b-5b73-5c3c-8978-522ce1d1c1b4`) of crypto asset to be purchased

Network name to receive crypto on e.g. `ethereum`, `base`

The ISO 3166-2 two letter state code e.g. `NY`, only required for `US`

Destination Wallet address. Optional for creating buy quote. Required for generating a ready-to-use one-click-buy URL

The client IP address of the end user. This parameter ensures the quote can only be used by the requesting user. Do not trust HTTP headers like `X-Forwarded-For` — these can be easily spoofed.

#### Response

Create Buy Quote API response

A monetary amount represented by a decimal value and currency symbol

A monetary amount represented by a decimal value and currency symbol

Ready-to-use one-click-buy URL. Only returned when `destination_address` is provided in the request

A monetary amount represented by a decimal value and currency symbol

A monetary amount represented by a decimal value and currency symbol

A monetary amount represented by a decimal value and currency symbol

UUID that should be passed into the Onramp Widget URL as the `quoteId` query parameter