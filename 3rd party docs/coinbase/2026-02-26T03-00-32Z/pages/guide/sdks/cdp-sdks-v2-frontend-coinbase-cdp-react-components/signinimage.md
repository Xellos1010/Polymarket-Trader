# signinimage

```
function SignInImage(props: SignInImageProps): null | Element;

```

A logo or success icon for the SignIn component.

## Parameters

Parameter

Type

Description

`props`

[`SignInImageProps`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Interfaces/SignInImageProps)

The props for the component.

## Returns

`null` | `Element` The rendered component.

## Example

```
function App() {
  // Use a different image from your app logo in the SignIn component
  return (
    <CDPReactProvider config={config} theme={themeOverrides}>
      <SignIn>
        <SignInBackButton />
        <SignInImage src="https://example.com/image.png" alt="Example Image" />
        <SignInTitle />
        <SignInDescription />
        <SignInForm />
        <SignInFooter />
      </SignIn>
    </CDPReactProvider>
  );
}

```