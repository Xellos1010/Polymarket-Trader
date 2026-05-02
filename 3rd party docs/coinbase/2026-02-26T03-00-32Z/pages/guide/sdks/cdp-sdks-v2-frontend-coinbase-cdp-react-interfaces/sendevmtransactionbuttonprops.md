# sendevmtransactionbuttonprops

The props for the SendEvmTransactionButton component.

## See

[SendEvmTransactionButton](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/SendEvmTransactionButton)

## Extends

-   `Omit`<[`ButtonProps`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Interfaces/ButtonProps), `"onError"`\>

## Properties

Property

Type

Description

Inherited from

`account`

`` `0x${string}` ``

The account to send the transaction from.

\-

`network`

[`SendEvmTransactionWithEndUserAccountBodyNetwork`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/SendEvmTransactionWithEndUserAccountBodyNetwork)

The network to send the transaction on.

\-

`onError?`

(`error`: | `Error` | [`APIError`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Classes/APIError)) => `void`

A function to call when the transaction errors.

\-

`onSuccess?`

(`hash`: `string`) => `void`

A function to call when the transaction is successful.

\-

`transaction`

[`AllowedEvmTransactionType`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-hooks/Type-Aliases/AllowedEvmTransactionType)

The transaction to send.

\-

`size?`

[`ButtonSize`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/ButtonSize)

The size of the button. Defaults to “md”.

`Omit.size`

`asChild?`

`boolean`

Set to true to use a custom element or component in place of the default button element.

`Omit.asChild`

`fullWidth?`

`boolean`

Whether the button should be full width.

`Omit.fullWidth`

`isPending?`

`boolean`

Whether the button state is pending.

`Omit.isPending`

`pendingLabel?`

`ReactNode`

A label to render when the button state is pending.

`Omit.pendingLabel`

`variant?`

[`ButtonVariant`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/ButtonVariant)

The variant of the button. Defaults to “primary”.

`Omit.variant`