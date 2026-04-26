# exportwalletmodalprops

Props for the ExportWalletModal component.

## See

[ExportWalletModal](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/ExportWalletModal)

## Extends

-   `Pick`<[`ExportWalletProps`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Interfaces/ExportWalletProps), | `"address"` | `"onIframeReady"` | `"onCopySuccess"` | `"onIframeError"` | `"onIframeSessionExpired"`\>

## Properties

Property

Type

Description

Inherited from

`children?`

`ReactNode`

If provided, will render the children instead of the default trigger button.

\-

`open?`

`boolean`

Whether the modal is open. Note: if you set this, you must also set `setIsOpen`.

\-

`setIsOpen?`

(`value`: `boolean`) => `void`

A function to set the modal’s open state. Note: if you set this, you must also set `open`.

\-

`skipMfa?`

`boolean`

Whether to skip the MFA verification flow.

\-

`address`

`string`

The address of the account to export.

`Pick.address`

`onCopySuccess?`

() => `void`

A function to call when the key is copied successfully.

`Pick.onCopySuccess`

`onIframeReady?`

() => `void`

A function to call when the iframe UI is ready.

`Pick.onIframeReady`

`onIframeError?`

(`error?`: `string`) => `void`

A function to call when there is an error in the secure iframe

`Pick.onIframeError`

`onIframeSessionExpired?`

() => `void`

A function to call when the iframe session has expired.

`Pick.onIframeSessionExpired`