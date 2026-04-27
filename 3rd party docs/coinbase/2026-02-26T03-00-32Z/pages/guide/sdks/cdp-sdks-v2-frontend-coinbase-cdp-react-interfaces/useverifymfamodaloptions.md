# useverifymfamodaloptions

Options for the useVerifyMfaModal hook.

## Extends

-   `Pick`<[`VerifyMfaModalProps`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Interfaces/VerifyMfaModalProps), `"onCancel"` | `"onError"` | `"onSuccess"`\>

## Properties

Property

Type

Description

Inherited from

`closeOnSuccessDelay?`

`null` | `number`

The delay in milliseconds before the modal is closed after successful verification. If `null`, the modal will not close automatically after success. If `0`, the modal closes immediately. Defaults to 500ms. **Default** `500`

\-

`onError?`

(`error`: `Error`) => `void`

A function to call when the verification errors.

`Pick.onError`

`onCancel?`

() => `void`

A function to call when the verification is canceled.

`Pick.onCancel`

`onSuccess?`

(`mfaCode`: `string`) => `void`

A function to call when MFA is successful verified.

`Pick.onSuccess`