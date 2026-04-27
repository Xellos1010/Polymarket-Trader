# verifymfamodal

```
function VerifyMfaModal(props: VerifyMfaModalProps): Element;

```

A verify MFA modal component that wraps the [VerifyMfa](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/VerifyMfa) component.

## Parameters

Parameter

Type

Description

`props`

[`VerifyMfaModalProps`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Interfaces/VerifyMfaModalProps)

The props for the VerifyMfaModal component.

## Returns

`Element` The VerifyMfaModal component.

## See

-   [VerifyMfaModalTrigger](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/VerifyMfaModalTrigger) for the trigger button.
-   [VerifyMfaModalContent](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/VerifyMfaModalContent) for the modal content.

## Examples

```
// Render the VerifyMfaModal component with a custom success handler
function App() {
  const handleSuccess = (mfaCode: string) => {
    console.log(`Successfully verified`, mfaCode);
  };
  return (
    <CDPReactProvider config={config}>
      <VerifyMfaModal onSuccess={handleSuccess} />
    </CDPReactProvider>
  );
}

```

```
// Render the VerifyMfaModal component with a custom trigger button
function App() {
  return (
    <CDPReactProvider config={config}>
      <VerifyMfaModal>
        <button type="button">Verify</button>
      </VerifyMfaModal>
    </CDPReactProvider>
  );
}

```

```
// Render the VerifyMfaModal component with a custom trigger button label
function App() {
  return (
    <CDPReactProvider config={config}>
      <VerifyMfaModal>
        <VerifyMfaModalTrigger variant="secondary" label="Verify" />
      </VerifyMfaModal>
    </CDPReactProvider>
  );
}

```

```
// Render the VerifyMfaModal with controlled open state
function App() {
  const [isOpen, setIsOpen] = useState(false);
  return (
    <CDPReactProvider config={config}>
      <VerifyMfaModal open={isOpen} setIsOpen={setIsOpen}>
        <VerifyMfaModalTrigger />
        <VerifyMfaModalContent />
      </VerifyMfaModal>
    </CDPReactProvider>
  );
}

```