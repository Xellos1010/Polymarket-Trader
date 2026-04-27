# getenrolledmfamethods

```
function getEnrolledMfaMethods(user: User): MfaMethod[];

```

Gets the list of MFA methods that the user has enrolled in.

## Parameters

Parameter

Type

Description

`user`

[`User`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/User)

The user object.

## Returns

[`MfaMethod`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/MfaMethod)\[\] An array of enrolled MFA methods (‘totp’, ‘sms’, or both).

## Example

```
const user = await getCurrentUser();
const methods = getEnrolledMfaMethods(user);
// methods = ['totp', 'sms']
if (methods.length > 1) {
  // Prompt user to choose which method to use
  const selectedMethod = await promptUser(methods);
  await initiateMfaVerification({ mfaMethod: selectedMethod });
}

```