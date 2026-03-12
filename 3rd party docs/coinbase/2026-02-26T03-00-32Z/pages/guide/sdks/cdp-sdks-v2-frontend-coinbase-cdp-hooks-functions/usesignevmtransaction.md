# usesignevmtransaction

```
function useSignEvmTransaction(): {
  signEvmTransaction: (options: SignEvmTransactionOptions) => Promise<SignEvmTransactionResult>;
};

```

Hook that provides a wrapped function to sign EVM transactions with authentication checks. This hook uses useEnforceAuthenticated to ensure the user is signed in before attempting to sign.

## Returns

```
{
  signEvmTransaction: (options: SignEvmTransactionOptions) => Promise<SignEvmTransactionResult>;
}

```

### signEvmTransaction()

```
signEvmTransaction: (options: SignEvmTransactionOptions) => Promise<SignEvmTransactionResult>;

```

#### Parameters

#### Returns

`Promise`<[`SignEvmTransactionResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-hooks/Type-Aliases/SignEvmTransactionResult)\>

## Example

```
function SignTransaction() {
  const { signEvmTransaction } = useSignEvmTransaction();
  const { evmAddress } = useEvmAddress();
  const handleSign = async () => {
    if (!evmAddress) return;
    try {
      const result = await signEvmTransaction({
        evmAccount: evmAddress,
        transaction: {
          to: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
          value: 100000000000000n, // 0.0001 ETH in wei
          nonce: 0,
          gas: 21000n,
          maxFeePerGas: 30000000000n,
          maxPriorityFeePerGas: 1000000000n,
          chainId: 84532, // Base Sepolia
          type: "eip1559"
        }
      });
      console.log("Signed Transaction:", result.signedTransaction);
    } catch (error) {
      console.error("Failed to sign transaction:", error);
    }
  };
  return (
    <button onClick={handleSign}>Sign Transaction</button>
  );
}

```