# verifymfamodalprops

Props for the VerifyMfaModal component.

## See

[VerifyMfaModal](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/VerifyMfaModal)

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

`onCancel?`

() => `void`

A function to call when the verification is canceled.

`onError?`

(`error`: `Error`) => `void`

A function to call when the verification errors.

`onSuccess?`

(`mfaCode`: `string`) => `void`

A function to call when MFA is successful verified.