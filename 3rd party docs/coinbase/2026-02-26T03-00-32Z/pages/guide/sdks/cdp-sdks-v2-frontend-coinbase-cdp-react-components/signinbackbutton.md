# signinbackbutton

```
function SignInBackButton(props: SignInBackButtonProps): null | Element;

```

A button to go back to the previous step of the sign-in flow.

## Parameters

Parameter

Type

Description

`props`

[`SignInBackButtonProps`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Interfaces/SignInBackButtonProps)

The props for the component.

## Returns

`null` | `Element` The sign-in back button.

## Example

```
function App() {
  // Customize the back button icon and label
  return (
    <CDPReactProvider config={config} theme={themeOverrides}>
      <SignIn>
        <SignInBackButton aria-label="go back">
          <MyCustomIcon />
        </SignInBackButton>
        <SignInImage />
        <SignInTitle />
        <SignInDescription />
        <SignInForm />
        <SignInFooter />
      </SignIn>
    </CDPReactProvider>
  );
}

```