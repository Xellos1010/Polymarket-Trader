# linkauthmodalprops

Props for the LinkAuthModal component.

## See

[LinkAuthModal](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/LinkAuthModal)

## Properties

Property

Type

Description

`children?`

`ReactNode`

If provided, will render the children instead of the default trigger button.

`open?`

`boolean`

Whether the modal is open. Note: if you set this, you must also set `setIsOpen`.

`setIsOpen?`

(`value`: `boolean`) => `void`

A function to set the modal’s open state. Note: if you set this, you must also set `open`.

`onLinkSuccess?`

(`method`: | `null` | [`AuthMethod`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/AuthMethod)) => `void`

A function to call when an auth method is successfully linked.