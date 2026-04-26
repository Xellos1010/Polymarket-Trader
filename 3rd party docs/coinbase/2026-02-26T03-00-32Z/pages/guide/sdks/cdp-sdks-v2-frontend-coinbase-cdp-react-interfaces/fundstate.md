# fundstate

The state of the Fund component.

## Properties

Property

Type

`country`

`string`

`cryptoAmount?`

`number`

`cryptoCurrency`

`string`

`cryptoDecimalPlaces?`

`number`

`exchangeRate?`

`number`

`exchangeRateError?`

| `null` | [`FundStateError`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/FundStateError)

`isExchangeRatePending?`

`boolean`

`isPaymentMethodsPending?`

`boolean`

`fiatAmount?`

`number`

`fiatCurrency`

`string`

`fiatDecimalPlaces?`

`number`

`locale?`

`string`

`network`

`string`

`paymentMethods?`

[`FundPaymentMethod`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Interfaces/FundPaymentMethod)\[\]

`paymentMethodsError?`

| `null` | [`FundStateError`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/FundStateError)

`presetAmountInputs?`

[`FundPresetAmountInputs`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/FundPresetAmountInputs)

`selectedInputType`

[`InputType`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/InputType)

`selectedPaymentMethod?`

[`FundPaymentMethod`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Interfaces/FundPaymentMethod)

`subdivision?`

`string`

`transactionStatus`

[`FundLifecycleStatus`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/FundLifecycleStatus)

`destinationAddress`

`string`