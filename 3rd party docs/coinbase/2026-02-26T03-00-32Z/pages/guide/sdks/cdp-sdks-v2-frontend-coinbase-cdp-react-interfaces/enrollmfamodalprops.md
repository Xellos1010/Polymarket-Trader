# enrollmfamodalprops

Props for the EnrollMfaModal component.

## See

[EnrollMfaModal](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/EnrollMfaModal)

## Extends

-   `Pick`<[`EnrollMfaProps`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Interfaces/EnrollMfaProps), `"onEnrollSuccess"` | `"resetOnSuccess"`\>

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

`onEnrollSuccess?`

() => `void`

A function to call when the enrollment is successful.

`Pick.onEnrollSuccess`

`resetOnSuccess?`

`boolean`

Whether to reset the enrollment state when the enrollment is successful. Defaults to `true`.

`Pick.resetOnSuccess`