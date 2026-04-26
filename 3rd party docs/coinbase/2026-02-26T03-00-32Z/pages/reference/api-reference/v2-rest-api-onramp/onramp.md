# onramp

The v2 Onramp APIs are an evolution of our [v1 APIs](https://docs.cdp.coinbase.com/api-reference/rest-api/onramp-offramp/), designed to make the fiat-to-crypto experience feel native to your applications for higher conversion and less friction. These APIs only cover a subset of Coinbase Onramp functionality as described below, for all other use cases please refer to our [v1 APIs](https://docs.cdp.coinbase.com/api-reference/rest-api/onramp-offramp/).

## Supported Use Cases

#### Apple Pay Onramp

This use case allows you to offer an Apple Pay onramp experience with debit cards in your app without users needing to create or log into a Coinbase account. Our API returns a `paymentLink` URL that renders an Apple Pay button for you to load in a webview or iframe in your app, making the onramp experience feel native to your application. Refer to our [detailed integration guide](https://docs.cdp.coinbase.com/onramp/headless-onramp/apple-pay-onramp-api) for more information on the limitations and requirements of this use case. See our [onramp mobile demo app](https://github.com/coinbase/onramp-demo-mobile) for a reference implementation.

#### Hosted UI Onramp

This use case enables you to offer a full-featured onramp experience by redirecting users to a Coinbase-hosted page where they can choose to log into their existing Coinbase account or proceed as a guest. The Onramp Session API generates secure, single-use Onramp URLs with customizable parameters, allowing you to control available payment methods, preset transaction amounts, and cryptocurrencies for a tailored user experience. Refer to our [guide](https://docs.cdp.coinbase.com/onramp/headless-onramp/generating-onramp-url) for implementation details and customization options for this use case.