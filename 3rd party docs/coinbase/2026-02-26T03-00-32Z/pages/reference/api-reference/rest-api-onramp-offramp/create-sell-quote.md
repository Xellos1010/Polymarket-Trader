# create sell quote

#### Authorizations

Enter your JSON Web Token (JWT) here. Refer to the [Generate JWT](https://developer.chrome.com/api-reference/authentication#2-generate-jwt-server-only) section of our Authentication docs for information on how to generate your Bearer Token.

#### Body

Create Sell Quote API request parameters

Fiat currency to be cashed out to e.g. `USD`

The ISO 3166-1 two letter country code e.g. `US`

Payment method type to be deposited to

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

Amount of sell\_currency to be converted to fiat e.g. `0.05`

The ticker (e.g. `BTC`, `USDC`) or the UUID (e.g. `d85dce9b-5b73-5c3c-8978-522ce1d1c1b4`) of crypto asset to be sold

Partner's user identifier. Optional field for quote, required for offramp URL generation

URL to redirect after transaction completion. Optional field for quote, required for generating a ready-to-use one-click-sell URL

Network name crypto will be sent on e.g. `ethereum`, `base`

Optional source address for the asset to sell. Optional field for quote, required for generating a ready-to-use one-click-sell URL

The ISO 3166-2 two letter state code e.g. `NY`, only required for `US`

The client IP address of the end user. This parameter ensures the quote can only be used by the requesting user. Do not trust HTTP headers like `X-Forwarded-For` — these can be easily spoofed.

#### Response

Create Sell Quote API response

A monetary amount represented by a decimal value and currency symbol

A monetary amount represented by a decimal value and currency symbol

A monetary amount represented by a decimal value and currency symbol

Ready-to-use offramp URL. Only returned when `sourceAddress`, `redirectUrl`, and `partnerUserRef` are ALL provided in the request

UUID that should be passed into the Offramp Widget URL as the `quoteId` query parameter

A monetary amount represented by a decimal value and currency symbol