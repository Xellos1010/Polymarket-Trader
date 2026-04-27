# verifymfastate

The state of the VerifyMfa component.

## Properties

Property

Type

Description

`method`

`"totp"` | `"sms"`

The currently selected MFA method.

`methods`

(`"totp"` | `"sms"`)\[\]

The available MFA methods for verification (from user’s enrolled methods).

`step`

[`VerifyMfaStep`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/VerifyMfaStep)

The current step of the verification flow.

`flowDirection`

`"left"` | `"right"`

The direction of the flow transition.

`mfaCode`

`string`

The MFA code entered by the user.

`error`

| `null` | `string` | [`APIError`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Classes/APIError)

Error message or APIError object.

`isPending`

`boolean`

Whether a verification request is pending.

`isSuccess`

`boolean`

Whether the verification was successful.