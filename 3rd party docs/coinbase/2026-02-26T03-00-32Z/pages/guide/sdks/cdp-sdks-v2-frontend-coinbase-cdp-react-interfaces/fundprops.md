# fundprops

All the props for the Fund component.

## Extends

-   [`FundStateProps`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/FundStateProps).`FundLifecycleEvents`

## Properties

Property

Type

Inherited from

`children?`

| `ReactNode` | (`state`: [`FundState`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Interfaces/FundState)) => `ReactNode`

\-

`className?`

`string`

\-

`fetchBuyOptions`

[`FetchBuyOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/FetchBuyOptions)

\-

`fetchBuyQuote`

[`FetchBuyQuote`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/FetchBuyQuote)

\-

`inputType?`

[`InputType`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/InputType)

\-

`openIn?`

`"popup"` | `"tab"`

\-

`redirectUrl?`

`string`

\-

`style?`

`CSSProperties`

\-

`submitLabel?`

`ReactNode`

\-

`title?`

`ReactNode`

\-

`country`

`string`

`FundStateProps.country`

`locale?`

`string`

`FundStateProps.locale`

`cryptoDecimalPlaces?`

`number`

`FundStateProps.cryptoDecimalPlaces`

`cryptoCurrency`

`string`

`FundStateProps.cryptoCurrency`

`fiatCurrency`

`string`

`FundStateProps.fiatCurrency`

`fiatDecimalPlaces?`

`number`

`FundStateProps.fiatDecimalPlaces`

`network`

`string`

`FundStateProps.network`

`presetAmountInputs?`

[`FundPresetAmountInputs`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/FundPresetAmountInputs)

`FundStateProps.presetAmountInputs`

`subdivision?`

`string`

`FundStateProps.subdivision`

`destinationAddress`

`string`

`FundStateProps.destinationAddress`

`onError?`

(`e`: | `undefined` | [`OnrampError`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Interfaces/OnrampError)) => `void`

`FundLifecycleEvents.onError`

`onStatus?`

(`lifecycleStatus`: [`FundLifecycleStatus`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/FundLifecycleStatus)) => `void`

`FundLifecycleEvents.onStatus`

`onSuccess?`

(`result?`: [`OnrampSuccessEventData`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/OnrampSuccessEventData)) => `void`

`FundLifecycleEvents.onSuccess`