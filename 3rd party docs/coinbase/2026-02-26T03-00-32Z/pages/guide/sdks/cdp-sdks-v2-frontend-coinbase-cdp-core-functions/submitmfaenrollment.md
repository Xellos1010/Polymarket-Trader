# submitmfaenrollment

```
function submitMfaEnrollment(options: SubmitMfaEnrollmentOptions): Promise<SubmitMfaEnrollmentResult>;

```

Submits an MFA code to complete the enrollment process. On success, the MFA configuration is persisted to the end user. For TOTP, this validates the code from the user’s authenticator app. For SMS, this validates the code received via text message.

## Parameters

Parameter

Type

Description

`options`

[`SubmitMfaEnrollmentOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/SubmitMfaEnrollmentOptions)

The options for the submission.

## Returns

`Promise`<[`SubmitMfaEnrollmentResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/SubmitMfaEnrollmentResult)\> The result containing the updated user.

## Example

```
// TOTP enrollment
const totpResult = await submitMfaEnrollment({
  mfaMethod: "totp",
  mfaCode: "123456"
});
// SMS enrollment
const smsResult = await submitMfaEnrollment({
  mfaMethod: "sms",
  mfaCode: "654321"
});
console.log("MFA enrolled for user:", smsResult.user.userId);

```