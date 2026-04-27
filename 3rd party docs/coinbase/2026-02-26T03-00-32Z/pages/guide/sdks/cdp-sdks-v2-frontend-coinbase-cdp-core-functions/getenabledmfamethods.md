# getenabledmfamethods

```
function getEnabledMfaMethods(): MfaMethod[];

```

Gets the list of MFA methods enabled in the project configuration.

## 

[​](#returns)

Returns

[`MfaMethod`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/MfaMethod)\[\] An array of enabled MFA methods.

## 

[​](#example)

Example

```
const config = getMfaConfigState();
if (config) {
  const methods = getEnabledMfaMethods(config);
  // methods = ['totp', 'sms']
}

```