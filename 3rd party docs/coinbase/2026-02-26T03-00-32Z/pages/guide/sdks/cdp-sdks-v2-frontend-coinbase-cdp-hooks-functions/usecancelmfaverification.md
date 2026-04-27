# usecancelmfaverification

```
function useCancelMfaVerification(): {
  cancelMfaVerification: () => void;
};

```

A hook for cancelling an in-progress MFA verification flow. Call `cancelMfaVerification` when the user dismisses the MFA UI without completing verification. This will reject the original operation that triggered MFA with a “cancelled” error.

## Returns

```
{
  cancelMfaVerification: () => void;
}

```

An object containing the cancelMfaVerification function.

### cancelMfaVerification()

```
cancelMfaVerification: () => void;

```

#### Returns

`void`

## Example

```
function MfaModal({ onClose }: { onClose: () => void }) {
  const { cancelMfaVerification } = useCancelMfaVerification();
  const handleCancel = () => {
    cancelMfaVerification();
    onClose();
  };
  return (
    <div>
      <h2>Enter your MFA code</h2>
      <button onClick={handleCancel}>Cancel</button>
    </div>
  );
}

```