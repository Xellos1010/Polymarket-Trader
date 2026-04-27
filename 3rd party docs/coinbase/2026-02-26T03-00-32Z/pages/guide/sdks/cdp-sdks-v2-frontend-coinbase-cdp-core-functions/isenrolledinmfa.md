# isenrolledinmfa

```
function isEnrolledInMfa(user: User, method?: MfaMethod): boolean;

```

Checks if the user is enrolled in MFA.

## Parameters

Parameter

Type

Description

`user`

[`User`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/User)

The user object.

`method?`

[`MfaMethod`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/MfaMethod)

Optional specific method to check. If not provided, checks if user has any MFA method enrolled.

## Returns

`boolean` True if the user is enrolled in the specified method (or any method if not specified), false otherwise.

## Example

```
const user = await getCurrentUser();
// Check if user has any MFA enrolled
if (isEnrolledInMfa(user)) {
  console.log("User has MFA enabled");
}
// Check if user has specific method enrolled
if (isEnrolledInMfa(user, "totp")) {
  console.log("User has TOTP MFA enabled");
}
if (isEnrolledInMfa(user, "sms")) {
  console.log("User has SMS MFA enabled");
}

```