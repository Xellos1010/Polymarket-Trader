# verifymfaprops

Props for the VerifyMfa component.

## Extends

-   `Omit`<`HTMLAttributes`<`HTMLDivElement`\>, `"children"` | `"onSubmit"` | `"onError"`\>

## Properties

Property

Type

Description

`children?`

| `ReactNode` | (`state`: [`VerifyMfaState`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Interfaces/VerifyMfaState)) => `ReactNode`

The children of the component. Leave empty to use the default verification UI. If a function is provided, it will be called with the current state of the verification flow. The function should return a `ReactNode`. **Example** `<VerifyMfa> {(state) => ( <> <VerifyMfaTitle /> <VerifyMfaDescription /> <VerifyMfaFlow /> </> )} </VerifyMfa>`

`onSuccess?`

(`mfaCode`: `string`) => `void`

A function to call when the verification is successful.

`onError?`

(`error`: | `Error` | [`APIError`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Classes/APIError)) => `void`

A function to call when the verification errors.