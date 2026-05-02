# appconfig

Optional app config to add branding

## Example

```
const appConfig: AppConfig = {
  appName: "My App",
  appLogoUrl: "https://placehold.co/64",
};

```

## Properties

Property

Type

Description

`appName?`

`string`

The name of the app.

`appLogoUrl?`

`string`

The URL of the app logo. This should be at least 64 by 64px and must start with `http` or `https`.

`showCoinbaseFooter?`

`boolean`

Whether to show the “secured by Coinbase” footer. Defaults to `true`.

`authMethods?`

\[[`AuthMethod`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/AuthMethod), `...AuthMethod[]`\]

Authentication methods to allow for the user. Defaults to `["email"]`.

`mfa?`

{ `disableAutoPrompt?`: `boolean`; }

MFA verification configuration.

`mfa.disableAutoPrompt?`

`boolean`

Disable automatic prompting of the MFA verification modal. Set to `true` when using `useRegisterMfaListener` for custom MFA UI, or when you want to trigger the modal manually via `useVerifyMfaModal`. **Default** `false`