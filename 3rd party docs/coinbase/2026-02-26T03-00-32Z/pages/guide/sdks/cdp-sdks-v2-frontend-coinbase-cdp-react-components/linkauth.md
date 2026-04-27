# linkauth

```
function LinkAuth(props: LinkAuthProps): Element;

```

A component for managing linked authentication methods for the current user. This component displays a list of available authentication methods (email, phone, OAuth providers) and allows users to link or unlink them from their account.

## Parameters

Parameter

Type

Description

`props`

[`LinkAuthProps`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Interfaces/LinkAuthProps)

The props for the component.

## Returns

`Element` The LinkAuth component.

## See

-   [LinkAuthError](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/LinkAuthError)
-   [LinkAuthFlow](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/LinkAuthFlow)
-   [LinkAuthFlowBackButton](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/LinkAuthFlowBackButton)
-   [LinkAuthItem](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/LinkAuthItem)
-   [LinkAuthItems](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/LinkAuthItems)
-   [LinkAuthTitle](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/LinkAuthTitle)
-   [useLinkAuthContext](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Hooks/useLinkAuthContext)
-   [useLinkAuthFlow](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Hooks/useLinkAuthFlow)

## Examples

```
// Render the LinkAuth component with a custom success handler
function App() {
  return (
    <CDPReactProvider config={config}>
      <LinkAuth onLinkSuccess={method => console.log(`Linked ${method}`)} />
    </CDPReactProvider>
  );
}

```

```
// Recreate the default UI
function App() {
  return (
    <CDPReactProvider config={config}>
      <LinkAuth>
        <div className="header">
          <LinkAuthTitle />
          <LinkAuthFlowBackButton />
        </div>
        <div className="error">
          <LinkAuthError />
        </div>
        <LinkAuthFlow />
      </LinkAuth>
    </CDPReactProvider>
  );
}

```

```
// Add a message based on the LinkAuth state
function App() {
  return (
    <CDPReactProvider config={config}>
      <LinkAuth>
        {state => (
          <>
            <div className="header">
              <LinkAuthTitle />
              <LinkAuthFlowBackButton />
            </div>
            {state.methodToLink && (
              <p className="message">
                Linking {state.methodToLink}...
              </p>
            )}
            <div className="error">
              <LinkAuthError />
            </div>
            <LinkAuthFlow />
          </>
        )}
      </LinkAuth>
    </CDPReactProvider>
  );
}

```

```
// Customize LinkAuthItems to show a modal for non-OAuth methods instead of transitioning in place.
function CustomLinkAuthItems() {
  const { link, back } = useLinkAuthFlow();
  const { authMethods } = useAppConfig();
  const [openModal, setOpenModal] = useState<AuthMethod | null>(null);
  const modalMethods = useMemo(
    () => authMethods.filter(method => !method.startsWith("oauth:")),
    [authMethods],
  );
  const handleClose = useCallback(() => {
    setOpenModal(null);
    back();
  }, [back, setOpenModal]);
 const handleLink = useCallback(
   (method: AuthMethod) => {
     link(method);
     if (!method.startsWith("oauth:")) {
       setOpenModal(method);
     }
   },
   [link, setOpenModal],
 );
  return (
    <>
      <LinkAuthItems onLink={handleLink} />
      {modalMethods.map(method => {
        return (
          <SignInModal
            key={method}
            open={openModal === method}
            authMethods={[method]}
            setIsOpen={isOpen => (isOpen ? setOpenModal(method) : handleClose())}
            onSuccess={() => setOpenModal(null)}
          >
            <SignInModalTrigger>null</SignInModalTrigger>
          </SignInModal>
        );
      })}
    </>
  );
};
function App() {
  return (
    <CDPReactProvider config={config}>
      <LinkAuth>
        <h2>Link a profile</h2>
        <CustomLinkAuthItems />
      </LinkAuth>
    </CDPReactProvider>
  );
}

```

## Further reading

-   [LinkAuth Overview](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/LinkAuth.README)