# signintitle

```
function SignInTitle(props: SignInTitleProps): Element;

```

A title for the SignIn component.

## Parameters

Parameter

Type

Description

`props`

[`SignInTitleProps`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Interfaces/SignInTitleProps)

The props for the component.

## Returns

`Element` The rendered component.

## Example

```
function App() {
  // Render a custom title in the SignIn component
  return (
    <CDPReactProvider config={config} theme={themeOverrides}>
      <SignIn>
        <SignInBackButton />
        <SignInImage />
        <SignInTitle>
          Custom Sign In Title
        </SignInTitle>
        <SignInDescription />
        <SignInForm />
        <SignInFooter />
      </SignIn>
    </CDPReactProvider>
  );
}

```