# exportwalletprops

The props for the ExportWallet component.

## See

[ExportWallet](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/ExportWallet)

## Extends

-   `Omit`<`HTMLAttributes`<`HTMLDivElement`\>, `"children"`\>

## Properties

Property

Type

Description

`children?`

| `ReactNode` | (`props`: { `type`: `"solana"` | `"evm-eoa"` | `"evm-smart"`; `isSessionExpired`: `boolean`; }) => `ReactNode`

The children to render. If a function is provided, it will be called with the address type and isSessionExpired as props.

`address`

`string`

The address of the account to export.

`onIframeReady?`

() => `void`

A function to call when the iframe UI is ready.

`onCopySuccess?`

() => `void`

A function to call when the key is copied successfully.

`onIframeError?`

(`error?`: `string`) => `void`

A function to call when there is an error in the secure iframe

`onIframeSessionExpired?`

() => `void`

A function to call when the iframe session has expired.

`skipMfa?`

`boolean`

Skip the MFA verification flow. Set to true if you want to handle MFA verification separately or if you’re using your own MFA UI. **Default** `false`