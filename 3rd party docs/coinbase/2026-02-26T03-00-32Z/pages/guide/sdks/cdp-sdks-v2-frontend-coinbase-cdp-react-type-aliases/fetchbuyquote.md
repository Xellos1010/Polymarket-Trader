# fetchbuyquote

```
type FetchBuyQuote = (params: {
  destinationAddress: string;
  purchaseCurrency: string;
  purchaseNetwork: string;
  paymentAmount: string;
  paymentCurrency: string;
  paymentMethod: string;
  country: string;
  subdivision?: string;
}) => Promise<OnrampBuyQuoteResponse>;

```

Get Buy Quote function (used for fetching the exchange rate and building the buy url)

## Parameters

Parameter

Type

`params`

{ `destinationAddress`: `string`; `purchaseCurrency`: `string`; `purchaseNetwork`: `string`; `paymentAmount`: `string`; `paymentCurrency`: `string`; `paymentMethod`: `string`; `country`: `string`; `subdivision?`: `string`; }

`params.destinationAddress`

`string`

`params.purchaseCurrency`

`string`

`params.purchaseNetwork`

`string`

`params.paymentAmount`

`string`

`params.paymentCurrency`

`string`

`params.paymentMethod`

`string`

`params.country`

`string`

`params.subdivision?`

`string`

## Returns

`Promise`<[`OnrampBuyQuoteResponse`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/OnrampBuyQuoteResponse)\>