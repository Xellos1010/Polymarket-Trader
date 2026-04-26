# usesignevmmessage

```
function useSignEvmMessage(): {
  signEvmMessage: (options: SignEvmMessageOptions) => Promise<SignEvmMessageResult>;
};

```

Hook that provides a wrapped function to sign EVM messages with authentication checks. This hook uses useEnforceAuthenticated to ensure the user is signed in before attempting to sign.

## Returns

```
{
  signEvmMessage: (options: SignEvmMessageOptions) => Promise<SignEvmMessageResult>;
}

```

### signEvmMessage()

```
signEvmMessage: (options: SignEvmMessageOptions) => Promise<SignEvmMessageResult>;

```

#### Parameters

#### Returns

`Promise`<[`SignEvmMessageResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-hooks/Type-Aliases/SignEvmMessageResult)\>

## Example

```
function SignMessage() {
  const { signEvmMessage } = useSignEvmMessage();
  const { evmAddress } = useEvmAddress();
  const handleSign = async () => {
    if (!evmAddress) return;
    try {
      const result = await signEvmMessage({
        evmAccount: evmAddress,
        message: "Welcome to our dApp! Click to sign in."
      });
      console.log("Signature:", result.signature);
    } catch (error) {
      console.error("Failed to sign message:", error);
    }
  };
  return (
    <button onClick={handleSign}>Sign Message</button>
  );
}

```