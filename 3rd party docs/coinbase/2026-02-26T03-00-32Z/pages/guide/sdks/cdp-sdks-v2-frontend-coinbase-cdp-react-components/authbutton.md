# authbutton

```
function AuthButton(props: AuthButtonProps & Omit<HTMLAttributes<HTMLDivElement>, "children">): Element;

```

A button that signs the user in or out. This component will render the [SignInModal](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/SignInModal) component when the user is signed out, and a [SignOutButton](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/SignOutButton) when the user is signed in. If the SDK is initializing (i.e. the initial user state is pending), the component will render a loading skeleton.

## Parameters

Parameter

Type

Description

`props`

[`AuthButtonProps`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Interfaces/AuthButtonProps) & `Omit`<`HTMLAttributes`<`HTMLDivElement`\>, `"children"`\>

The props for the component.

## Returns

`Element` The rendered component.

## Examples

```
// Render the AuthButton component
function App() {
  return (
    <CDPReactProvider config={config} theme={themeOverrides}>
      <AuthButton />
    </CDPReactProvider>
  );
}

```

```
// Render the AuthButton component with custom components
// Define a custom placeholder
const SmallPlaceholder: AuthButtonProps["placeholder"] = props => (
  <LoadingSkeleton {...props} className={`${props.className} small-placeholder`} />
);
// Define a custom sign in modal using the secondary variant and small size for the trigger button
const SmallSecondarySignInModal: AuthButtonProps["signInModal"] = props => (
  <SignInModal {...props}>
    <SignInModalTrigger variant="secondary" size="sm" label="Log me in" />
  </SignInModal>
);
// Define a custom sign out button using the secondary variant and small size
const SmallSecondarySignOutButton: AuthButtonProps["signOutButton"] = props => (
  <SignOutButton {...props} variant="secondary" size="sm">Log me out</SignOutButton>
);
function App() {
  return (
    <CDPReactProvider config={config} theme={themeOverrides}>
      <AuthButton
        placeholder={SmallPlaceholder}
        signInModal={SmallSecondarySignInModal}
        signOutButton={SmallSecondarySignOutButton}
     />
    </CDPReactProvider>
  );
}

```

```
// Render the AuthButton component with custom containers around the slot content
function App() {
  return (
    <CDPReactProvider config={config} theme={themeOverrides}>
      <AuthButton className="auth-wrapper">
        {({ isInitialized, isSignedIn, Placeholder, SignOutButton, SignInModal }) => (
          <>
            {!isInitialized && Placeholder}
            {isInitialized && (
              <div>
                <p>Hello {isSignedIn ? "signed in" : "signed out"} user!</p>
                {isSignedIn && <div>{SignOutButton}</div>}
                {!isSignedIn && <div>{SignInModal}</div>}
              </div>
            )}
          </>
        )}
      </AuthButton>
    </CDPReactProvider>
  );
}

```