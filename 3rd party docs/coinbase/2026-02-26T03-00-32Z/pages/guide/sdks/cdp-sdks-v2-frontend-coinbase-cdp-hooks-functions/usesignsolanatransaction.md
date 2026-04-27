# usesignsolanatransaction

```
function useSignSolanaTransaction(): {
  signSolanaTransaction: (options: SignSolanaTransactionOptions) => Promise<SignSolanaTransactionResult>;
};

```

Hook that provides a wrapped function to sign Solana transactions with authentication checks. This hook uses useEnforceAuthenticated to ensure the user is signed in before attempting to sign.

## Returns

```
{
  signSolanaTransaction: (options: SignSolanaTransactionOptions) => Promise<SignSolanaTransactionResult>;
}

```

### signSolanaTransaction()

```
signSolanaTransaction: (options: SignSolanaTransactionOptions) => Promise<SignSolanaTransactionResult>;

```

#### Parameters

Parameter

Type

`options`

`SignSolanaTransactionOptions`

#### Returns

`Promise`<`SignSolanaTransactionResult`\>

## Example

```
function SignSolanaTransaction() {
  const { signSolanaTransaction } = useSignSolanaTransaction();
  const { solanaAddress } = useSolanaAddress();
  const handleSign = async () => {
    if (!solanaAddress) return;
    try {
      const result = await signSolanaTransaction({
        solanaAccount: solanaAddress,
        transaction: "base64-encoded-transaction"
      });
      console.log("Signed Transaction:", result.signedTransaction);
    } catch (error) {
      console.error("Failed to sign transaction:", error);
    }
  };
  return (
    <button onClick={handleSign}>Sign Solana Transaction</button>
  );
}

```