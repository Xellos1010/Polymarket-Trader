# signindescription

```
function SignInDescription(props: SignInDescriptionProps): Element;

```

A description for the SignIn component.

## Parameters

Parameter

Type

Description

`props`

[`SignInDescriptionProps`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Interfaces/SignInDescriptionProps)

The props for the component.

## Returns

`Element` The rendered component.

## Example

```
function App() {
  // Render a custom description in the SignIn component
  return (
    <CDPReactProvider config={config} theme={themeOverrides}>
      <SignIn>
        <SignInBackButton />
        <SignInImage />
        <SignInTitle />
        <SignInDescription>
          Custom Sign In Description
        </SignInDescription>
        <SignInForm />
        <SignInFooter />
      </SignIn>
    </CDPReactProvider>
  );
}

```