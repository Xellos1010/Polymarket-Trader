# signin

```
function SignIn(props: SignInProps): Element;

```

A sign-in component that handles the email and OTP flow.

## Parameters

Parameter

Type

Description

`props`

[`SignInProps`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Interfaces/SignInProps)

The props for the component.

## Returns

`Element` The SignIn component.

## Examples

```
function App() {
  // Render the SignIn component with a custom onSuccess handler
  const handleSuccess = () => {
    console.log("Sign in successful");
  }
  return (
    <CDPReactProvider config={config} theme={themeOverrides}>
      <SignIn onSuccess={handleSuccess} />
    </CDPReactProvider>
  );
}

```

```
function App() {
  // Render the title, description, and auth method buttons inside the transition containers
  // This is the default UI if no children are provided.
  return (
    <CDPReactProvider config={config} theme={themeOverrides}>
      <SignIn>
        <SignInBackButton />
        <SignInImage />
        <SignInForm>
          {({ authMethod, step, Form }) => {
            // Pass the authMethod and step from the render function args to the title
            // and description components so the UI is rendered correctly when both states
            // are visible during the transition
            return (
              <>
                <SignInTitle step={step} authMethod={authMethod} />
                <SignInDescription step={step} authMethod={authMethod} />
                {Form}
                {state.step === "credentials" && <SignInAuthMethodButtons activeMethod={authMethod} />}
              </>
            );
          }}
        </SignInForm>
        <SignInFooter />
      </SignIn>
    </CDPReactProvider>
  );
}

```

```
function App() {
  // Render a page title based on the current step
  return (
    <CDPReactProvider config={config} theme={themeOverrides}>
      <SignIn>
        {(state) => {
           return (
             <>
               <SignInBackButton />
               <SignInImage />
               <h1>
                 {state.step === "credentials" && "Welcome"}
                 {state.step === "verification" && "Almost there"}
               </h1>
               <SignInTitle />
               <SignInDescription />
               <SignInForm />
               {state.step === "credentials" && <SignInAuthMethodButtons activeMethod={state.authMethod} />}
               <SignInFooter />
             </>
           );
         }}
      </SignIn>
    </CDPReactProvider>
  );
}

```

## Further reading

-   [SignIn Overview](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/SignIn.README)