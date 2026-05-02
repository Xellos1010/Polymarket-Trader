# sendsolanatransactionbuttonprops

The props for the SendSolanaTransactionButton component.

## See

[SendSolanaTransactionButton](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/SendSolanaTransactionButton)

## Extends

-   `Omit`<[`ButtonProps`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Interfaces/ButtonProps), `"onError"`\>

## Properties

Property

Type

Description

Inherited from

`account`

`string`

The Solana account to send the transaction from.

\-

`network`

[`SendSolanaTransactionWithEndUserAccountBodyNetwork`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/SendSolanaTransactionWithEndUserAccountBodyNetwork)

The network to send the transaction on.

\-

`onError?`

(`error`: | `Error` | [`APIError`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Classes/APIError)) => `void`

A function to call when the transaction errors.

\-

`onSuccess?`

(`signature`: `string`) => `void`

A function to call when the transaction is successful.

\-

`transaction`

`string`

The base64 encoded transaction to send.

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