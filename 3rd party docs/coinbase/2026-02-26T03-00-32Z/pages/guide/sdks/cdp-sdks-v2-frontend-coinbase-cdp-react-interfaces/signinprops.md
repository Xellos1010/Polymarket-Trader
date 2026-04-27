# signinprops

Props for the SignIn component

## Extends

-   `Omit`<`HTMLAttributes`<`HTMLDivElement`\>, `"children"`\>

## Properties

Property

Type

Description

`children?`

| `ReactNode` | (`state`: [`SignInState`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Interfaces/SignInState)) => `ReactNode`

The children of the component. Leave empty to use the default sign-in flow UI. If a function is provided, it will be called with the current state of the sign-in flow. The function should return a `ReactNode`. **Example** `lines <SignIn> {(state) => { // Render a page title based on the current step return ( <> <SignInBackButton /> <SignInImage /> <h1> {state.step === "credentials" && "Welcome"} {state.step === "verification" && "Almost there"} </h1> <SignInTitle /> <SignInDescription /> <SignInForm /> {state.step === "credentials" && <SignInAuthMethodButtons activeMethod={state.authMethod} />} <SignInFooter /> </> ); }} </SignIn>`

`onSuccess?`

() => `void`

A function to call when the sign-in flow is successful.

`authMethods?`

[`AuthMethod`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/AuthMethod)\[\]

Filter the auth methods that are shown to the user (this still respects the CDP config auth methods).