# usesendevmtransaction

```
function useSendEvmTransaction(): {
  sendEvmTransaction: (options: SendEvmTransactionOptions) => Promise<SendEvmTransactionResult>;
  data: TransactionState;
};

```

Hook that provides a wrapped function to send EVM transactions with authentication checks. This hook uses useEnforceAuthenticated to ensure the user is signed in before attempting to send. The hook internally waits for transactions to succeed and returns the related success/error via the `data` discriminated union object. Note: The `data` returned from the hook only represents the *last* sent transaction. If you wish to call one instance of the hook multiple times in quick succession it is recommended to save the txHash returned from `sendEvmTransaction` yourself and handle waiting for the receipt.

## Returns

```
{
  sendEvmTransaction: (options: SendEvmTransactionOptions) => Promise<SendEvmTransactionResult>;
  data: TransactionState;
}

```

### sendEvmTransaction()

```
sendEvmTransaction: (options: SendEvmTransactionOptions) => Promise<SendEvmTransactionResult>;

```

#### Parameters

#### Returns

`Promise`<[`SendEvmTransactionResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-hooks/Type-Aliases/SendEvmTransactionResult)\>

### data

## Example

```
function SendTransaction() {
  const { sendEvmTransaction, data } = useSendEvmTransaction();
  const { evmAddress } = useEvmAddress();
  const handleSend = async () => {
    if (!evmAddress) return;
    try {
      const result = await sendEvmTransaction({
        evmAccount: evmAddress,
        network: "base-sepolia",
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
      console.log("Transaction Hash:", result.transactionHash);
    } catch (error) {
      console.error("Failed to send transaction:", error);
    }
  };
  return (
    <div>
      {(() => {
        switch (data.status) {
          case "idle":
            return <p>No transaction in progress</p>;
          case "pending":
            return (
              <div>
                <p>Transaction pending...</p>
                <p>Hash: {data.hash}</p>
              </div>
            );
          case "success":
            return (
              <div>
                <p>Transaction successful!</p>
                <p>Block: {data.receipt.blockNumber.toString()}</p>
                <p>Gas used: {data.receipt.gasUsed.toString()}</p>
              </div>
            );
          case "error":
            return (
              <div>
                <p>Transaction failed</p>
                <p>Error: {data.error.message}</p>
              </div>
            );
        }
      })()}
    </div>
    <button onClick={handleSend}>Send Transaction</button>
  );
}

```