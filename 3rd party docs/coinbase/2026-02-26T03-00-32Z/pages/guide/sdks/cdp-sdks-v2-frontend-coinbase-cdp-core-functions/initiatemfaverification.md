# initiatemfaverification

```
function initiateMfaVerification(options: InitiateMfaVerificationOptions): Promise<void>;

```

Initiates an MFA verification flow for operations requiring MFA. This endpoint should be called when a user attempts a sensitive operation (like transaction signing) but doesn’t have a valid MFA-verified session. For TOTP, this prepares a verification session. For SMS, this sends a 6-digit OTP code to the user’s enrolled phone number.

## Parameters

Parameter

Type

Description

`options`

[`InitiateMfaVerificationOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/InitiateMfaVerificationOptions)

The options for initiating MFA verification.

## Returns

`Promise`<`void`\> A promise that resolves if MFA initiation is successful.

## Example

```
// TOTP verification
await initiateMfaVerification({ mfaMethod: "totp" });
// Prompt user for code from authenticator app
// SMS verification
await initiateMfaVerification({ mfaMethod: "sms" });
// SMS sent to user's phone - prompt for code

```