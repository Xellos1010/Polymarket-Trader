# usesignincontext

```
function useSignInContext(): {
  state: SignInState;
  dispatch: Dispatch<SignInAction>;
};

```

A context for the SignIn component.

## Returns

```
{
  state: SignInState;
  dispatch: Dispatch<SignInAction>;
}

```

The current state of the SignIn component.

### state

### dispatch

```
dispatch: Dispatch<SignInAction>;

```

## Example

```
function EmailComponent() {
  const { state } = useSignInContext();
  return <div>Submitted email: {state.email}</div>;
}
function App() {
  return (
    <CDPReactProvider config={config} theme={themeOverrides}>
      <SignIn>
        <SignInTitle />
        <SignInDescription />
        <EmailComponent />
        <SignInForm />
      </SignIn>
    </CDPReactProvider>
  );

```