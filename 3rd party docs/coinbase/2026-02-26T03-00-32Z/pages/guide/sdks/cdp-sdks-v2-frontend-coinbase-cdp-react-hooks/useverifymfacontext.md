# useverifymfacontext

Hooks

```
function useVerifyMfaContext(): VerifyMfaContextValue;

```

Hook to access the VerifyMfa context.

## 

[​](#returns)

Returns

[`VerifyMfaContextValue`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Interfaces/VerifyMfaContextValue) The current state and dispatch function of the VerifyMfa component.

## 

[​](#see)

See

[VerifyMfa](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/VerifyMfa)

## 

[​](#example)

Example

```
function MfaCodeDisplay() {
  const { state } = useVerifyMfaContext();
  return <div>Code: {state.mfaCode}</div>;
}

```

Was this page helpful?

[

useTheme

Previous

](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Hooks/useTheme)[

useVerifyMfaFlow

Next

](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Hooks/useVerifyMfaFlow)

⌘I