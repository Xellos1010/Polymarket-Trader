# initiatemfaenrollment

```
function initiateMfaEnrollment(options: InitiateMfaOptions): Promise<InitiateMfaEnrollmentResult>;

```

Initiates MFA enrollment for an end user. For TOTP, this generates a TOTP secret and returns an otpauth:// URL that can be scanned with an authenticator app like Google Authenticator. The secret must be verified within 5 minutes by calling submitMfaEnrollment. For SMS, this sends a 6-digit OTP code to the provided phone number. The OTP code must be verified within 5 minutes. You can use any phone number for SMS MFA, even if it differs from your SMS authentication phone number.

## Parameters

Parameter

Type

Description

`options`

[`InitiateMfaOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/InitiateMfaOptions)

The options for initiating MFA enrollment.

## Returns

`Promise`<[`InitiateMfaEnrollmentResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/InitiateMfaEnrollmentResult)\> The result containing enrollment details specific to the method.

## Example

```
// TOTP enrollment
const totpResult = await initiateMfaEnrollment({ mfaMethod: "totp" });
// Display totpResult.authUrl as QR code or provide totpResult.secret for manual entry
console.log("Scan this QR code:", totpResult.authUrl);
console.log("Or enter this secret manually:", totpResult.secret);
// SMS enrollment
const smsResult = await initiateMfaEnrollment({
  mfaMethod: "sms",
  phoneNumber: "+14155552671"
});
console.log("SMS sent:", smsResult.success);
// User receives SMS with 6-digit code

```