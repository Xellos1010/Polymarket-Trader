# enrollmfastate

The state of the EnrollMfa component.

## Properties

Property

Type

Description

`method`

`"totp"` | `"sms"`

The currently selected MFA method.

`methods`

(`"totp"` | `"sms"`)\[\]

The available MFA methods for enrollment.

`step`

[`EnrollMfaStep`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/EnrollMfaStep)

The current step of the enrollment flow.

`flowDirection`

`"left"` | `"right"`

The direction of the flow.

`mfaCode`

`string`

The MFA code entered by the user (for verification step).

`authUrl`

`null` | `string`

The otpauth:// URL for QR code generation (from initiate).

`secret`

`null` | `string`

The base32-encoded secret for manual entry (from initiate).

`phoneNumber`

`string`

The phone number for SMS MFA enrollment (E.164 format).

`initiatedAt`

`null` | `number`

The timestamp when enrollment was initiated.

`isExpired`

`boolean`

Whether the enrollment session has expired.

`error`

| `null` | `string` | [`APIError`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Classes/APIError)

Error message or APIError object.

`isPending`

`boolean`

Whether a request is pending.

`isSuccess`

`boolean`

Whether the enrollment was successful.