# signinfooter

```
function SignInFooter(props: HTMLAttributes<HTMLDivElement>): Element;

```

A footer component for the sign-in flow.

## Parameters

Parameter

Type

Description

`props`

`HTMLAttributes`<`HTMLDivElement`\>

The props for the component.

## Returns

`Element` The sign-in footer.

## Example

```
function App() {
  // Add class to the footer
  return (
    <CDPReactProvider config={config} theme={themeOverrides}>
      <SignIn>
        <SignInBackButton />
        <SignInImage />
        <SignInTitle />
        <SignInDescription />
        <SignInForm />
        <SignInFooter className="sign-in-footer" />
      </SignIn>
    </CDPReactProvider>
  );
}

```