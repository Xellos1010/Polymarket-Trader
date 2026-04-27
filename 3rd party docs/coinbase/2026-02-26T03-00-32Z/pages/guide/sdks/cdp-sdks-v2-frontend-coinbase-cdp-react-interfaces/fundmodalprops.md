# fundmodalprops

-   [Extends](#extends)
-   [Properties](#properties)

## Extends

-   `Omit`<[`FundProps`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Interfaces/FundProps), `"children"`\>

## Properties

Property

Type

Description

Inherited from

`children?`

`ReactNode`

The children to render inside the modal.

\-

`open?`

`boolean`

Whether the modal is open. Note: if you set this, you must also set `setIsOpen`.

\-

`setIsOpen?`

(`value`: `boolean`) => `void`

A function to set the modal’s open state. Note: if you set this, you must also set `open`.

\-

`style?`

`CSSProperties`

\-

`Omit.style`

`title?`

`ReactNode`

\-

`Omit.title`

`className?`

`string`

\-

`Omit.className`

`onError?`

(`e`: | `undefined` | [`OnrampError`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Interfaces/OnrampError)) => `void`

\-

`Omit.onError`

`country`

`string`

\-

`Omit.country`

`onSuccess?`

(`result?`: [`OnrampSuccessEventData`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/OnrampSuccessEventData)) => `void`

\-

`Omit.onSuccess`

`submitLabel?`

`ReactNode`

\-

`Omit.submitLabel`

`locale?`

`string`

\-

`Omit.locale`

`cryptoDecimalPlaces?`

`number`

\-

`Omit.cryptoDecimalPlaces`

`cryptoCurrency`

`string`

\-

`Omit.cryptoCurrency`

`fiatCurrency`

`string`

\-

`Omit.fiatCurrency`

`fiatDecimalPlaces?`

`number`

\-

`Omit.fiatDecimalPlaces`

`inputType?`

[`InputType`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/InputType)

\-

`Omit.inputType`

`network`

`string`

\-

`Omit.network`

`presetAmountInputs?`

[`FundPresetAmountInputs`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/FundPresetAmountInputs)

\-

`Omit.presetAmountInputs`

`subdivision?`

`string`

\-

`Omit.subdivision`

`destinationAddress`

`string`

\-

`Omit.destinationAddress`

`fetchBuyOptions`

[`FetchBuyOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/FetchBuyOptions)

\-

`Omit.fetchBuyOptions`

`fetchBuyQuote`

[`FetchBuyQuote`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/FetchBuyQuote)

\-

`Omit.fetchBuyQuote`

`openIn?`

`"popup"` | `"tab"`

\-

`Omit.openIn`

`redirectUrl?`

`string`

\-

`Omit.redirectUrl`

`onStatus?`

(`lifecycleStatus`: [`FundLifecycleStatus`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/FundLifecycleStatus)) => `void`

\-

`Omit.onStatus`