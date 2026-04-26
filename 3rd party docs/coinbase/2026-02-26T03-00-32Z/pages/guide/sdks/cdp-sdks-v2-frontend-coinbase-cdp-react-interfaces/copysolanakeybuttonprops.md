# copysolanakeybuttonprops

The props for the CopySolanaKeyButton component.

## See

-   [CopySolanaKeyButton](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/CopySolanaKeyButton)
-   [IframeTheme](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/IframeTheme)

## Extends

-   `Pick`<[`ButtonProps`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Interfaces/ButtonProps), `"size"` | `"fullWidth"`\>

## Properties

Property

Type

Description

Inherited from

`address`

`string`

The address of the solana account to copy.

\-

`className?`

`string`

The class name to apply to the button.

\-

`copiedLabel?`

`string`

The label to display when the private key is copied.

\-

`icon?`

`boolean`

Whether to show an icon in the button.

\-

`label?`

`string`

The label to display on the button.

\-

`onReady?`

() => `void`

A function to call when the iframe UI is ready.

\-

`onSessionExpired?`

() => `void`

A function to call when the iframe session has expired.

\-

`onSuccess?`

() => `void`

A function to call when the key is copied successfully.

\-

`onError?`

(`error?`: `string`) => `void`

A function to call when there is an error in the secure iframe

\-

`theme?`

`Partial`<`SecureIframeTheme`\>

Theme overrides for the iframe.

\-

`variant?`

`"primary"` | `"secondary"`

The variant of the button.

\-

`size?`

[`ButtonSize`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/ButtonSize)

The size of the button. Defaults to “md”.

`Pick.size`

`fullWidth?`

`boolean`

Whether the button should be full width.

`Pick.fullWidth`