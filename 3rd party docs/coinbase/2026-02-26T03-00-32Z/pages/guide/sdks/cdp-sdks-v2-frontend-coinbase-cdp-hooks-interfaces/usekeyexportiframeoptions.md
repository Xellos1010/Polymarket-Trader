# usekeyexportiframeoptions

Options for the EVM and Solana key export iframe hooks.

## Properties

Property

Type

Description

`address`

`string`

The address of the account to export.

`containerRef`

`RefObject`<`null` | `HTMLElement`\>

A ref to the container element where the iframe will be appended.

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

`Partial`<`SecureIframeTheme`\>

Theme overrides for the iframe button.