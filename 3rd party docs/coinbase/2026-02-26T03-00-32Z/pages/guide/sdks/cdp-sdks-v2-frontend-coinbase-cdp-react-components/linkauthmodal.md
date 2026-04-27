# linkauthmodal

```
function LinkAuthModal(props: LinkAuthModalProps): Element;

```

A link auth modal component that wraps the [LinkAuth](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/LinkAuth) component.

## Parameters

Parameter

Type

Description

`props`

[`LinkAuthModalProps`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Interfaces/LinkAuthModalProps)

The props for the LinkAuthModal component.

## Returns

`Element` The LinkAuthModal component.

## See

-   [LinkAuthModalTrigger](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/LinkAuthModalTrigger) for the trigger button.
-   [LinkAuthModalContent](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/LinkAuthModalContent) for the modal content.

## Examples

```
// Render the LinkAuthModal component with a custom success handler
function App() {
  const handleLinkSuccess = (method: string) => {
    console.log(`Successfully linked ${method}`);
  };
  return (
    <CDPReactProvider config={config}>
      <LinkAuthModal onLinkSuccess={handleLinkSuccess} />
    </CDPReactProvider>
  );
}

```

```
// Render the LinkAuthModal component with a custom trigger button
function App() {
  return (
    <CDPReactProvider config={config}>
      <LinkAuthModal>
        <button type="button">Manage linked accounts</button>
      </LinkAuthModal>
    </CDPReactProvider>
  );
}

```

```
// Render the LinkAuthModal component with a custom trigger button label
function App() {
  return (
    <CDPReactProvider config={config}>
      <LinkAuthModal>
        <LinkAuthModalTrigger variant="secondary" label="Link more accounts" />
      </LinkAuthModal>
    </CDPReactProvider>
  );
}

```

```
// Render the LinkAuthModal with controlled open state
function App() {
  const [isOpen, setIsOpen] = useState(false);
  return (
    <CDPReactProvider config={config}>
      <LinkAuthModal open={isOpen} setIsOpen={setIsOpen}>
        <LinkAuthModalTrigger />
        <LinkAuthModalContent />
      </LinkAuthModal>
    </CDPReactProvider>
  );
}

```