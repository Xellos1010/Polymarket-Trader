# signinmodalprops

Props for the SignInModal component.

## See

[SignInModal](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/SignInModal)

## Properties

Property

Type

Description

`authMethods?`

[`AuthMethod`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/AuthMethod)\[\]

Filter the auth methods that are shown to the user (this still respects the CDP config auth methods).

`children?`

`ReactNode`

If provided, will render the children instead of the default trigger button.

`open?`

`boolean`

Whether the modal is open. Note: if you set this, you must also set `setIsOpen`.

`setIsOpen?`

(`value`: `boolean`) => `void`

A function to set the modal’s open state. Note: if you set this, you must also set `open`.

`onSuccess?`

() => `void`

A function to call when the sign-in flow is successful.