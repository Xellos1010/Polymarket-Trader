# verifymfainline

```
function VerifyMfaInline(props: VerifyMfaInlineProps): Element;

```

A wrapper component that provides context and coordination for inline MFA verification. Use with VerifyMfaInlineFlow and optionally VerifyMfaInlineBackButton. By default, shows content first and automatically transitions to MFA verification when a protected action (like signing or exporting) is called. After verification, the action completes automatically. Use `verifyFirst` prop to force MFA verification before showing content.

## Parameters

Parameter

Type

Description

`props`

[`VerifyMfaInlineProps`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Interfaces/VerifyMfaInlineProps)

The props for the VerifyMfaInline component.

## Returns

`Element` The rendered component.

## See

-   [VerifyMfaInlineBackButton](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/VerifyMfaInlineBackButton)
-   [VerifyMfaInlineFlow](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/VerifyMfaInlineFlow)
-   [useVerifyMfaInlineContext](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Hooks/useVerifyMfaInlineContext)

## Examples

```
// Simple usage: Just the flow
function ExportWalletModal({ address }) {
  return (
    <Modal>
      <VerifyMfaInline verifyFirst>
        <VerifyMfaInlineFlow>
          <ExportWallet address={address} skipMfa />
        </VerifyMfaInlineFlow>
      </VerifyMfaInline>
    </Modal>
  );
}

```

```
// With back button outside the transition
function CustomModal() {
  return (
    <Modal>
      <VerifyMfaInline>
        <VerifyMfaInlineBackButton />
        <VerifyMfaInlineFlow transition="fade">
          <MyProtectedContent />
        </VerifyMfaInlineFlow>
      </VerifyMfaInline>
    </Modal>
  );
}

```

```
// With render function for full control
function AdvancedModal() {
  return (
    <VerifyMfaInline verifyFirst>
      <VerifyMfaInlineBackButton />
      <VerifyMfaInlineFlow>
        {({ view, Content }) => (
          view === "verify" ? <div>{Content}</div> : <MyProtectedContent />
        )}
      </VerifyMfaInlineFlow>
    </VerifyMfaInline>
  );
}

```

## Further reading

-   [VerifyMfaInline Overview](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/VerifyMfaInline.README)