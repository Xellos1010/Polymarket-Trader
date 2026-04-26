# createkeyexportiframeoptions

Options for setting up a key export iframe.

## Properties

Property

Type

Description

`address`

`string`

The address of the account to export.

`target`

`string` | `HTMLElement`

The target element: either a CSS selector string or an HTMLElement container. An iframe will be created and appended to the target container.

`projectId`

`string`

The project ID for authentication.

`label?`

`string`

The label for the button displayed in the iframe.

`copiedLabel?`

`string`

The label to display when the key is copied successfully.

`icon?`

`boolean`

Whether to show an icon in the button. **Default** `true`

`theme?`

`Partial`<[`SecureIframeTheme`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/SecureIframeTheme)\>

Theme overrides for the iframe button.

`onStatusUpdate?`

(`status`: [`SecureIframeStatus`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/SecureIframeStatus), `message?`: `string`) => `void`

Callback invoked when the iframe status changes.