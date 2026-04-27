# onrampbuyquoteresponse

```
type OnrampBuyQuoteResponse = {
  paymentTotal: OnrampAmount;
  paymentSubtotal: OnrampAmount;
  purchaseAmount: OnrampAmount;
  coinbaseFee: OnrampAmount;
  networkFee: OnrampAmount;
  quoteId: string;
  onrampUrl?: string;
};

```

The response from the Onramp Buy Quote API

## Properties

Property

Type

Description

`paymentTotal`

[`OnrampAmount`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/OnrampAmount)

Object with amount and currency of the total fiat payment required to complete the purchase, inclusive of any fees. The currency will match the `paymentCurrency` in the request if it is supported, otherwise it falls back to USD.

`paymentSubtotal`

[`OnrampAmount`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/OnrampAmount)

Object with amount and currency of the fiat cost of the crypto asset to be purchased, exclusive of any fees. The currency will match the `paymentCurrency`.

`purchaseAmount`

[`OnrampAmount`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/OnrampAmount)

Object with amount and currency of the crypto that to be purchased. The currency will match the `purchaseCurrency` in the request. The number of decimals will be based on the crypto asset.

`coinbaseFee`

[`OnrampAmount`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/OnrampAmount)

Object with amount and currency of the fee changed by the Coinbase exchange to complete the transaction. The currency will match the `paymentCurrency`.

`networkFee`

[`OnrampAmount`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/OnrampAmount)

Object with amount and currency of the network fee required to send the purchased crypto to the user’s wallet. The currency will match the `paymentCurrency`.

`quoteId`

`string`

Reference to the quote that should be passed into the initialization parameters when launching the Coinbase Onramp widget via the SDK or URL generator.

`onrampUrl?`

`string`

Ready-to-use one-click-buy URL. Only returned when destination\_address is provided in the request.