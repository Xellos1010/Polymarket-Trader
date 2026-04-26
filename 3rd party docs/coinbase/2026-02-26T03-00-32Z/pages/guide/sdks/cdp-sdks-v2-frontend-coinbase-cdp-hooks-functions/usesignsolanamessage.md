# usesignsolanamessage

```
function useSignSolanaMessage(): {
  signSolanaMessage: (options: SignSolanaMessageOptions) => Promise<SignSolanaMessageResult>;
};

```

Hook that provides a wrapped function to sign messages with a Solana account with authentication checks. This hook uses useEnforceAuthenticated to ensure the user is signed in before attempting to sign.

## Returns

```
{
  signSolanaMessage: (options: SignSolanaMessageOptions) => Promise<SignSolanaMessageResult>;
}

```

### signSolanaMessage()

```
signSolanaMessage: (options: SignSolanaMessageOptions) => Promise<SignSolanaMessageResult>;

```

#### Parameters

Parameter

Type

`options`

`SignSolanaMessageOptions`

#### Returns

`Promise`<`SignSolanaMessageResult`\>

## Example

```
function SignSolanaMessage() {
  const { signSolanaMessage } = useSignSolanaMessage();
  const { solanaAddress } = useSolanaAddress();
  const handleSign = async () => {
    if (!solanaAddress) return;
    try {
      const result = await signSolanaMessage({
        solanaAccount: solanaAddress,
        message: "Welcome to our dApp! Click to sign in."
      });
      console.log("Signature:", result.signature);
    } catch (error) {
      console.error("Failed to sign message:", error);
    }
  };
  return <button onClick={handleSign}>Sign Message</button>;
}

```