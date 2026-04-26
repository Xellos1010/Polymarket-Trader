# signinstate

The state of the SignIn component.

## See

-   [SignIn](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/SignIn)
-   [useSignInReducer](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Hooks/useSignInReducer)

## Properties

Property

Type

Description

`authMethod`

[`AuthMethod`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/AuthMethod)

The auth method selected by the user.

`authMethods`

[`AuthMethod`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/AuthMethod)\[\]

\-

`canResetOTP`

`boolean`

Whether the user can request a new OTP.

`email`

`string`

The email address of the user.

`error`

| `null` | `string` | [`APIError`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Classes/APIError)

The error message or APIError object.

`flowId`

`string`

The flow ID of the current sign-in flow.

`isPending`

`boolean`

Whether the form state is pending.

`isSuccess`

`boolean`

Whether the sign-in flow is successful.

`otp`

`string`

The OTP code entered by the user.

`phoneNumber`

`string`

The phone number of the user.

`step`

`"verification"` | `"credentials"`

The current step of the sign-in flow.