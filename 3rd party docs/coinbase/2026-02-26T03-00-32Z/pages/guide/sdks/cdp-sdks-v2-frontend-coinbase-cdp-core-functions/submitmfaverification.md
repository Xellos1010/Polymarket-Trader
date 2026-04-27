# submitmfaverification

```
function submitMfaVerification(options: SubmitMfaVerificationOptions): Promise<void>;

```

Submits an MFA code to complete the verification process. After successful verification, any pending operation that was waiting for MFA will automatically continue.

## Parameters

Parameter

Type

Description

`options`

[`SubmitMfaVerificationOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/SubmitMfaVerificationOptions)

The options for the verification.

## Returns

`Promise`<`void`\> A promise that resolves if MFA verification is successful.

## Example

```
// TOTP verification
await submitMfaVerification({
  mfaMethod: "totp",
  mfaCode: "123456"
});
// SMS verification
await submitMfaVerification({
  mfaMethod: "sms",
  mfaCode: "654321"
});

```